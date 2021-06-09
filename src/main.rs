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

use location::Location;
use output::Output;
use statehub::Cli;

// statehub API implementation
mod api;
// K8s interface and helpers
mod k8s;
// Location definitions
mod location;
// `Output` wrapper
mod output;
// `Show` trait definition and impls
mod show;
// Main tool bussines logic
mod statehub;

// stathub API v1 definitions
pub mod v1;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init_custom_env("STATEHUB_LOG");
    Cli::execute().await
}
