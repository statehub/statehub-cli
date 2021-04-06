//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(clippy::use_self)]
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![deny(warnings)]

use structopt::StructOpt;

use location::Location;

mod api;
mod location;
mod output;
pub mod v1;

const ABOUT: &str = "statehub CLI tool";

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init_custom_env("STATEHUB_LOG");
    StateHub::from_args().execute()
}

#[derive(Debug, StructOpt)]
#[structopt(about = ABOUT)]
struct StateHub {
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
    #[structopt(about = "Create new state", alias = "create-st")]
    CreateState {
        #[structopt(help = "State name")]
        state: v1::StateName,
        #[structopt(long, short, help = "Defalt owning cluster")]
        owner: Option<v1::ClusterName>,
        #[structopt(help = "Location definition")]
        location: Vec<Location>,
    },
    #[structopt(about = "Create new state", aliases = &["list-state", "list-st", "ls"])]
    ListStates,
    #[structopt(about = "Register new cluster", aliases = &["list-cluster", "list-cl", "lc"])]
    ListClusters,
    #[structopt(about = "Register new cluster", aliases = &["register-cl", "rc"])]
    RegisterCluster,
    #[structopt(about = "Unregister existing cluster", aliases = &["unregister-cl", "uc"])]
    UnregisterCluster,
    #[structopt(about = "Manually create new volume")]
    CreateVolume,
    #[structopt(about = "Manually delete existing volume")]
    DeleteVolume,
    #[structopt(about = "Manually make state available in a specified location")]
    AddLocation,
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
}

impl StateHub {
    fn execute(self) -> anyhow::Result<()> {
        let api = api::Api::new(
            self.management,
            self.token,
            self.json,
            self.raw,
            self.verbose,
        );
        println!("{}", self.verbose);
        match self.command {
            Command::CreateState {
                state,
                owner,
                location,
            } => {
                let locations = location.into();
                api.create_state(state, owner, locations)
            }
            Command::ListStates => api.list_states(),
            Command::ListClusters => api.list_clusters(),
            Command::RegisterCluster => api.register_cluster(),
            Command::UnregisterCluster => api.unregister_cluster(),
            Command::CreateVolume => api.create_volume(),
            Command::DeleteVolume => api.delete_volume(),
            Command::AddLocation => api.add_location(),
            Command::RemoveLocation => api.remove_location(),
            Command::SetAvailability => api.set_availability(),
            Command::SetOwner { state, cluster } => api.set_owner(state, cluster),
            Command::UnsetOwner { state, cluster } => api.unset_owner(state, cluster),
        }
    }
}
