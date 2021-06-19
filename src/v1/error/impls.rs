//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use super::*;

impl Permission {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReadClusters => "read:clusters",
            Self::CreateClusters => "create:clusters",
            Self::DeleteClusters => "delete:clusters",
            Self::CreateClusterToken => "create:cluster_token",
            Self::ReadClusterToken => "read:cluster_token",
            Self::DeleteClusterToken => "delete:cluster_token",
            Self::ReadClusterLocations => "read:cluster_locations",
            Self::UpdateClusterLocations => "update:cluster_locations",
            Self::ReadStates => "read:states",
            Self::CreateStates => "create:states",
            Self::DeleteStates => "delete:states",
            Self::CreateStateOwner => "create:state_owner",
            Self::DeleteStateOwner => "delete:state_owner",
            Self::CreateStateLocations => "create:state_locations",
            Self::ReadStateLocations => "read:state_locations",
            Self::DeleteStateLocations => "delete:state_locations",
            Self::ReadStateLocationPrincipals => "read:state_location_principals",
            Self::CreateStateLocationPrincipals => "create:state_location_principals",
            Self::UpdateStateLocationPle => "update:state_location_ple",
            Self::ReadVolumes => "read:volumes",
            Self::CreateVolumes => "create:volumes",
            Self::DeleteVolumes => "delete:volumes",
            Self::UpdateVolumeActiveLocation => "update:volume_active_location",
            Self::DeleteVolumeActiveLocation => "delete:volume_active_location",
            Self::ReadOrganization => "read:organization",
            Self::UpdateOrganization => "update:organization",
            Self::ReadOrganizationRole => "read:organization_roles",
            Self::ReadPersonalTokens => "read:personal_tokens",
            Self::CreatePersonalTokens => "create:personal_tokens",
            Self::UpdatePersonalTokens => "update:personal_tokens",
            Self::DeletePersonalTokens => "delete:personal_tokens",
            Self::ReadInvitations => "read:invitations",
            Self::CreateInvitations => "create:invitations",
            Self::UpdateInvitations => "update:invitations",
            Self::DeleteInvitations => "delete:invitations",
            Self::ReadMembers => "read:members",
            Self::CreateMembers => "create:members",
            Self::UpdateMembers => "update:members",
            Self::DeleteMembers => "delete:members",
            Self::ReadProfile => "read:profile",
            Self::UpdateProfile => "update:profile",
        }
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl str::FromStr for Permission {
    type Err = anyhow::Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "read:clusters" => Ok(Self::ReadClusters),
            "create:clusters" => Ok(Self::CreateClusters),
            "delete:clusters" => Ok(Self::DeleteClusters),
            "create:cluster_token" => Ok(Self::CreateClusterToken),
            "read:cluster_token" => Ok(Self::ReadClusterToken),
            "delete:cluster_token" => Ok(Self::DeleteClusterToken),
            "read:cluster_locations" => Ok(Self::ReadClusterLocations),
            "update:cluster_locations" => Ok(Self::UpdateClusterLocations),
            "read:states" => Ok(Self::ReadStates),
            "create:states" => Ok(Self::CreateStates),
            "delete:states" => Ok(Self::DeleteStates),
            "create:state_owner" => Ok(Self::CreateStateOwner),
            "delete:state_owner" => Ok(Self::DeleteStateOwner),
            "create:state_locations" => Ok(Self::CreateStateLocations),
            "read:state_locations" => Ok(Self::ReadStateLocations),
            "delete:state_locations" => Ok(Self::DeleteStateLocations),
            "read:state_location_principals" => Ok(Self::ReadStateLocationPrincipals),
            "create:state_location_principals" => Ok(Self::CreateStateLocationPrincipals),
            "update:state_location_ple" => Ok(Self::UpdateStateLocationPle),
            "read:volumes" => Ok(Self::ReadVolumes),
            "create:volumes" => Ok(Self::CreateVolumes),
            "delete:volumes" => Ok(Self::DeleteVolumes),
            "update:volume_active_location" => Ok(Self::UpdateVolumeActiveLocation),
            "delete:volume_active_location" => Ok(Self::DeleteVolumeActiveLocation),
            "read:organization" => Ok(Self::ReadOrganization),
            "update:organization" => Ok(Self::UpdateOrganization),
            "read:organization_roles" => Ok(Self::ReadOrganizationRole),
            "read:personal_tokens" => Ok(Self::ReadPersonalTokens),
            "create:personal_tokens" => Ok(Self::CreatePersonalTokens),
            "update:personal_tokens" => Ok(Self::UpdatePersonalTokens),
            "delete:personal_tokens" => Ok(Self::DeletePersonalTokens),
            "read:invitations" => Ok(Self::ReadInvitations),
            "create:invitations" => Ok(Self::CreateInvitations),
            "update:invitations" => Ok(Self::UpdateInvitations),
            "delete:invitations" => Ok(Self::DeleteInvitations),
            "read:members" => Ok(Self::ReadMembers),
            "create:members" => Ok(Self::CreateMembers),
            "update:members" => Ok(Self::UpdateMembers),
            "delete:members" => Ok(Self::DeleteMembers),
            "read:profile" => Ok(Self::ReadProfile),
            "update:profile" => Ok(Self::UpdateProfile),
            other => Err(anyhow::anyhow!("Invalid permission '{}'", other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json as json;

    use super::*;

    #[test]
    fn invalid_token() {
        let text = r#"{"httpCode":401,"httpStatus":"Unauthorized","error":{"errorCode":"InvalidToken"},"msg":"string"}"#;
        let err: Error = json::from_str(text).unwrap();
        assert!(matches!(err.error, StateHubError::InvalidToken));
    }

    #[test]
    fn cluster_not_authorized1() {
        let text = r#"{"httpCode":403,"httpStatus":"Forbidden","error":{"errorCode":"ClusterNotAuthorized","permission":"read:clusters","resourceType":"string","resourceName":"string"},"msg":"string"}"#;
        let err: Error = json::from_str(text).unwrap();
        assert!(matches!(
            err.error,
            StateHubError::ClusterNotAuthorized {
                permission: Permission::ReadClusters,
                ..
            }
        ));
    }

    #[test]
    fn cluster_not_found() {
        let text = r#"{"httpCode":404,"httpStatus":"Not Found","error":{"errorCode":"ClusterNotFound","cluster":"zulu"},"msg":"string"}"#;
        let err: Error = json::from_str(text).unwrap();
        assert!(matches!(err.error, StateHubError::ClusterNotFound { .. }));
        if let StateHubError::ClusterNotFound { cluster } = err.error {
            assert_eq!(cluster, "zulu");
        }
    }
}
