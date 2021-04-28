//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::fmt;
use std::str;

use crate::location::{InvalidRegion, Location};

use super::*;

mod aws;
mod azure;
mod gcp;

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State").field("name", &self.name).finish()
    }
}

impl From<String> for StateName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl fmt::Display for StateName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl str::FromStr for StateName {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(text.to_string().into())
    }
}

impl State {
    pub(crate) fn is_available_in(&self, location: &Location) -> bool {
        self.locations.contains(location)
    }
}

impl fmt::Display for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cluster").field("name", &self.name).finish()
    }
}

impl From<String> for ClusterName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl fmt::Display for ClusterName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl str::FromStr for ClusterName {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(text.to_string().into())
    }
}

impl Default for VolumeBindingMode {
    fn default() -> Self {
        Self::WaitForFirstConsumer
    }
}

impl From<Vec<Location>> for Locations {
    fn from(locations: Vec<Location>) -> Self {
        let mut aws = vec![];
        let mut azure = vec![];
        for location in locations {
            match location {
                Location::Aws(region) => aws.push(region),
                Location::Azure(region) => azure.push(region),
                // Location::Gcp(region) => gcp.push(region),
            }
        }
        Self { aws, azure }
    }
}

impl Locations {
    pub(crate) fn contains(&self, location: &Location) -> bool {
        match location {
            Location::Aws(region) => self.aws.contains(region),
            Location::Azure(region) => self.azure.contains(region),
        }
    }
}

impl VolumeStatus {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Degraded => "degraded",
            Self::Error => "error",
            Self::Syncing => "syncing",
            Self::Pending => "pending",
        }
    }
}

impl fmt::Display for VolumeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl str::FromStr for VolumeStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let status = match s {
            "ok" => Self::Ok,
            "degraded" => Self::Degraded,
            "error" => Self::Error,
            "syncing" => Self::Syncing,
            "pending" => Self::Pending,
            other => anyhow::bail!("Inknown volume status: {}", other),
        };

        Ok(status)
    }
}
