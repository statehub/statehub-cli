//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::fmt;
use std::str::FromStr;

use itertools::Itertools;
use thiserror::Error;

use crate::traits::Show;
use crate::v0;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum Location {
    Aws(v0::AwsRegion),
    Azure(v0::AzureRegion),
    // Gcp(v1::GcpRegion),
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Aws(region) => region.fmt(f),
            Self::Azure(region) => region.fmt(f),
        }
    }
}

impl From<v0::AwsRegion> for Location {
    fn from(region: v0::AwsRegion) -> Self {
        Self::Aws(region)
    }
}

impl From<v0::AzureRegion> for Location {
    fn from(region: v0::AzureRegion) -> Self {
        Self::Azure(region)
    }
}

impl FromStr for Location {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let aws = text.parse::<v0::AwsRegion>();
        let azure = text.parse::<v0::AzureRegion>();
        // let gcp = text.parse::<v1::GcpRegion>();

        match (aws, azure) {
            (Ok(aws), Err(_)) => Ok(Self::Aws(aws)),
            (Err(_), Ok(azure)) => Ok(Self::Azure(azure)),
            (Ok(aws), Ok(azure)) => Err(format!(
                "Ambiguous region, use either {:#} or {:#}",
                aws, azure
            )),
            (Err(aws), Err(azure)) => {
                let error = format!("{} or {}", aws, azure);
                Err(error)
            }
        }
    }
}

impl Show for Location {
    fn show(&self) -> String {
        format!("{:#}", self)
    }
}

impl Show for Vec<Location> {
    fn show(&self) -> String {
        self.iter().map(Show::show).join(" and ")
    }
}

pub(crate) trait CloudLocation {
    const PREFIX: &'static str;

    fn as_str(&self) -> &'static str;
}

#[derive(Debug, Error)]
#[error(r#"Invalid {vendor} region "{region}""#)]
pub struct InvalidRegion {
    vendor: String,
    region: String,
}

impl InvalidRegion {
    pub(crate) fn new(vendor: &str, region: &str) -> Self {
        let vendor = vendor.to_string();
        let region = region.to_string();
        Self { vendor, region }
    }
}
