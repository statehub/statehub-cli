//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::collections::HashMap;
use std::fmt;
use std::io;

use anyhow::Context;
use console::Term;
use dialoguer::{theme, Confirm, Input};
use itertools::Itertools;
use serde::{de::DeserializeOwned, Serialize};
use structopt::StructOpt;
// use structopt::clap;

use crate::api;
use crate::k8s;
use crate::show::{Detailed, Quiet};
use crate::traits::Show;
use crate::v0;
use crate::Location;
use crate::Output;

use config::Config;
use helper::AddLocation;

mod config;
mod helper;

const ABOUT: &str = "statehub CLI tool";

#[derive(Debug, StructOpt)]
#[structopt(about = ABOUT)]
pub(crate) struct Cli {
    #[structopt(
        help = "Management server URL or address",
        // default_value = "https://api.statehub.io",
        short,
        long,
        env = "SHAPI"
    )]
    management: Option<String>,
    #[structopt(
        help = "Console server URL or address",
        // default_value = "https://console.statehub.io",
        short,
        long,
        env = "SHCONSOLE"
    )]
    console: Option<String>,
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
    #[structopt(about = "Authenticate against statehub service", display_order(0))]
    Login,
    #[structopt(about = "Create new state", aliases = &["create-st", "cs"], display_order(20))]
    CreateState {
        #[structopt(help = "State name")]
        name: v0::StateName,
        #[structopt(long, short, help = "Defalt owning cluster")]
        owner: Option<v0::ClusterName>,
        #[structopt(long, short, help = "Location definition")]
        location: Vec<Location>,
    },

    #[structopt(about = "Delete existing state", aliases = &["delete-st", "ds"], display_order(20))]
    DeleteState {
        #[structopt(help = "State name")]
        name: v0::StateName,
    },

    #[structopt(about = "List available states", aliases = &["list-state", "list-st", "ls"], display_order(20))]
    ListStates,

    #[structopt(about = "Show state details", aliases = &["show-s", "ss"], display_order(20))]
    ShowState {
        #[structopt(help = "State name")]
        name: v0::StateName,
    },

    #[structopt(about = "Register new cluster", aliases = &["register-cl", "rc"], display_order(10))]
    RegisterCluster {
        #[structopt(help = "Cluster name, defaults to current k8s context")]
        name: Option<v0::ClusterName>,

        #[structopt(
            help = "List of states to for this cluster to use",
            long,
            alias = "state",
            default_value = "default"
        )]
        states: Vec<v0::StateName>,

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
            default_value = "statehub-system"
        )]
        namespace: String,

        #[structopt(help = "Skip running 'helm install'", long)]
        skip_helm: bool,

        #[structopt(help = "K8s cluster provider [default: autodetect]", long)]
        provider: Option<v0::Provider>,
    },

    #[structopt(about = "Unregister existing cluster", aliases = &["unregister-cl", "uc"], display_order(11))]
    UnregisterCluster {
        #[structopt(help = "Skip confirmation", long, short)]
        force: bool,
        #[structopt(help = "Cluster name")]
        name: v0::ClusterName,
    },

    #[structopt(about = "List registered clusters", aliases = &["list-cluster", "list-cl", "lc"], display_order(12))]
    ListClusters,

    #[structopt(about = "Show registered cluster details", aliases = &["show-cl", "sc"], display_order(13))]
    ShowCluster {
        #[structopt(help = "Cluster name")]
        name: Option<v0::ClusterName>,
    },

    #[structopt(
        about = "Manually make state available in a specified location",
        aliases = &["add-l", "al"],
        display_order(30)
    )]
    AddLocation {
        #[structopt(help = "State name")]
        state: v0::StateName,
        #[structopt(
            help = "Location specification",
            conflicts_with = "cluster",
            required_unless = "cluster"
        )]
        location: Option<Location>,
        #[structopt(
            help = "Add locations from this cluster",
            long,
            short,
            conflicts_with = "location",
            required_unless = "location"
        )]
        cluster: Option<v0::ClusterName>,
        #[structopt(help = "Wait until new location is ready", long)]
        wait: bool,
    },

    #[structopt(
        about = "Manually make state unavailable in a specified location",
        aliases = &["remove-l", "rem-l", "rl"],
        display_order(30)
    )]
    RemoveLocation {
        #[structopt(help = "State name")]
        state: v0::StateName,
        #[structopt(help = "Location specification")]
        location: Location,
    },

    #[structopt(about = "Set state availability grade", display_order(40))]
    SetAvailability,

    #[structopt(about = "Set cluster as the state owner", display_order(40))]
    SetOwner {
        #[structopt(help = "State name")]
        state: v0::StateName,
        #[structopt(help = "Cluster name")]
        cluster: v0::ClusterName,
    },

    #[structopt(about = "Clear state owner", display_order(40))]
    UnsetOwner {
        #[structopt(help = "State name")]
        state: v0::StateName,
        #[structopt(help = "Cluster name")]
        cluster: v0::ClusterName,
    },

    #[structopt(about = "Manually create new volume", aliases = &["create-v", "cv"], display_order(50))]
    CreateVolume {
        #[structopt(help = "State name")]
        state: v0::StateName,
        #[structopt(help = "Volume name")]
        volume: v0::VolumeName,
        #[structopt(help = "valume size")]
        size: u64,
        #[structopt(help = "volume file system")]
        fs_type: v0::VolumeFileSystem,
    },

    #[structopt(about = "Manually delete existing volume", aliases = &["delete-v", "dv"], display_order(50))]
    DeleteVolume {
        #[structopt(help = "State name")]
        state: v0::StateName,
        #[structopt(help = "Volume name")]
        volume: v0::VolumeName,
        #[structopt(help = "Wait until volume is deleted", long)]
        wait: bool,
    },

    #[structopt(
        about = "Manually choose primary location for volume",
        aliases = &["set-v", "sv"],
        display_order(50)
    )]
    SetVolume {
        #[structopt(help = "State name")]
        state: v0::StateName,
        #[structopt(help = "Volume name")]
        volume: v0::VolumeName,
        #[structopt(help = "Primary location specification", long, short)]
        primary: Location,
    },

    #[structopt(
        about = "List volumes in the given state",
        aliases = &["list-volume", "list-v", "lv"],
        display_order(50)
    )]
    ListVolumes {
        #[structopt(help = "State name")]
        state: v0::StateName,
    },

    #[structopt(
        about = "Create new namespace",
        aliases = &["cns", "c-ns", "create-ns"],
        display_order(1000),
        // setting(clap::AppSettings::Hidden)
    )]
    CreateNamespace {
        #[structopt(help = "Namespace name", default_value = "statehub-system")]
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
        about = "Setup statehub configmap",
        aliases = &["scm", "setup-cm"],
        display_order(1000),
        // setting(clap::AppSettings::Hidden)
    )]
    SetupConfigmap {
        #[structopt(help = "cluster name")]
        cluster: v0::ClusterName,
        #[structopt(
            help = "Namespace to install statehub components",
            default_value = "statehub-system"
        )]
        namespace: String,
        #[structopt(
            help = "The name of the state to configure as default storage class, nothing by default"
        )]
        default_state: Option<String>,
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

    #[structopt(about = "List K8s node regions", aliases = &["list-r", "lr"], display_order(1000))]
    ListRegions {
        #[structopt(help = "Also list zones", long, short)]
        zone: bool,
    },

    #[structopt(about = "Save default configuration file", display_order(2000))]
    SaveConfig,
}

impl Cli {
    pub(crate) async fn execute() -> anyhow::Result<()> {
        Self::from_args().dispatch().await
    }

    async fn config(&self) -> anyhow::Result<Config> {
        let config = Config::load().unwrap_or_default();
        Ok(config)
    }

    async fn dispatch(self) -> anyhow::Result<()> {
        let config = self
            .config()
            .await?
            .optionally_management_api(self.management)
            .optionally_management_console(self.console)
            .set_token(self.token);

        let statehub = StateHub::new(config, self.json, self.verbose);

        statehub.validate_auth().await?;

        match self.command {
            Command::Login => statehub.login().await,
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
            Command::RegisterCluster {
                name,
                states,
                no_state,
                no_default_storage_class,
                default_storage_class,
                no_state_owner,
                namespace,
                skip_helm,
                provider,
            } => {
                let name = name.or_else(k8s::get_cluster_name).ok_or_else(|| {
                    anyhow::anyhow!(
                        "No default Kubernetes context found, need to provide cluster name"
                    )
                })?;
                let no_default_storage_class = if no_state {
                    true
                } else {
                    no_default_storage_class
                };
                let default_state = if no_default_storage_class {
                    None
                } else if default_storage_class.is_none() {
                    states.first().map(ToString::to_string)
                } else {
                    default_storage_class
                };
                let states = if no_state { None } else { Some(states) };
                let claim_unowned_states = !no_state_owner;
                let helm = k8s::Helm::new(namespace, default_state, skip_helm);
                statehub
                    .register_cluster(name, provider, states, helm, claim_unowned_states)
                    .await
            }
            Command::UnregisterCluster { force, name } => {
                statehub.unregister_cluster(name, force).await
            }
            Command::ListClusters => statehub.list_clusters().await,
            Command::ShowCluster { name } => {
                let name = name.or_else(k8s::get_cluster_name).ok_or_else(|| {
                    anyhow::anyhow!(
                        "No default Kubernetes context found, need to provide cluster name"
                    )
                })?;
                statehub.show_cluster(name).await
            }
            Command::AddLocation {
                state,
                location,
                cluster,
                wait,
            } => {
                let location = location.map(AddLocation::FromLocation);
                let cluster = cluster.map(AddLocation::FromCluster);
                if let Some(add_location) = location.or(cluster) {
                    statehub.add_location(state, add_location, wait).await
                } else {
                    anyhow::bail!("Need to specify either location or --cluster");
                }
            }
            Command::RemoveLocation { state, location } => {
                statehub.remove_location(state, location).await
            }
            Command::SetAvailability => statehub.set_availability().await,
            Command::SetOwner { state, cluster } => statehub.set_owner(state, cluster).await,
            Command::UnsetOwner { state, cluster } => statehub.unset_owner(state, cluster).await,
            Command::CreateVolume {
                state,
                volume,
                size,
                fs_type,
            } => statehub.create_volume(state, volume, size, fs_type).await,
            Command::DeleteVolume {
                state,
                volume,
                wait,
            } => statehub.delete_volume(state, volume, wait).await,
            Command::SetVolume {
                state,
                volume,
                primary,
            } => statehub.set_volume_primary(state, volume, primary).await,
            Command::ListVolumes { state } => statehub.list_volumes(state).await,
            Command::CreateNamespace { namespace } => statehub.create_namespace(namespace).await,
            Command::SaveClusterToken { namespace, token } => {
                statehub.save_cluster_token(namespace, token).await
            }
            Command::SetupConfigmap {
                namespace,
                cluster,
                default_state,
            } => {
                statehub
                    .setup_configmap(namespace, cluster, default_state)
                    .await
            }
            Command::ListNamespaces => statehub.list_namespaces().await,
            Command::ListNodes => statehub.list_nodes().await,
            Command::ListPods => statehub.list_pods().await,
            Command::ListRegions { zone } => statehub.list_regions(zone).await,
            Command::SaveConfig => statehub.save_config().await,
        }
    }
}

pub(crate) struct StateHub {
    config: Config,
    api: api::Api,
    stdout: Term,
    theme: theme::SimpleTheme,
    json: bool,
    verbose: bool,
}

impl StateHub {
    fn new(config: Config, json: bool, verbose: bool) -> Self {
        let api = api::Api::new(config.api(), config.token());
        let stdout = Term::stdout();
        let theme = theme::SimpleTheme;

        Self {
            config,
            api,
            stdout,
            theme,
            json,
            verbose,
        }
    }

    async fn validate_auth(&self) -> anyhow::Result<()> {
        if self.api.is_unauthorized().await {
            // log::error!("Unauthorized - perhaps an invalid token?");
            anyhow::bail!("Unauthorized - perhaps an invalid token?");
        }

        Ok(())
    }

    async fn login(&self) -> anyhow::Result<()> {
        let console = self.config.console();
        let (token, id) = self.login_prompt_helper()?;

        let prompt = format!(
            "{}\n{}\n\n{}\n{}",
            "Welcome to Statehub! Please login or sign up at:",
            format_args!("{}/?cli={}", console, token),
            "Waiting for authentication to complete in your browser...",
            format_args!(
                "Please paste the token generated for {} here, and press RETURN",
                id
            )
        );

        let token = self.input(prompt)?;

        self.config.clone().set_token(Some(token)).save()?;
        Ok(())
    }

    async fn create_state(
        &self,
        name: v0::StateName,
        owner: Option<v0::ClusterName>,
        locations: v0::CreateStateLocationsDto,
    ) -> anyhow::Result<()> {
        let state = v0::CreateStateDto {
            name,
            storage_class: None,
            owner,
            locations,
            allowed_clusters: None,
        };
        self.api
            .create_state(state)
            .await
            .handle_output(&self.stdout, self.json)
    }

    async fn delete_state(&self, name: v0::StateName) -> anyhow::Result<()> {
        self.api
            .delete_state(name)
            .await
            .handle_output(&self.stdout, self.json)
    }

    async fn show_state(self, state: &v0::StateName) -> anyhow::Result<()> {
        let state = self.api.get_state(state).await.map(Detailed)?;
        if let Ok(clusters) = self.api.get_all_clusters().await {
            (state, clusters).handle_output(&self.stdout, self.json)
        } else {
            state.handle_output(&self.stdout, self.json)
        }
    }

    async fn list_states(&self) -> anyhow::Result<()> {
        self.api
            .get_all_states()
            .await
            .handle_output(&self.stdout, self.json)
    }

    async fn list_clusters(&self) -> anyhow::Result<()> {
        self.api
            .get_all_clusters()
            .await
            .handle_output(&self.stdout, self.json)
    }

    async fn register_cluster(
        &self,
        cluster: v0::ClusterName,
        provider: Option<v0::Provider>,
        states: Option<Vec<v0::StateName>>,
        helm: k8s::Helm,
        claim_unowned_states: bool,
    ) -> anyhow::Result<()> {
        let helm = if k8s::helm_is_found() {
            helm
        } else {
            log::warn!("helm is not detected. Will show helm command instead of executing them");
            helm.skip()
        };

        let locations = k8s::collect_node_locations().await?;
        let provider = if let Some(provider) = provider {
            provider
        } else {
            k8s::get_cluster_provider(&cluster).await?
        };

        let cluster = self
            .api
            .register_cluster(&cluster, provider, &locations)
            .await
            .map(Quiet)?;

        self.inform(format_args!(
            "Registering {:#} cluster {} in {}",
            provider,
            cluster.name,
            locations.show(),
        ))?;
        if let Some(ref states) = states {
            self.adjust_all_states(states, &locations).await?;
        } else {
            self.verbosely("Skip adding this cluster to any state")?;
        }

        k8s::validate_namespace(helm.namespace()).await?;

        self.setup_cluster_token_helper(&cluster, &helm).await?;
        self.setup_configmap_helper(&cluster, &helm).await?;

        helm.execute(&cluster)
            .await
            .and_then(|text| self.inform(text))?;

        if claim_unowned_states {
            self.claim_unowned_states_helper(&cluster, states).await?;
        }

        cluster.handle_output(&self.stdout, self.json)
    }

    async fn show_cluster(&self, name: v0::ClusterName) -> anyhow::Result<()> {
        let cluster = self.api.get_cluster(&name).await.map(Detailed)?;
        if let Ok(states) = self.api.get_all_states().await {
            (cluster, states).handle_output(&self.stdout, self.json)
        } else {
            cluster.handle_output(&self.stdout, self.json)
        }
    }

    async fn unregister_cluster(&self, name: v0::ClusterName, force: bool) -> anyhow::Result<()> {
        if force || self.confirm("Are you sure?") {
            log::info!("Make sure all the pods using statehub are terminated");
            log::info!("Uninstall helm");

            self.relinquish_states_helper(&name).await?;

            self.api
                .unregister_cluster(name)
                .await
                .handle_output(&self.stdout, self.json)
        } else {
            Ok(())
        }
    }

    async fn add_location(
        &self,
        state: v0::StateName,
        add_location: AddLocation,
        wait: bool,
    ) -> anyhow::Result<()> {
        let locations = match add_location {
            AddLocation::FromLocation(location) => vec![location],
            AddLocation::FromCluster(cluster) => {
                let locations = self.api.get_cluster(&cluster).await?.all_locations();
                self.inform(&format!(
                    "Cluster {} available in {}",
                    cluster,
                    locations.iter().map(ToString::to_string).join(" and ")
                ))?;
                locations
            }
        };

        for location in locations {
            let state = self.api.get_state(&state).await?;
            if state.is_available_in(&location) {
                self.inform(format_args!(
                    "State {} is already available in {:#}",
                    state, location
                ))?;
            } else {
                self.inform(format_args!("Extending state {} to {:#}", state, location))?;
                self.add_location_helper(&state, &location, wait).await?;
            }
        }
        Ok(())
    }

    async fn remove_location(self, state: v0::StateName, location: Location) -> anyhow::Result<()> {
        let state = self.api.get_state(&state).await?;
        if state.is_available_in(&location) {
            self.remove_location_helper(&state, &location).await?;
        } else {
            log::info!("{} is not availabe in {}", state, location);
        }
        Ok(())
    }

    async fn create_volume(
        self,
        state_name: v0::StateName,
        volume_name: v0::VolumeName,
        size: u64,
        fs: v0::VolumeFileSystem,
    ) -> anyhow::Result<()> {
        let volume = v0::CreateVolumeDto {
            name: volume_name.to_string(),
            size_gi: size,
            fs_type: fs.to_string(),
        };

        self.api
            .create_volume(state_name, volume)
            .await
            .handle_output(&self.stdout, self.json)
    }

    async fn delete_volume(
        self,
        state: v0::StateName,
        volume: v0::VolumeName,
        wait: bool,
    ) -> anyhow::Result<()> {
        if let Ok(volume) = self.api.get_volume(&state, &volume).await {
            if volume.is_deleting() {
                self.verbosely(format_args!(
                    "Volume {} is already being deleted",
                    volume.name
                ))?;
            } else {
                self.delete_volume_helper(&state, &volume.name, wait)
                    .await
                    .map(Quiet)
                    .handle_output(&self.stdout, self.json)?;
            }
        } else {
            self.verbosely("No such volume")?;
        }
        Ok(())
    }

    async fn set_volume_primary(
        &self,
        state: v0::StateName,
        volume: v0::VolumeName,
        primary: Location,
    ) -> anyhow::Result<()> {
        self.api
            .set_volume_primary(state, volume, primary)
            .await
            .handle_output(&self.stdout, self.json)
    }

    async fn list_volumes(&self, state: v0::StateName) -> anyhow::Result<()> {
        self.api
            .get_all_volumes(state)
            .await
            .handle_output(&self.stdout, self.json)
    }

    async fn set_availability(self) -> anyhow::Result<()> {
        // Ok(Output::<String>::todo()).handle_output(&self.stdout, self.json)
        anyhow::bail!(self.show(Output::<String>::todo()))
    }

    async fn set_owner(
        &self,
        state: v0::StateName,
        cluster: v0::ClusterName,
    ) -> anyhow::Result<()> {
        self.api
            .set_owner(&state, &cluster)
            .await
            .handle_output(&self.stdout, self.json)
    }

    async fn unset_owner(
        &self,
        state: v0::StateName,
        cluster: v0::ClusterName,
    ) -> anyhow::Result<()> {
        let state = self.api.get_state(&state).await?;

        if state.owner == Some(cluster) {
            self.api
                .unset_owner(&state.name)
                .await
                .handle_output(&self.stdout, self.json)
        } else {
            anyhow::bail!("Permission denied, you are not theowner of this state.")
        }
    }

    pub(crate) fn show<T>(&self, output: Output<T>) -> String
    where
        T: DeserializeOwned + Serialize + Show,
    {
        output.into_text(self.json)
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

    async fn setup_configmap(
        &self,
        namespace: String,
        cluster: v0::ClusterName,
        default_state: Option<String>,
    ) -> anyhow::Result<()> {
        let default_state = default_state.as_deref().unwrap_or("");
        let api = self.api.url("");
        let _configmap = k8s::store_configmap(&namespace, &cluster, default_state, &api).await?;

        Ok(())
    }

    async fn list_regions(&self, zone: bool) -> anyhow::Result<()> {
        k8s::get_regions(zone)
            .await
            .map(|map| {
                map.into_iter()
                    .map(|(key, value)| (key.unwrap_or_default(), value))
                    .collect::<HashMap<_, _>>()
            })
            .map(Output::from)
            .handle_output(&self.stdout, self.json)
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

    async fn save_config(&self) -> anyhow::Result<()> {
        self.config.save().and_then(|path| {
            self.inform(format_args!("Saving config to {}", path.display()))
                .context("Inform")
        })
    }

    fn confirm(&self, prompt: impl Into<String>) -> bool {
        let theme = self.theme();
        Confirm::with_theme(theme)
            .with_prompt(prompt)
            .default(false)
            .show_default(true)
            .interact()
            .unwrap_or(false)
    }

    fn theme(&self) -> &dyn theme::Theme {
        &self.theme
    }

    fn verbosely(&self, text: impl fmt::Display) -> io::Result<()> {
        if self.verbose {
            self.stdout.write_line(&text.to_string())
        } else {
            Ok(())
        }
    }

    fn inform(&self, text: impl fmt::Display) -> io::Result<()> {
        let text = text.to_string();
        if !text.is_empty() {
            self.stdout.write_line(&text)?;
        }
        Ok(())
    }

    fn input(&self, prompt: impl Into<String>) -> io::Result<String> {
        Input::with_theme(self.theme())
            .with_prompt(prompt)
            .interact_text_on(&self.stdout)
    }
}

trait HandleOutput {
    fn handle_output(self, stdout: &Term, json: bool) -> anyhow::Result<()>;
}

impl<T> HandleOutput for anyhow::Result<T>
where
    T: HandleOutput,
{
    fn handle_output(self, stdout: &Term, json: bool) -> anyhow::Result<()> {
        self?.handle_output(stdout, json)
    }
}

impl<T> HandleOutput for Output<T>
where
    T: DeserializeOwned + Serialize + Show,
{
    fn handle_output(self, stdout: &Term, json: bool) -> anyhow::Result<()> {
        let text = self.into_text(json);
        stdout.write_line(&text)?;
        Ok(())
    }
}

impl<T> HandleOutput for Detailed<Output<T>>
where
    T: DeserializeOwned + Serialize + Show,
{
    fn handle_output(self, stdout: &Term, json: bool) -> anyhow::Result<()> {
        let text = self.into_text(json);
        stdout.write_line(&text)?;
        Ok(())
    }
}

impl<T> HandleOutput for Quiet<Output<T>>
where
    T: DeserializeOwned + Serialize + Show,
{
    fn handle_output(self, stdout: &Term, json: bool) -> anyhow::Result<()> {
        let text = self.into_text(json);
        stdout.write_line(&text)?;
        Ok(())
    }
}

impl HandleOutput for (Detailed<Output<v0::State>>, Output<Vec<v0::Cluster>>) {
    fn handle_output(self, stdout: &Term, json: bool) -> anyhow::Result<()> {
        let (state, clusters) = self;
        let state_locations = state.all_locations();

        state.handle_output(stdout, json)?;

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

impl HandleOutput for (Detailed<Output<v0::Cluster>>, Output<Vec<v0::State>>) {
    fn handle_output(self, stdout: &Term, json: bool) -> anyhow::Result<()> {
        let (cluster, states) = self;
        let cluster_locations = cluster.all_locations();

        cluster.handle_output(stdout, json)?;

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
