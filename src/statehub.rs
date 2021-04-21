//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use serde::{de::DeserializeOwned, Serialize};
use structopt::StructOpt;

use crate::api;
use crate::kubectl::{self, Kubectl};
use crate::show::Show;
use crate::v1;
use crate::Location;
use crate::Output;

const ABOUT: &str = "statehub CLI tool";

#[derive(Debug, StructOpt)]
#[structopt(about = ABOUT)]
pub(crate) struct Cli {
    #[structopt(
        help = "Management server URL or address",
        default_value = "https://api.statehub.io",
        short,
        long,
        env = "SHAPI"
    )]
    management: String,
    #[structopt(help = "Authentication token", short, long, env = "SHTOKEN")]
    token: Option<String>,
    #[structopt(
        long,
        global = true,
        conflicts_with = "json",
        help = "Show raw JSON output (different from '--json')"
    )]
    raw: bool,
    #[structopt(
        long,
        global = true,
        conflicts_with = "raw",
        help = "Show results as JSON (different from '--raw')"
    )]
    json: bool,
    #[structopt(short, long, global = true)]
    verbose: bool,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(about = "Create new state", aliases = &["create-st", "cs"])]
    CreateState {
        #[structopt(help = "State name")]
        name: v1::StateName,
        #[structopt(long, short, help = "Defalt owning cluster")]
        owner: Option<v1::ClusterName>,
        #[structopt(long, short, help = "Location definition")]
        location: Vec<Location>,
    },
    #[structopt(about = "Delete existing state", aliases = &["delete-st", "ds"])]
    DeleteState {
        #[structopt(help = "State name")]
        name: v1::StateName,
    },
    #[structopt(about = "List available states", aliases = &["list-state", "list-st", "ls"])]
    ListStates,
    #[structopt(about = "Show state details", aliases = &["show-s", "ss"])]
    ShowState {
        #[structopt(help = "State name")]
        name: v1::StateName,
    },
    #[structopt(about = "Register new cluster", aliases = &["list-cluster", "list-cl", "lc"])]
    ListClusters,
    #[structopt(about = "Register new cluster", aliases = &["register-cl", "rc"])]
    RegisterCluster {
        #[structopt(help = "Cluster name")]
        name: v1::ClusterName,
    },
    #[structopt(about = "Unregister existing cluster", aliases = &["unregister-cl", "uc"])]
    UnregisterCluster {
        #[structopt(help = "Cluster name")]
        name: v1::ClusterName,
    },
    #[structopt(about = "Manually create new volume")]
    CreateVolume,
    #[structopt(about = "Manually delete existing volume")]
    DeleteVolume,
    #[structopt(about = "Manually make state available in a specified location", aliases = &["add-l", "al"])]
    AddLocation {
        #[structopt(help = "State name")]
        state: v1::StateName,
        #[structopt(help = "Location definition")]
        location: Location,
    },
    #[structopt(about = "Manually make state unavailable in a specified location")]
    RemoveLocation,
    #[structopt(about = "Set state availability grade")]
    SetAvailability,
    #[structopt(about = "Set cluster as the state owner")]
    SetOwner {
        #[structopt(help = "State name")]
        state: v1::StateName,
        #[structopt(help = "Cluster name")]
        cluster: v1::ClusterName,
    },
    #[structopt(about = "Clear state owner")]
    UnsetOwner {
        #[structopt(help = "State name")]
        state: v1::StateName,
        #[structopt(help = "Cluster name")]
        cluster: v1::ClusterName,
    },
    #[structopt(about = "List K8s nodes")]
    ListNodes,
    #[structopt(about = "List K8s pods")]
    ListPods,
    #[structopt(about = "List K8s node regions")]
    ListRegions {
        #[structopt(help = "Also list zones", long, short)]
        zone: bool,
    },
}

impl Cli {
    pub(crate) async fn execute() -> anyhow::Result<()> {
        Self::from_args().dispatch().await
    }

    async fn dispatch(self) -> anyhow::Result<()> {
        let statehub = StateHub::new(
            self.management,
            self.token,
            self.json,
            self.raw,
            self.verbose,
        );

        match self.command {
            Command::CreateState {
                name,
                owner,
                location,
            } => {
                let locations = location.into();
                statehub.create_state(name, owner, locations).await
            }
            Command::DeleteState { name: state } => statehub.delete_state(state).await,
            Command::ListStates => statehub.list_states().await,
            Command::ShowState { name } => statehub.show_state(name).await,
            Command::ListClusters => statehub.list_clusters().await,
            Command::RegisterCluster { name } => statehub.register_cluster(name).await,
            Command::UnregisterCluster { name } => statehub.unregister_cluster(name).await,
            Command::CreateVolume => statehub.create_volume().await,
            Command::DeleteVolume => statehub.delete_volume().await,
            Command::AddLocation { state, location } => {
                statehub.add_location(state, location).await
            }
            Command::RemoveLocation => statehub.remove_location().await,
            Command::SetAvailability => statehub.set_availability().await,
            Command::SetOwner { state, cluster } => statehub.set_owner(state, cluster).await,
            Command::UnsetOwner { state, cluster } => statehub.unset_owner(state, cluster).await,
            Command::ListNodes => Kubectl::list_nodes().await,
            Command::ListPods => Kubectl::list_pods().await,
            Command::ListRegions { zone } => statehub.list_regions(zone).await,
        }
    }
}

pub(crate) struct StateHub {
    api: api::Api,
    json: bool,
    raw: bool,
}

impl StateHub {
    fn new(
        management: String,
        token: Option<String>,
        json: bool,
        raw: bool,
        verbose: bool,
    ) -> Self {
        let api = api::Api::new(management, token, json, raw, verbose);

        Self { api, json, raw }
    }

    pub(crate) async fn create_state(
        &self,
        name: v1::StateName,
        owner: Option<v1::ClusterName>,
        locations: v1::Locations,
    ) -> anyhow::Result<()> {
        let text = |output| self.show(output);
        let state = v1::State {
            name,
            storage_class: None,
            owner,
            locations,
            allowed_clusters: None,
        };
        self.api.create_state(state).await.map(text).map(print)
    }

    pub(crate) async fn delete_state(&self, name: v1::StateName) -> anyhow::Result<()> {
        let text = |output| self.show(output);
        self.api.delete_state(name).await.map(text).map(print)
    }

    pub(crate) async fn show_state(self, state: v1::StateName) -> anyhow::Result<()> {
        let text = |output| self.show(output);
        self.api.show_state(state).await.map(text).map(print)
    }

    async fn list_states(&self) -> anyhow::Result<()> {
        let text = |output| self.show(output);
        self.api.get_states().await.map(text).map(print)
    }

    pub(crate) async fn list_clusters(&self) -> anyhow::Result<()> {
        let text = |output| self.show(output);
        self.api.list_clusters().await.map(text).map(print)
    }

    async fn register_cluster(&self, name: v1::ClusterName) -> anyhow::Result<()> {
        let text = |output| self.show(output);

        // Find where my nodes are located
        let locations = kubectl::collect_node_locations().await?;

        // Get all the states (user token allows to *see* all of them)
        let states = self.api.get_states().await?;

        // Verify that all the states are available in all the locations
        for state in states {
            for location in &locations {
                if !state.is_available_in(location) {
                    // need to extend the state to this location as well
                    self.add_location_helper(&state.name, location).await?;
                }
            }
        }

        self.api.register_cluster(name).await.map(text).map(print)
    }

    async fn unregister_cluster(&self, _name: v1::ClusterName) -> anyhow::Result<()> {
        // let text = |output| self.show(output);
        // Ok(Output::<String>::todo()).map(text).map(print)
        anyhow::bail!(self.show(Output::<String>::todo()))
    }

    async fn add_location(&self, state: v1::StateName, location: Location) -> anyhow::Result<()> {
        self.add_location_helper(&state, &location).await
    }

    pub(crate) async fn create_volume(self) -> anyhow::Result<()> {
        // let text = |output| self.show(output);
        // Ok(Output::<String>::todo()).map(text).map(print)
        anyhow::bail!(self.show(Output::<String>::todo()))
    }

    pub(crate) async fn delete_volume(self) -> anyhow::Result<()> {
        // let text = |output| self.show(output);
        // Ok(Output::<String>::todo()).map(text).map(print)
        anyhow::bail!(self.show(Output::<String>::todo()))
    }

    pub(crate) async fn remove_location(self) -> anyhow::Result<()> {
        // let text = |output| self.show(output);
        // Ok(Output::<String>::todo()).map(text).map(print)
        anyhow::bail!(self.show(Output::<String>::todo()))
    }

    pub(crate) async fn set_availability(self) -> anyhow::Result<()> {
        // let text = |output| self.show(output);
        // Ok(Output::<String>::todo()).map(text).map(print)
        anyhow::bail!(self.show(Output::<String>::todo()))
    }

    pub(crate) async fn set_owner(
        &self,
        state: v1::StateName,
        cluster: v1::ClusterName,
    ) -> anyhow::Result<()> {
        let text = |output| self.show(output);
        self.api
            .set_owner(state, cluster)
            .await
            .map(text)
            .map(print)
    }

    pub(crate) async fn unset_owner(
        &self,
        state: v1::StateName,
        cluster: v1::ClusterName,
    ) -> anyhow::Result<()> {
        let text = |output| self.show(output);
        self.api
            .unset_owner(state, cluster)
            .await
            .map(text)
            .map(print)
    }

    async fn add_location_helper(
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

    pub(crate) fn show<T>(&self, output: Output<T>) -> String
    where
        T: DeserializeOwned + Serialize + Show,
    {
        if self.json {
            output.into_value().show()
        } else if self.raw {
            output.show()
        } else {
            output.into_typed().show()
        }
    }

    async fn list_regions(&self, zone: bool) -> anyhow::Result<()> {
        kubectl::get_regions(zone)
            .await
            .map(|nodes| println!("{}", nodes.show()))
    }
}

fn print(text: String) {
    println!("{}", text);
}
