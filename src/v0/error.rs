//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

#![allow(clippy::use_self)]

use thiserror::Error;

use super::*;

mod impls;

#[derive(Debug, Error, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[error("{msg}")]
pub struct Error {
    http_code: u16,
    http_status: String,
    error: StatehubError,
    msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "errorCode")]
pub enum StatehubError {
    InvalidToken,
    #[serde(rename_all = "camelCase")]
    ClusterNotAuthorized {
        permission: Permission,
        resource_name: String,
        resource_type: String,
    },
    ClusterNameConflict {
        cluster: ClusterName,
    },
    ClusterNotFound {
        cluster: ClusterName,
    },
    ClusterIsStateOwner {
        cluster: ClusterName,
        state: StateName,
    },
    StateNameConflict {
        state: StateName,
    },
    StateNotFound {
        state: StateName,
    },
    AwsLocationExists {
        state: StateName,
        region: AwsRegion,
    },
    AzureLocationExists {
        state: StateName,
        region: AzureRegion,
    },
    UnknownError {
        message: String,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, SerializeDisplay, DeserializeFromStr)]
pub enum Permission {
    ReadClusters,
    CreateClusters,
    DeleteClusters,
    CreateClusterToken,
    ReadClusterToken,
    DeleteClusterToken,
    ReadClusterLocations,
    UpdateClusterLocations,
    ReadStates,
    CreateStates,
    DeleteStates,
    CreateStateOwner,
    DeleteStateOwner,
    CreateStateLocations,
    ReadStateLocations,
    DeleteStateLocations,
    ReadStateLocationPrincipals,
    CreateStateLocationPrincipals,
    UpdateStateLocationPle,
    ReadVolumes,
    CreateVolumes,
    DeleteVolumes,
    UpdateVolumeActiveLocation,
    DeleteVolumeActiveLocation,
    ReadOrganization,
    UpdateOrganization,
    ReadOrganizationRole,
    ReadPersonalTokens,
    CreatePersonalTokens,
    UpdatePersonalTokens,
    DeletePersonalTokens,
    ReadInvitations,
    CreateInvitations,
    UpdateInvitations,
    DeleteInvitations,
    ReadMembers,
    CreateMembers,
    UpdateMembers,
    DeleteMembers,
    ReadProfile,
    UpdateProfile,
}
