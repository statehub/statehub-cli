//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use super::*;

impl StateHub {
    // async fn get_states_helper(&self, names: &[v1::StateName]) -> anyhow::Result<Vec<v1::State>> {
    //     let mut states = vec![];
    //     for name in names {
    //         let state = self.api.get_state(name).await?.into_inner()?;
    //         states.push(state);
    //     }
    //     Ok(states)
    // }

    pub(super) async fn add_location_helper(
        &self,
        state: &v1::State,
        location: &Location,
    ) -> anyhow::Result<()> {
        log::info!("Extending {} to {}", state, location);

        let name = state.name.clone();

        match location {
            Location::Aws(region) => self
                .api
                .add_aws_location(name, *region)
                .await
                .map(|_aws| ()),
            Location::Azure(region) => self
                .api
                .add_azure_location(name, *region)
                .await
                .map(|_azure| ()),
        }
    }

    pub(super) async fn remove_location_helper(
        &self,
        state: &v1::State,
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
        names: &[v1::StateName],
        locations: &[Location],
    ) -> anyhow::Result<()> {
        for name in names {
            self.adjust_state_locations(name, locations).await?;
        }

        Ok(())
    }

    async fn adjust_state_locations(
        &self,
        name: &v1::StateName,
        locations: &[Location],
    ) -> anyhow::Result<()> {
        let state = self.api.get_state(name).await?;

        log::info!("Checking {}", state.show());
        for location in locations {
            if !state.is_available_in(location) {
                self.inform(format!("Extdending state {} to {}", state.name, location))?;
                self.add_location_helper(&state, location).await?;
            } else {
                log::info!(
                    "Skipping state {} which is already available in {}",
                    state.name,
                    location
                );
            }
        }

        Ok(())
    }

    pub(super) async fn setup_configmap_helper(
        &self,
        cluster: &v1::Cluster,
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
        cluster: &v1::Cluster,
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
        cluster: &v1::Cluster,
        states: Option<Vec<v1::StateName>>,
    ) -> anyhow::Result<()> {
        if let Some(states) = states {
            for state in states {
                if self.api.get_state(&state).await?.owner.is_none() {
                    self.verbosely(format!("Claiming ownership of state {}", state))?;
                    self.api.set_owner(state, &cluster.name).await?;
                }
            }
        }
        Ok(())
    }

    pub(super) async fn relinquish_states_helper(
        &self,
        cluster: &v1::ClusterName,
    ) -> anyhow::Result<()> {
        for state in self.api.get_all_states().await? {
            if state.owner.as_ref() == Some(cluster) {
                log::info!("Relinquish ownership for state {}", state.name);
                self.api.unset_owner(state.name).await?;
            } else if let Some(owner) = state.owner {
                log::debug!("Skipping state {} (owned by {})", state.name, owner);
            } else {
                log::debug!("Skipping state {} (unowned)", state.name);
            }
        }
        Ok(())
    }
}
