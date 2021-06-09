//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::convert::Infallible;
use std::fmt;
use std::ops;
use std::str;

use crate::location::{InvalidRegion, Location};

use super::*;

mod aws;
mod azure;
mod gcp;
mod volume;

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("name", &self.name.0)
            .field("aws", &self.locations.aws)
            .field("azure", &self.locations.azure)
            .finish()
    }
}

impl From<String> for StateName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl From<&str> for StateName {
    fn from(text: &str) -> Self {
        text.to_string().into()
    }
}

impl fmt::Display for StateName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl str::FromStr for StateName {
    type Err = Infallible;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(text.into())
    }
}

impl AsRef<Self> for StateName {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl ops::Deref for StateName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
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

impl From<&str> for ClusterName {
    fn from(text: &str) -> Self {
        text.to_string().into()
    }
}

impl fmt::Display for ClusterName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl str::FromStr for ClusterName {
    type Err = Infallible;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(text.into())
    }
}

impl AsRef<Self> for ClusterName {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl ops::Deref for ClusterName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl Default for VolumeBindingMode {
    fn default() -> Self {
        Self::WaitForFirstConsumer
    }
}

impl Default for Condition {
    fn default() -> Self {
        Self::Green
    }
}

impl From<AwsRegion> for CreateStateLocationAwsDto {
    fn from(region: AwsRegion) -> Self {
        Self { region }
    }
}

impl From<AzureRegion> for CreateStateLocationAzureDto {
    fn from(region: AzureRegion) -> Self {
        Self { region }
    }
}

impl From<AwsRegion> for ClusterLocationAws {
    fn from(region: AwsRegion) -> Self {
        Self {
            region,
            account_principal: None,
        }
    }
}

impl From<AzureRegion> for ClusterLocationAzure {
    fn from(region: AzureRegion) -> Self {
        Self { region }
    }
}

impl From<&[Location]> for ClusterLocations {
    fn from(locations: &[Location]) -> Self {
        let mut aws = vec![];
        let mut azure = vec![];
        for location in locations {
            match location {
                Location::Aws(region) => aws.push((*region).into()),
                Location::Azure(region) => azure.push((*region).into()),
                // Location::Gcp(region) => gcp.push(region),
            }
        }
        Self { aws, azure }
    }
}

impl From<Vec<Location>> for CreateStateLocationsDto {
    fn from(locations: Vec<Location>) -> Self {
        let mut aws = vec![];
        let mut azure = vec![];
        for location in locations {
            match location {
                Location::Aws(region) => aws.push(region.into()),
                Location::Azure(region) => azure.push(region.into()),
                // Location::Gcp(region) => gcp.push(region),
            }
        }
        Self { aws, azure }
    }
}

impl StateLocations {
    pub(crate) fn contains(&self, location: &Location) -> bool {
        match location {
            Location::Aws(region) => self.aws.iter().any(|aws| aws.region == *region),
            Location::Azure(region) => self.azure.iter().any(|azure| azure.region == *region),
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

trait CloudRegion {
    const VENDOR: &'static str;
    const VENDOR_PREFIX: &'static str;
    fn as_str(&self) -> &'static str;
}
