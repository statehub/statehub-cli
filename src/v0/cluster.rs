//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use super::*;

mod impls;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cluster {
    pub id: Uuid,
    pub name: ClusterName,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    #[serde(default)]
    pub locations: ClusterLocations,
    pub helm: Vec<Helm>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Eks,
    Aks,
    Kops,
    Generic,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateClusterDto {
    pub name: ClusterName,
    pub provider: Provider,
    pub locations: ClusterLocations,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClusterToken {
    pub token: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterName(pub String);

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ClusterLocations {
    pub aws: Vec<ClusterLocationAws>,
    pub azure: Vec<ClusterLocationAzure>,
}
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterLocationAws {
    pub region: AwsRegion,
    pub account_principal: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClusterLocationAzure {
    pub region: AzureRegion,
}
