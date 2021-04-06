//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use serde::{Deserialize, Serialize};

mod impls;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub name: StateName,
    pub storage_class: Option<StorageClass>,
    pub owner: Option<ClusterName>,
    pub locations: Locations,
    pub allowed_clusters: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cluster {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateName(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterName(pub String);

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StorageClass {
    pub mount_options: Option<String>,
    pub volume_binding_mode: VolumeBindingMode,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VolumeBindingMode {
    WaitForFirstConsumer,
    Immediate,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Locations {
    aws: Vec<AwsRegion>,
    azure: Vec<AzureRegion>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum GcpRegion {
    Antarctica,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    name: String,
    size_gi: u64,
    fs_type: String,
    owner: Option<String>,
}
