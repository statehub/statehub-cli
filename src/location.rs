//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::fmt;
use std::str::FromStr;

use thiserror::Error;

use crate::v1;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Location {
    Aws(v1::AwsRegion),
    Azure(v1::AzureRegion),
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

impl FromStr for Location {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let aws = text.parse::<v1::AwsRegion>();
        let azure = text.parse::<v1::AzureRegion>();
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
    pub(crate) fn aws(region: impl ToString) -> Self {
        let vendor = "AWS".to_string();
        let region = region.to_string();
        Self { vendor, region }
    }

    pub(crate) fn azure(region: impl ToString) -> Self {
        let vendor = "Azure".to_string();
        let region = region.to_string();
        Self { vendor, region }
    }
}
