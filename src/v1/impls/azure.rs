//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::fmt;
use std::str;

use thiserror::Error;

use super::*;

impl AzureRegion {
    const PREFIX: &'static str = "azure:";

    fn as_str(&self) -> &'static str {
        match self {
            Self::CentralUs => "centralus",
            Self::EastUs => "eastus",
            Self::EastUs2 => "eastus2",
            Self::FranceCentral => "francecentral",
            Self::JapanEast => "japaneast",
            Self::NorthEurope => "northeurope",
            Self::SouthEastasia => "southeastasia",
            Self::UkSouth => "uksouth",
            Self::WestEurope => "westeurope",
            Self::WestUs2 => "westus2",
        }
    }
}

impl str::FromStr for AzureRegion {
    type Err = InvalidAzureRegion;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let text = s.strip_prefix(Self::PREFIX).unwrap_or(s);
        match text {
            "centralus" => Ok(Self::CentralUs),
            "eastus" => Ok(Self::EastUs),
            "eastus2" => Ok(Self::EastUs2),
            "francecentral" => Ok(Self::FranceCentral),
            "japaneast" => Ok(Self::JapanEast),
            "northeurope" => Ok(Self::NorthEurope),
            "southeastasia" => Ok(Self::SouthEastasia),
            "uksouth" => Ok(Self::UkSouth),
            "westeurope" => Ok(Self::WestEurope),
            "westus2" => Ok(Self::WestUs2),
            other => Err(other.into()),
        }
    }
}

impl fmt::Display for AzureRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            format!("{}{}", Self::PREFIX, self.as_str()).fmt(f)
        } else {
            self.as_str().fmt(f)
        }
    }
}

#[derive(Debug, Error)]
#[error(r#"Invalid Azure Location "{0}""#)]
pub struct InvalidAzureRegion(String);

impl InvalidAzureRegion {
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<&str> for InvalidAzureRegion {
    fn from(location: &str) -> Self {
        let location = location.to_string();
        Self(location)
    }
}
