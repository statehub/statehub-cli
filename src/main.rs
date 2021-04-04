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

const ABOUT: &str = "statehub CLI tool";

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init_custom_env("STATEHUB_LOG");
    StateHub::from_args().execute()
}

#[derive(Debug, StructOpt)]
#[structopt(about = ABOUT)]
struct StateHub {
    #[structopt(short, long)]
    verbose: bool,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(about = "Create new state")]
    CreateState,
    #[structopt(about = "Register new cluster")]
    RegisterCluster,
    #[structopt(about = "Unregister existing cluster")]
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
    #[structopt(about = "Set specified cluster as owner for a given state")]
    SetOwner,
}

impl StateHub {
    fn execute(self) -> anyhow::Result<()> {
        println!("{}", self.verbose);
        match self.command {
            Command::CreateState => Command::create_state(),
            Command::RegisterCluster => Command::register_cluster(),
            Command::UnregisterCluster => Command::unregister_cluster(),
            Command::CreateVolume => Command::create_volume(),
            Command::DeleteVolume => Command::delete_volume(),
            Command::AddLocation => Command::add_location(),
            Command::RemoveLocation => Command::remove_location(),
            Command::SetAvailability => Command::set_availability(),
            Command::SetOwner => Command::set_owner(),
        }
    }
}

impl Command {
    fn create_state() -> anyhow::Result<()> {
        Ok(())
    }

    fn register_cluster() -> anyhow::Result<()> {
        Ok(())
    }

    fn unregister_cluster() -> anyhow::Result<()> {
        Ok(())
    }

    fn create_volume() -> anyhow::Result<()> {
        Ok(())
    }

    fn delete_volume() -> anyhow::Result<()> {
        Ok(())
    }

    fn add_location() -> anyhow::Result<()> {
        Ok(())
    }

    fn remove_location() -> anyhow::Result<()> {
        Ok(())
    }

    fn set_availability() -> anyhow::Result<()> {
        Ok(())
    }

    fn set_owner() -> anyhow::Result<()> {
        Ok(())
    }
}
