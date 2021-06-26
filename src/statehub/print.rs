//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use serde_json as json;

use super::*;

pub(super) trait Print {
    fn print(self, stdout: &Term, json: bool) -> anyhow::Result<()>;
}

impl<T> Print for anyhow::Result<T>
where
    T: Print,
{
    fn print(self, stdout: &Term, json: bool) -> anyhow::Result<()> {
        self?.print(stdout, json)
    }
}

impl<T> Print for Output<T>
where
    T: de::DeserializeOwned + Serialize + Show,
{
    fn print(self, stdout: &Term, json: bool) -> anyhow::Result<()> {
        let text = self.into_text(json);
        stdout.write_line(&text)?;
        Ok(())
    }
}

impl<T> Print for Detailed<Output<T>>
where
    T: de::DeserializeOwned + Serialize + Show,
{
    fn print(self, stdout: &Term, json: bool) -> anyhow::Result<()> {
        let text = self.into_text(json);
        stdout.write_line(&text)?;
        Ok(())
    }
}

impl<T> Print for Quiet<Output<T>>
where
    T: de::DeserializeOwned + Serialize + Show,
{
    fn print(self, stdout: &Term, json: bool) -> anyhow::Result<()> {
        let text = self.into_text(json);
        stdout.write_line(&text)?;
        Ok(())
    }
}

impl Print for StateAndClusters {
    fn print(self, stdout: &Term, json: bool) -> anyhow::Result<()> {
        let text = if json {
            json::to_string(&self)?
        } else {
            self.detailed_show()
        };
        stdout.write_line(&text)?;
        Ok(())
    }
}

impl Print for ClusterAndStates {
    fn print(self, stdout: &Term, json: bool) -> anyhow::Result<()> {
        let text = if json {
            json::to_string(&self)?
        } else {
            let cluster = self.cluster.detailed_show();
            let states = self.states.iter().map(ToString::to_string).join(" ");
            format!("{}\nVisible states:\n    {}", cluster, states)
        };
        stdout.write_line(&text)?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct ClusterAndStates {
    #[serde(flatten)]
    cluster: v0::Cluster,
    states: Vec<v0::StateName>,
}

impl ClusterAndStates {
    pub(super) fn new(
        cluster: Detailed<Output<v0::Cluster>>,
        states: Output<Vec<v0::State>>,
    ) -> Self {
        let cluster = cluster.0.into_inner();
        let cluster_locations = cluster.all_locations();
        let states = states
            .into_iter()
            .filter(|state| {
                state
                    .all_locations()
                    .iter()
                    .any(|location| cluster_locations.contains(location))
            })
            .map(|state| state.name)
            .collect();

        Self { cluster, states }
    }
}

impl Show for ClusterAndStates {
    fn show(&self) -> String {
        self.detailed_show()
    }

    fn detailed_show(&self) -> String {
        let cluster = self.cluster.detailed_show();
        let states = self.states.iter().map(ToString::to_string).join(" ");
        format!("{}\nVisible states:\n    {}", cluster, states)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct StateAndClusters {
    #[serde(flatten)]
    state: v0::State,
    clusters: Vec<v0::ClusterName>,
}

impl StateAndClusters {
    pub(super) fn new(
        state: Detailed<Output<v0::State>>,
        clusters: Output<Vec<v0::Cluster>>,
    ) -> Self {
        let state = state.0.into_inner();
        let state_locations = state.all_locations();
        let clusters = clusters
            .into_iter()
            .filter(|cluster| {
                cluster
                    .all_locations()
                    .iter()
                    .any(|location| state_locations.contains(location))
            })
            .map(|cluster| cluster.name)
            .collect();

        Self { state, clusters }
    }
}

impl Show for StateAndClusters {
    fn show(&self) -> String {
        self.detailed_show()
    }

    fn detailed_show(&self) -> String {
        let state = self.state.detailed_show();
        let clusters = self.clusters.iter().map(ToString::to_string).join(" ");
        format!("{}\nVisible clusters:\n    {}", state, clusters)
    }
}
