//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt;
use std::ops;
use std::str;

use chrono::{DateTime, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_with::{skip_serializing_none, DeserializeFromStr, SerializeDisplay};
use uuid::Uuid;

use crate::location::{InvalidRegion, Location};
use crate::traits::{CloudRegion, Show};

pub use aws::{AwsRegion, PrivateLinkServiceAws};
pub use azure::{AzureRegion, PrivateLinkServiceAzure};
pub use cluster::{
    Cluster, ClusterLocationAws, ClusterLocationAzure, ClusterLocations, ClusterName, ClusterToken,
    CreateClusterDto, Provider,
};
pub use error::{Error, Permission, StatehubError};
pub use gcp::GcpRegion;
pub use helm::Helm;
pub use login::Login;
pub use state::{
    Condition, CreateStateDto, CreateStateLocationAwsDto, CreateStateLocationAzureDto,
    CreateStateLocationsDto, ProvisioningStatus, State, StateLocationAws, StateLocationAzure,
    StateLocationStatus, StateLocations, StateName,
};
pub use volume::{
    CreateVolumeDto, LocationVolumeStatus, StateLocationVolumeProgress, Volume, VolumeBindingMode,
    VolumeFileSystem, VolumeLocation, VolumeName, VolumeStatus,
};

mod aws;
mod azure;
mod cluster;
mod error;
mod gcp;
mod helm;
mod login;
mod state;
mod volume;

pub const VERSION: &str = "/v0";
