//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use uuid::Uuid;

mod impls;

pub const VERSION: &str = "";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub id: Uuid,
    pub name: StateName,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub storage_class: Option<StorageClass>,
    #[serde(default)]
    pub locations: StateLocations,
    pub owner: Option<ClusterName>,
    pub provisioning_status: ProvisioningStatus,
    pub allowed_clusters: Option<Vec<ClusterName>>,
    pub condition: Condition,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStateDto {
    pub name: StateName,
    pub storage_class: Option<StorageClass>,
    pub locations: CreateStateLocationsDto,
    pub owner: Option<ClusterName>,
    pub allowed_clusters: Option<Vec<ClusterName>>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ProvisioningStatus {
    Ready,
    Provisioning,
    Error,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Condition {
    Green,
    Yellow,
    Red,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateClusterDto {
    pub name: ClusterName,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cluster {
    pub id: Uuid,
    pub name: ClusterName,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub locations: Locations,
    pub helm: Vec<Helm>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClusterToken {
    pub token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Helm {
    pub repo: String,
    pub chart: String,
    pub version: String,
    pub parameters: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StateName(pub String);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterName(pub String);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateStateLocationsDto {
    aws: Vec<CreateStateLocationAwsDto>,
    azure: Vec<CreateStateLocationAzureDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateStateLocationAwsDto {
    pub region: AwsRegion,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateStateLocationAzureDto {
    pub region: AzureRegion,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StateLocations {
    #[serde(default)]
    pub aws: Vec<StateLocationAws>,
    #[serde(default)]
    pub azure: Vec<StateLocationAzure>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateLocationAzure {
    pub region: AzureRegion,
    pub status: StateLocationStatus,
    pub volumes: Vec<StateLocationVolume>,
    pub private_link_service: Option<PrivateLinkServiceAzure>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateLocationAws {
    pub region: AwsRegion,
    pub status: StateLocationStatus,
    pub volumes: Vec<StateLocationVolume>,
    pub private_link_service: Option<PrivateLinkServiceAws>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivateLinkServiceAws {
    pub id: String,
    pub name: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivateLinkServiceAzure {
    pub id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StateLocationVolume {
    pub status: StateLocationVolumeStatus,
    pub progress: Option<StateLocationVolumeProgress>,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StateLocationVolumeStatus {
    pub status: StateLocationStatus,
    pub msg: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateLocationVolumeProgress {
    pub bytes_syncronized: u64,
    pub bytes_total: u64,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum StateLocationStatus {
    Ok,
    Provisioning,
    Recovering,
    Deleting,
    Error,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageClass {
    pub mount_options: Option<String>,
    pub volume_binding_mode: VolumeBindingMode,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VolumeBindingMode {
    WaitForFirstConsumer,
    Immediate,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Locations {
    aws: Vec<ClusterLocationAws>,
    azure: Vec<ClusterLocationAzure>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterLocationAws {
    region: AwsRegion,
    account_principal: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClusterLocationAzure {
    region: AzureRegion,
}

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

#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeDisplay, DeserializeFromStr,
)]
pub enum GcpRegion {
    Antarctica,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeName(pub String);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    pub id: Uuid,
    pub name: String,
    pub size_gi: u64,
    pub fs_type: String,
    pub active_location: Locations,
    pub status: VolumeStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVolumeDto {
    pub name: String,
    pub size_gi: u64,
    pub fs_type: String,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeDisplay, DeserializeFromStr,
)]
pub enum VolumeFileSystem {
    EXT,
    EXT2,
    EXT3,
    EXT4,
    JFS,
    SWAP,
    FAT,
    FAT32,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeDisplay, DeserializeFromStr,
)]
pub enum VolumeStatus {
    Ok,
    Degraded,
    Error,
    Syncing,
    Pending,
}
