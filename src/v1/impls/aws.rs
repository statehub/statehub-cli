//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::fmt;
use std::str;

use super::*;

impl AwsRegion {
    const PREFIX: &'static str = "aws:";
    fn as_str(&self) -> &'static str {
        match self {
            Self::ApNortheast1 => "ap-northeast-1",
            Self::ApNortheast2 => "ap-northeast-2",
            Self::ApSouth1 => "ap-south-1",
            Self::ApSoutheast1 => "ap-southeast-1",
            Self::ApSoutheast2 => "ap-southeast-2",
            Self::CaCentral1 => "ca-central-1",
            Self::EuCentral1 => "eu-central-1",
            Self::EuNorth1 => "eu-north-1",
            Self::EuWest1 => "eu-west-1",
            Self::EuWest2 => "eu-west-2",
            Self::EuWest3 => "eu-west-3",
            Self::SaEast1 => "sa-east-1",
            Self::UsEast1 => "us-east-1",
            Self::UsEast2 => "us-east-2",
            Self::UsWest1 => "us-west-1",
            Self::UsWest2 => "us-west-2",
        }
    }
}

impl str::FromStr for AwsRegion {
    type Err = InvalidRegion;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let text = s.strip_prefix(Self::PREFIX).unwrap_or(s);
        match text {
            "ap-northeast-1" => Ok(Self::ApNortheast1),
            "ap-northeast-2" => Ok(Self::ApNortheast2),
            "ap-south-1" => Ok(Self::ApSouth1),
            "ap-southeast-1" => Ok(Self::ApSoutheast1),
            "ap-southeast-2" => Ok(Self::ApSoutheast2),
            "ca-central-1" => Ok(Self::CaCentral1),
            "eu-central-1" => Ok(Self::EuCentral1),
            "eu-north-1" => Ok(Self::EuNorth1),
            "eu-west-1" => Ok(Self::EuWest1),
            "eu-west-2" => Ok(Self::EuWest2),
            "eu-west-3" => Ok(Self::EuWest3),
            "sa-east-1" => Ok(Self::SaEast1),
            "us-east-1" => Ok(Self::UsEast1),
            "us-east-2" => Ok(Self::UsEast2),
            "us-west-1" => Ok(Self::UsWest1),
            "us-west-2" => Ok(Self::UsWest2),
            other => Err(InvalidRegion::aws(other)),
        }
    }
}

impl fmt::Display for AwsRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            format!("{}{}", Self::PREFIX, self.as_str()).fmt(f)
        } else {
            self.as_str().fmt(f)
        }
    }
}
