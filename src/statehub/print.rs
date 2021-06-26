//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

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
    T: DeserializeOwned + Serialize + Show,
{
    fn print(self, stdout: &Term, json: bool) -> anyhow::Result<()> {
        let text = self.into_text(json);
        stdout.write_line(&text)?;
        Ok(())
    }
}

impl<T> Print for Detailed<Output<T>>
where
    T: DeserializeOwned + Serialize + Show,
{
    fn print(self, stdout: &Term, json: bool) -> anyhow::Result<()> {
        let text = self.into_text(json);
        stdout.write_line(&text)?;
        Ok(())
    }
}

impl<T> Print for Quiet<Output<T>>
where
    T: DeserializeOwned + Serialize + Show,
{
    fn print(self, stdout: &Term, json: bool) -> anyhow::Result<()> {
        let text = self.into_text(json);
        stdout.write_line(&text)?;
        Ok(())
    }
}

impl Print for (Detailed<Output<v0::State>>, Output<Vec<v0::Cluster>>) {
    fn print(self, stdout: &Term, json: bool) -> anyhow::Result<()> {
        let (state, clusters) = self;
        let state_locations = state.all_locations();

        state.print(stdout, json)?;

        let visible_clusters = clusters
            .iter()
            .filter(|cluster| {
                cluster
                    .all_locations()
                    .iter()
                    .any(|location| state_locations.contains(location))
            })
            .map(ToString::to_string)
            .join(" ");

        stdout.write_line(&format!("Visible clusters:\n    {}", visible_clusters))?;
        Ok(())
    }
}

impl Print for (Detailed<Output<v0::Cluster>>, Output<Vec<v0::State>>) {
    fn print(self, stdout: &Term, json: bool) -> anyhow::Result<()> {
        let (cluster, states) = self;
        let cluster_locations = cluster.all_locations();

        cluster.print(stdout, json)?;

        let visible_states = states
            .iter()
            .filter(|state| {
                state
                    .all_locations()
                    .iter()
                    .any(|location| cluster_locations.contains(location))
            })
            .map(ToString::to_string)
            .join(" ");

        stdout.write_line(&format!("Visible states:\n    {}", visible_states))?;
        Ok(())
    }
}
