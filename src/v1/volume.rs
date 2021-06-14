use super::*;
mod impls;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    pub id: Uuid,
    pub name: String,
    pub size_gi: u64,
    pub fs_type: String,
    pub active_location: Option<String>,
    pub locations: Vec<VolumeLocation>,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VolumeBindingMode {
    WaitForFirstConsumer,
    Immediate,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeName(pub String);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VolumeLocation {
    pub status: LocationVolumeStatus,
    pub progress: Option<StateLocationVolumeProgress>,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateLocationVolumeProgress {
    pub bytes_synchronized: u64,
    pub bytes_total: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocationVolumeStatus {
    pub value: StateLocationStatus,
    pub msg: Option<String>,
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
    Ext,
    Ext2,
    Ext3,
    Ext4,
    Jfs,
    Swap,
    Fat,
    Fat32,
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
