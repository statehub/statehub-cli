//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::fmt;
use std::str;

use super::*;
use crate::location::Location;

mod aws;
mod azure;
mod gcp;

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
