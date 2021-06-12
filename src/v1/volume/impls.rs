use std::fmt;
use std::str;
use thiserror::Error;

use super::*;

impl From<String> for VolumeName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl fmt::Display for VolumeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl str::FromStr for VolumeName {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(text.to_string().into())
    }
}

impl AsRef<Self> for VolumeName {
    fn as_ref(&self) -> &Self {
        self
    }
}

#[derive(Debug, Error)]
#[error(r#"Invalid file system type "{file_system}"#)]
pub struct InvalidVolumeFileSystem {
    file_system: String,
}

impl InvalidVolumeFileSystem {
    pub(crate) fn new(fs: &str) -> Self {
        let file_system = fs.to_string();
        Self { file_system }
    }
}

impl VolumeFileSystem {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Ext => "ext",
            Self::Ext2 => "ext2",
            Self::Ext3 => "ext3",
            Self::Ext4 => "ext4",
            Self::Jfs => "jfs",
            Self::Swap => "swap",
            Self::Fat => "fat",
            Self::Fat32 => "fat32",
        }
    }
}

impl str::FromStr for VolumeFileSystem {
    type Err = InvalidVolumeFileSystem;

    fn from_str(fs: &str) -> Result<Self, Self::Err> {
        match fs {
            "ext" => Ok(Self::Ext),
            "ext2" => Ok(Self::Ext2),
            "ext3" => Ok(Self::Ext3),
            "ext4" => Ok(Self::Ext4),
            "jfs" => Ok(Self::Jfs),
            "swap" => Ok(Self::Swap),
            "fat" => Ok(Self::Fat),
            "fat32" => Ok(Self::Fat32),
            other => Err(InvalidVolumeFileSystem::new(other)),
        }
    }
}

impl fmt::Display for VolumeFileSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = self.as_str();
        text.fmt(f)
    }
}

impl VolumeStatus {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Degraded => "degraded",
            Self::Error => "error",
            Self::Syncing => "syncing",
            Self::Pending => "pending",
        }
    }
}

impl fmt::Display for VolumeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl str::FromStr for VolumeStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let status = match s {
            "ok" => Self::Ok,
            "degraded" => Self::Degraded,
            "error" => Self::Error,
            "syncing" => Self::Syncing,
            "pending" => Self::Pending,
            other => anyhow::bail!("Inknown volume status: {}", other),
        };

        Ok(status)
    }
}

impl Show for Volume {
    fn show(&self) -> String {
        volume(self)
    }
}

pub(crate) fn volume(volume: &Volume) -> String {
    let mut out = String::new();

    out += &format!("Volume  :{:>60}\n", volume.name);
    out += &format!("Size    :{:>56} GiB\n", volume.size_gi);
    out += &format!("FS Type :{:>60}\n", volume.fs_type);

    out
}
