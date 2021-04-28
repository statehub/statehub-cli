//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::fmt;
use std::str;

use super::*;

impl CloudRegion for AzureRegion {
    const VENDOR: &'static str = "Azure";
    const VENDOR_PREFIX: &'static str = "azure/";

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
    type Err = InvalidRegion;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let text = s.strip_prefix(Self::VENDOR_PREFIX).unwrap_or(s);
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
            other => Err(InvalidRegion::new(Self::VENDOR, other)),
        }
    }
}

impl fmt::Display for AzureRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = self.as_str();
        if f.alternate() {
            format!("{}{}", Self::VENDOR_PREFIX, text).fmt(f)
        } else {
            text.fmt(f)
        }
    }
}
