//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use super::*;

mod impls;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct StateName(pub String);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStateDto {
    pub name: StateName,
    pub storage_class: Option<StorageClass>,
    pub locations: CreateStateLocationsDto,
    pub owner: Option<ClusterName>,
    pub allowed_clusters: Option<Vec<ClusterName>>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ProvisioningStatus {
    Ready,
    Provisioning,
    Error,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Condition {
    Green,
    Yellow,
    Red,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StateLocations {
    #[serde(default)]
    pub aws: Vec<StateLocationAws>,
    #[serde(default)]
    pub azure: Vec<StateLocationAzure>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateLocationAzure {
    pub region: AzureRegion,
    pub status: StateLocationStatus,
    pub volumes: Vec<VolumeLocation>,
    pub private_link_service: Option<PrivateLinkServiceAzure>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateLocationAws {
    pub region: AwsRegion,
    pub status: StateLocationStatus,
    pub volumes: Vec<VolumeLocation>,
    pub private_link_service: Option<PrivateLinkServiceAws>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageClass {
    pub name: String,
    pub volume_binding_mode: VolumeBindingMode,
    pub fs_type: String,
    pub mount_options: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StateLocationStatus {
    Ok,
    Provisioning,
    Recovering,
    Deleting,
    Error,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateStateLocationsDto {
    pub aws: Vec<CreateStateLocationAwsDto>,
    pub azure: Vec<CreateStateLocationAzureDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateStateLocationAwsDto {
    pub region: AwsRegion,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateStateLocationAzureDto {
    pub region: AzureRegion,
}
