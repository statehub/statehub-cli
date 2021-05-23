//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::process::Command;
use tokio::process::Command as AsyncCmd;

use super::*;

impl StateHub {
    pub(super) fn helm(&self, helm: &v1::Helm) -> Command {
        let mut cmd = Command::new("helm");
        cmd.arg("insttall")
            .arg("--namespace")
            .arg("statehub")
            .arg("--repo")
            .arg(&helm.repo)
            .arg(&helm.chart);
        cmd
    }

    pub(super) async fn install_statehub_helper(&self, helm: Command) -> anyhow::Result<()> {
        let output = AsyncCmd::from(helm).output().await?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        self.verbosely(stdout);

        Ok(())
    }

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
        name: &v1::StateName,
        location: &Location,
    ) -> anyhow::Result<()> {
        match location {
            Location::Aws(region) => self
                .api
                .add_aws_location(name.clone(), *region)
                .await
                .map(|_aws| ()),
            Location::Azure(region) => self
                .api
                .add_azure_location(name.clone(), *region)
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
        let state = self.api.get_state(name).await?.into_inner()?;

        for location in locations {
            if !state.is_available_in(location) {
                // need to extend the state to this location as well
                self.verbosely(format!(
                    "Adding {:#} location to state '{}'",
                    location, state.name
                ));
                self.add_location_helper(&state.name, location).await?;
            }
        }

        Ok(())
    }
}
