//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::fmt;

use serde::{de::DeserializeOwned, Serialize};
use structopt::StructOpt;
// use structopt::clap;

use crate::api;
use crate::k8s;
use crate::show::Show;
use crate::v1;
use crate::Location;
use crate::Output;

mod helper;

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
    #[structopt(long, global = true, help = "Show results as JSON")]
    json: bool,
    #[structopt(short, long, global = true)]
    verbose: bool,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(about = "Create new state", aliases = &["create-st", "cs"], display_order(20))]
    CreateState {
        #[structopt(help = "State name")]
        name: v1::StateName,
        #[structopt(long, short, help = "Defalt owning cluster")]
        owner: Option<v1::ClusterName>,
        #[structopt(long, short, help = "Location definition")]
        location: Vec<Location>,
    },

    #[structopt(about = "Delete existing state", aliases = &["delete-st", "ds"], display_order(20))]
    DeleteState {
        #[structopt(help = "State name")]
        name: v1::StateName,
    },

    #[structopt(about = "List available states", aliases = &["list-state", "list-st", "ls"], display_order(20))]
    ListStates,

    #[structopt(about = "Show state details", aliases = &["show-s", "ss"], display_order(20))]
    ShowState {
        #[structopt(help = "State name")]
        name: v1::StateName,
    },

    #[structopt(about = "List registered clusters", aliases = &["list-cluster", "list-cl", "lc"], display_order(10))]
    ListClusters,

    #[structopt(about = "Register new cluster", aliases = &["register-cl", "rc"], display_order(10))]
    RegisterCluster {
        #[structopt(help = "Cluster name")]
        name: v1::ClusterName,

        #[structopt(
            help = "List of states to for this cluster to use",
            long,
            alias = "state",
            default_value = "default"
        )]
        states: Vec<v1::StateName>,

        #[structopt(
            help = "Skip adding this cluster locations to any state",
            long,
            conflicts_with = "states"
        )]
        no_state: bool,

        #[structopt(
            help = "Skip setting up default storage class",
            long,
            conflicts_with = "default_storage_class"
        )]
        no_default_storage_class: bool,

        #[structopt(
            help = "The name of the state to configure as default storage class",
            long,
            conflicts_with_all = &["no_state", "no_default_storage_class"]
        )]
        default_storage_class: Option<String>,

        #[structopt(help = "Do not register this cluster as state owner", long)]
        no_state_owner: bool,

        #[structopt(
            help = "Namespace to install statehub components",
            long,
            default_value = "statehub"
        )]
        namespace: String,

        #[structopt(help = "Skip running 'helm install'", long)]
        skip_helm: bool,
    },

    #[structopt(about = "Unregister existing cluster", aliases = &["unregister-cl", "uc"], display_order(10))]
    UnregisterCluster {
        #[structopt(help = "Skip confirmation", long, short)]
        force: bool,
        #[structopt(help = "Cluster name")]
        name: v1::ClusterName,
    },

    #[structopt(
        about = "Manually make state available in a specified location",
        aliases = &["add-l", "al"],
        display_order(30)
    )]
    AddLocation {
        #[structopt(help = "State name")]
        state: v1::StateName,
        #[structopt(help = "Location specification")]
        location: Location,
    },

    #[structopt(
        about = "Manually make state unavailable in a specified location",
        aliases = &["remove-l", "rem-l", "rl"],
        display_order(30)
    )]
    RemoveLocation {
        #[structopt(help = "State name")]
        state: v1::StateName,
        #[structopt(help = "Location specification")]
        location: Location,
    },

    #[structopt(about = "Set state availability grade", display_order(40))]
    SetAvailability,

    #[structopt(about = "Set cluster as the state owner", display_order(40))]
    SetOwner {
        #[structopt(help = "State name")]
        state: v1::StateName,
        #[structopt(help = "Cluster name")]
        cluster: v1::ClusterName,
    },

    #[structopt(about = "Clear state owner", display_order(40))]
    UnsetOwner {
        #[structopt(help = "State name")]
        state: v1::StateName,
        #[structopt(help = "Cluster name")]
        cluster: v1::ClusterName,
    },

    #[structopt(about = "Manually create new volume", display_order(50))]
    CreateVolume,

    #[structopt(about = "Manually delete existing volume", display_order(50))]
    DeleteVolume,

    #[structopt(
        about = "Create new namespace",
        alias = "c-ns",
        display_order(1000),
        // setting(clap::AppSettings::Hidden)
    )]
    CreateNamespace {
        #[structopt(help = "Namespace name")]
        namespace: String,
    },

    #[structopt(
        about = "Save cluster token",
        alias = "sct",
        display_order(1000),
        // setting(clap::AppSettings::Hidden)
    )]
    SaveClusterToken {
        #[structopt(help = "Namespace name")]
        namespace: String,
        #[structopt(help = "Cluster token")]
        token: String,
    },

    #[structopt(
        about = "List K8s namespaces",
        alias = "list-ns",
        display_order(1000),
        // setting(clap::AppSettings::Hidden)
    )]
    ListNamespaces,

    #[structopt(
        about = "List K8s nodes",
        display_order(1000),
        // setting(clap::AppSettings::Hidden)
    )]
    ListNodes,

    #[structopt(
        about = "List K8s pods",
        display_order(1000),
        // setting(clap::AppSettings::Hidden)
    )]
    ListPods,

    #[structopt(about = "List K8s node regions", display_order(1000))]
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
        let statehub = StateHub::new(self.management, self.token, self.json, self.verbose);

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
            Command::ShowState { name } => statehub.show_state(&name).await,
            Command::ListClusters => statehub.list_clusters().await,
            Command::RegisterCluster {
                name,
                states,
                no_state,
                no_default_storage_class,
                default_storage_class,
                no_state_owner,
                namespace,
                skip_helm,
            } => {
                let no_default_storage_class = if no_state {
                    true
                } else {
                    no_default_storage_class
                };
                let default_storage_class = if no_default_storage_class {
                    None
                } else if default_storage_class.is_none() {
                    states.first().map(|state| state.to_string())
                } else {
                    default_storage_class
                };
                let states = if no_state { None } else { Some(states) };
                let claim_unowned_states = !no_state_owner;
                let helm = k8s::Helm::new(namespace, default_storage_class, skip_helm);
                statehub
                    .register_cluster(name, states, helm, claim_unowned_states)
                    .await
            }
            Command::UnregisterCluster { force, name } => {
                statehub.unregister_cluster(name, force).await
            }
            Command::AddLocation { state, location } => {
                statehub.add_location(state, location).await
            }
            Command::RemoveLocation { state, location } => {
                statehub.remove_location(state, location).await
            }
            Command::SetAvailability => statehub.set_availability().await,
            Command::SetOwner { state, cluster } => statehub.set_owner(state, cluster).await,
            Command::UnsetOwner { state, cluster } => statehub.unset_owner(state, cluster).await,
            Command::CreateVolume => statehub.create_volume().await,
            Command::DeleteVolume => statehub.delete_volume().await,
            Command::CreateNamespace { namespace } => statehub.create_namespace(namespace).await,
            Command::SaveClusterToken { namespace, token } => {
                statehub.save_cluster_token(namespace, token).await
            }
            Command::ListNamespaces => statehub.list_namespaces().await,
            Command::ListNodes => statehub.list_nodes().await,
            Command::ListPods => statehub.list_pods().await,
            Command::ListRegions { zone } => statehub.list_regions(zone).await,
        }
    }
}

pub(crate) struct StateHub {
    api: api::Api,
    json: bool,
    verbose: bool,
}

impl StateHub {
    fn new(management: String, token: Option<String>, json: bool, verbose: bool) -> Self {
        let api = api::Api::new(management, token, verbose);

        Self { api, json, verbose }
    }

    pub(crate) async fn create_state(
        &self,
        name: v1::StateName,
        owner: Option<v1::ClusterName>,
        locations: v1::Locations,
    ) -> anyhow::Result<()> {
        let state = v1::CreateStateDto {
            name,
            storage_class: None,
            owner,
            locations,
            allowed_clusters: None,
        };
        self.api.create_state(state).await.handle_output(self.json)
    }

    pub(crate) async fn delete_state(&self, name: v1::StateName) -> anyhow::Result<()> {
        self.api.delete_state(name).await.handle_output(self.json)
    }

    pub(crate) async fn show_state(self, state: &v1::StateName) -> anyhow::Result<()> {
        self.api.get_state(state).await.handle_output(self.json)
    }

    async fn list_states(&self) -> anyhow::Result<()> {
        self.api.get_all_states().await.handle_output(self.json)
    }

    pub(crate) async fn list_clusters(&self) -> anyhow::Result<()> {
        self.api.list_clusters().await.handle_output(self.json)
    }

    async fn register_cluster(
        &self,
        cluster: v1::ClusterName,
        states: Option<Vec<v1::StateName>>,
        helm: k8s::Helm,
        claim_unowned_states: bool,
    ) -> anyhow::Result<()> {
        if !k8s::helm_is_found() {
            anyhow::bail!(
                "helm is not detected, please make sure helm is installed and is in the PATH"
            );
        }

        if let Some(ref states) = states {
            let locations = k8s::collect_node_locations().await?;
            self.adjust_all_states(states, &locations).await?;
        } else {
            self.verbosely("## Skip adding this cluster to any state");
        }

        let output = self.api.register_cluster(&cluster).await?;

        k8s::validate_namespace(helm.namespace()).await?;

        self.setup_cluster_token_helper(&output, &helm).await?;

        helm.execute(&output, self.verbose).await?;

        if claim_unowned_states {
            if let Some(states) = states {
                for state in states {
                    self.api.set_owner(state, &cluster).await?;
                }
            }
        }

        output.handle_output(self.json)
    }

    async fn unregister_cluster(&self, name: v1::ClusterName, force: bool) -> anyhow::Result<()> {
        if force || self.confirm("Are you sure?") {
            log::info!("Make sure all the pods using statehub are terminated");
            log::info!("Uninstall helm");

            self.relinquish_states_helper(&name).await?;

            self.api
                .unregister_cluster(name)
                .await
                .handle_output(self.json)
        } else {
            Ok(())
        }
    }

    async fn add_location(&self, state: v1::StateName, location: Location) -> anyhow::Result<()> {
        self.add_location_helper(&state, &location).await
    }

    async fn remove_location(self, state: v1::StateName, location: Location) -> anyhow::Result<()> {
        self.remove_location_helper(&state, &location).await
    }

    pub(crate) async fn create_volume(self) -> anyhow::Result<()> {
        // Ok(Output::<String>::todo()).handle_output(self.json)
        anyhow::bail!(self.show(Output::<String>::todo()))
    }

    pub(crate) async fn delete_volume(self) -> anyhow::Result<()> {
        // Ok(Output::<String>::todo()).handle_output(self.json)
        anyhow::bail!(self.show(Output::<String>::todo()))
    }

    pub(crate) async fn set_availability(self) -> anyhow::Result<()> {
        // Ok(Output::<String>::todo()).handle_output(self.json)
        anyhow::bail!(self.show(Output::<String>::todo()))
    }

    pub(crate) async fn set_owner(
        &self,
        state: v1::StateName,
        cluster: v1::ClusterName,
    ) -> anyhow::Result<()> {
        self.api
            .set_owner(state, cluster)
            .await
            .handle_output(self.json)
    }

    pub(crate) async fn unset_owner(
        &self,
        state: v1::StateName,
        cluster: v1::ClusterName,
    ) -> anyhow::Result<()> {
        self.api
            .unset_owner(state, cluster)
            .await
            .handle_output(self.json)
    }

    pub(crate) fn show<T>(&self, output: Output<T>) -> String
    where
        T: DeserializeOwned + Serialize + Show,
    {
        output.into_text(self.json)
    }

    async fn list_regions(&self, zone: bool) -> anyhow::Result<()> {
        k8s::get_regions(zone)
            .await
            .map(|nodes| println!("{}", nodes.show()))
    }

    async fn create_namespace(&self, namespace: String) -> anyhow::Result<()> {
        k8s::validate_namespace(namespace)
            .await
            .map(|namespace| println!("{:#?}", namespace))
    }

    async fn save_cluster_token(&self, namespace: String, token: String) -> anyhow::Result<()> {
        let secret = k8s::store_cluster_token(&namespace, &token).await?;
        if let Some(token) = k8s::extract_cluster_token(&secret) {
            log::debug!("Token: {}", token);
        }
        Ok(())
    }

    async fn list_namespaces(&self) -> anyhow::Result<()> {
        k8s::list_namespaces()
            .await?
            .into_iter()
            .for_each(|namespace| println!("{:#?}", namespace));
        Ok(())
    }

    async fn list_nodes(&self) -> anyhow::Result<()> {
        k8s::list_nodes()
            .await?
            .into_iter()
            .for_each(|node| println!("{}", node.show()));
        Ok(())
    }

    async fn list_pods(&self) -> anyhow::Result<()> {
        k8s::list_pods()
            .await?
            .into_iter()
            .for_each(|pod| println!("{}", pod.show()));
        Ok(())
    }

    fn confirm(&self, text: impl fmt::Display) -> bool {
        print!("{}", text);
        let yes: String = text_io::read!();
        yes == "y" || yes == "yes"
    }

    fn verbosely(&self, text: impl fmt::Display) {
        if self.verbose {
            println!("{}", text)
        }
    }
}

trait HandleOutput {
    fn handle_output(self, json: bool) -> anyhow::Result<()>;
}

impl<T> HandleOutput for api::ApiResult<T>
where
    T: DeserializeOwned + Serialize + Show,
{
    fn handle_output(self, json: bool) -> anyhow::Result<()> {
        self.and_then(|output| output.handle_output(json))
    }
}

impl<T> HandleOutput for Output<T>
where
    T: DeserializeOwned + Serialize + Show,
{
    fn handle_output(self, json: bool) -> anyhow::Result<()> {
        println!("{}", self.into_text(json));
        Ok(())
    }
}
