//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::time::Duration;

use indexmap::IndexMap;
use serde_json as json;
use tokio::time;

use super::*;

impl StateHub {
    pub(super) async fn add_location_helper(
        &self,
        state: &v0::State,
        location: &Location,
        wait: bool,
    ) -> anyhow::Result<()> {
        log::info!("Extending {} to {}", state, location);

        match location {
            Location::Aws(region) => {
                self.add_aws_location_helper(&state.name, *region, wait)
                    .await?;
            }
            Location::Azure(region) => {
                self.add_azure_location_helper(&state.name, *region, wait)
                    .await?;
            }
        }

        Ok(())
    }

    async fn add_aws_location_helper(
        &self,
        name: &v0::StateName,
        region: v0::AwsRegion,
        wait: bool,
    ) -> anyhow::Result<Output<v0::StateLocationAws>> {
        let aws = self.api.add_aws_location(name, region).await?;
        if wait {
            let delay = Duration::from_secs(5);
            loop {
                if self
                    .api
                    .get_aws_location(name, region)
                    .await?
                    .status
                    .is_final()
                {
                    break;
                }
                time::sleep(delay).await;
            }
        }
        Ok(aws)
    }

    async fn add_azure_location_helper(
        &self,
        name: &v0::StateName,
        region: v0::AzureRegion,
        wait: bool,
    ) -> anyhow::Result<Output<v0::StateLocationAzure>> {
        let azure = self.api.add_azure_location(name, region).await?;
        if wait {
            let delay = Duration::from_secs(5);
            loop {
                if self
                    .api
                    .get_azure_location(name, region)
                    .await?
                    .status
                    .is_final()
                {
                    break;
                }
                time::sleep(delay).await;
            }
        }
        Ok(azure)
    }

    pub(super) async fn remove_location_helper(
        &self,
        state: &v0::State,
        location: &Location,
    ) -> anyhow::Result<()> {
        log::info!("Truncating {} from {}", state, location);

        let name = state.name.clone();

        match location {
            Location::Aws(region) => self
                .api
                .del_aws_location(name, *region)
                .await
                .map(|_aws| ()),
            Location::Azure(region) => self
                .api
                .del_azure_location(name, *region)
                .await
                .map(|_azure| ()),
        }
    }

    pub(super) async fn adjust_all_states(
        &self,
        states: &[v0::StateName],
        locations: &[Location],
        wait: bool,
    ) -> anyhow::Result<()> {
        let missing_locations = self.get_missing_locations(states, locations).await?;
        let multiple_missing_locations = missing_locations
            .iter()
            .filter(|(_, locations)| locations.len() > 1)
            .map(|(state, locations)| (state.clone(), locations.clone()))
            .collect::<IndexMap<_, _>>();

        if !wait && !multiple_missing_locations.is_empty() {
            let add_locations = generate_add_location_commands(&missing_locations).join("\n");
            let text = format!(
                r#"Some states will need multiple locations to be added
and it is not possible at the moment without '--wait' flag.

You can either re-run this command with '--wait' flag
or run following commands before running 'statehub register-cluster`

{}
"#,
                add_locations
            );
            anyhow::bail!(text)
        }

        for (state, locations) in &missing_locations {
            self.add_missing_locations(state, locations, wait).await?;
        }

        Ok(())
    }

    async fn get_missing_locations(
        &self,
        states: &[v0::StateName],
        locations: &[Location],
    ) -> anyhow::Result<IndexMap<v0::State, Vec<Location>>> {
        let mut missing_locations = IndexMap::new();
        for state in states {
            let state = self.api.get_state(state).await?;
            let missing = locations
                .iter()
                .filter(|location| !state.is_available_in(location))
                .copied()
                .collect();
            let state = state.into_inner();
            missing_locations.insert(state, missing);
        }
        Ok(missing_locations)
    }

    async fn add_missing_locations(
        &self,
        state: &v0::State,
        locations: &[Location],
        wait: bool,
    ) -> anyhow::Result<()> {
        for location in locations {
            if state.is_available_in(location) {
                log::info!(
                    "Skipping state {} which is already available in {}",
                    state.name,
                    location
                );
            } else {
                self.inform(format_args!(
                    "Extdending state {} to {}",
                    state.name, location
                ))?;
                self.add_location_helper(state, location, wait).await?;
            }
        }

        Ok(())
    }

    async fn _adjust_state_locations(
        &self,
        name: &v0::StateName,
        locations: &[Location],
    ) -> anyhow::Result<()> {
        let state = self.api.get_state(name).await?;
        let wait = false;

        log::info!("Checking {}", state.show());
        for location in locations {
            if state.is_available_in(location) {
                log::info!(
                    "Skipping state {} which is already available in {}",
                    state.name,
                    location
                );
            } else {
                self.inform(format_args!(
                    "Extdending state {} to {}",
                    state.name, location
                ))?;
                self.add_location_helper(&state, location, wait).await?;
            }
        }

        Ok(())
    }

    pub(super) async fn setup_configmap_helper(
        &self,
        cluster: &v0::Cluster,
        helm: &k8s::Helm,
    ) -> anyhow::Result<()> {
        let namespace = helm.namespace();
        let default_state = helm.default_state().unwrap_or("");
        let api = self.api.url("");
        k8s::store_configmap(namespace, &cluster.name, default_state, &api).await?;
        Ok(())
    }

    pub(super) async fn setup_cluster_token_helper(
        &self,
        cluster: &v0::Cluster,
        helm: &k8s::Helm,
    ) -> anyhow::Result<()> {
        let token = self.api.issue_cluster_token(&cluster.name).await?;
        self.verbosely(format!("Issued token {} for {}", token.token, cluster))?;
        let namespace = helm.namespace();
        k8s::store_cluster_token(namespace, &token.token).await?;
        Ok(())
    }

    pub(super) async fn claim_unowned_states_helper(
        &self,
        cluster: &v0::Cluster,
        states: Option<Vec<v0::StateName>>,
    ) -> anyhow::Result<()> {
        if let Some(states) = states {
            for state in states {
                if self.api.get_state(&state).await?.owner.is_none() {
                    self.verbosely(format!("Claiming ownership of state {}", state))?;
                    self.api.set_owner(&state, &cluster.name).await?;
                }
            }
        }
        Ok(())
    }

    pub(super) async fn relinquish_states_helper(
        &self,
        cluster: &v0::ClusterName,
    ) -> anyhow::Result<()> {
        for state in self.api.get_all_states().await? {
            if state.owner.as_ref() == Some(cluster) {
                log::info!("Relinquish ownership for state {}", state.name);
                self.api.unset_owner(&state.name).await?;
            } else if let Some(owner) = state.owner {
                log::debug!("Skipping state {} (owned by {})", state.name, owner);
            } else {
                log::debug!("Skipping state {} (unowned)", state.name);
            }
        }
        Ok(())
    }

    pub(super) async fn delete_volume_helper(
        &self,
        state: &v0::StateName,
        volume: &v0::VolumeName,
        wait: bool,
    ) -> anyhow::Result<Output<v0::Volume>> {
        let mut volume = self.api.delete_volume(state, volume).await?;
        if wait {
            let delay = Duration::from_secs(5);
            loop {
                match self.api.get_volume(state, &volume.name).await {
                    Ok(deleting) => volume = deleting,
                    Err(err) if is_volume_not_found(&err) => break,
                    _ => continue,
                }
            }
            time::sleep(delay).await;
        }

        Ok(volume)
    }

    pub(super) fn login_prompt_helper(&self) -> anyhow::Result<(String, String)> {
        let username = whoami::username();
        let hostname = whoami::hostname();
        let login = v0::Login { username, hostname };
        let text = json::to_string(&login)?;
        let token = base64::encode(&text);
        let id = format!("{}@{}", login.username, login.hostname);
        Ok((token, id))
    }

    pub(super) async fn helm_install_helper(
        &self,
        helm: &k8s::Helm,
        cluster: &v0::Cluster,
    ) -> anyhow::Result<()> {
        let (stdout, stderr) = helm.execute(cluster).await?;

        self.verbosely(stdout)?;
        self.error(stderr)?;
        Ok(())
    }
}

fn is_volume_not_found(err: &anyhow::Error) -> bool {
    err.downcast_ref::<v0::Error>()
        .map(v0::Error::is_volume_not_found)
        .unwrap_or_default()
}

#[derive(Clone, Debug)]
pub(super) enum AddLocation {
    FromLocation(Location),
    FromCluster(v0::ClusterName),
}

fn generate_add_location_commands(
    missing_locations: &IndexMap<v0::State, Vec<Location>>,
) -> Vec<String> {
    missing_locations
        .iter()
        .flat_map(|(state, locations)| {
            locations.iter().map(move |location| {
                format!("statehub add-location --wait {} {}", state.name, location)
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_add_location_commands() {
        let missing_locations = vec![
            (
                v0::State::new("alfa"),
                vec![Location::Aws(v0::AwsRegion::UsWest2)],
            ),
            (
                v0::State::new("bravo"),
                vec![
                    Location::Aws(v0::AwsRegion::UsEast1),
                    Location::Azure(v0::AzureRegion::EastUs2),
                ],
            ),
            (
                v0::State::new("charlie"),
                vec![Location::Aws(v0::AwsRegion::UsWest2)],
            ),
        ]
        .into_iter()
        .collect();
        let cmds = super::generate_add_location_commands(&missing_locations);
        assert_eq!(cmds[0], "statehub add-location --wait alfa us-west-2");
        assert_eq!(cmds[1], "statehub add-location --wait bravo us-east-1");
        assert_eq!(cmds[2], "statehub add-location --wait bravo eastus2");
        assert_eq!(cmds[3], "statehub add-location --wait charlie us-west-2");
    }
}
