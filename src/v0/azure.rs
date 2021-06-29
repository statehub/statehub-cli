//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use super::*;

mod impls;

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeDisplay, DeserializeFromStr,
)]
pub enum AzureRegion {
    CentralUs,
    EastUs,
    EastUs2,
    FranceCentral,
    JapanEast,
    NorthEurope,
    SouthEastasia,
    UkSouth,
    WestEurope,
    WestUs2,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PrivateLinkServiceAzure {
    pub id: String,
}
