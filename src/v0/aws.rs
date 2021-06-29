//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use super::*;

mod impls;

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeDisplay, DeserializeFromStr,
)]
pub enum AwsRegion {
    ApNortheast1,
    ApNortheast2,
    ApSouth1,
    ApSoutheast1,
    ApSoutheast2,
    CaCentral1,
    EuCentral1,
    EuNorth1,
    EuWest1,
    EuWest2,
    EuWest3,
    SaEast1,
    UsEast1,
    UsEast2,
    UsWest1,
    UsWest2,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PrivateLinkServiceAws {
    pub id: String,
    pub name: String,
}
