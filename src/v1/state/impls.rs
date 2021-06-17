//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::cmp;

use chrono_humanize::HumanTime;
use console::Emoji;

use super::*;

mod show;

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.debug_struct("State")
                .field("name", &self.name.0)
                .field("aws", &self.locations.aws)
                .field("azure", &self.locations.azure)
                .finish()
        } else {
            self.name.fmt(f)
        }
    }
}

impl From<String> for StateName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl From<&str> for StateName {
    fn from(text: &str) -> Self {
        text.to_string().into()
    }
}

impl fmt::Display for StateName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl str::FromStr for StateName {
    type Err = Infallible;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(text.into())
    }
}

impl AsRef<Self> for StateName {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl ops::Deref for StateName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl State {
    pub(crate) fn is_available_in(&self, location: &Location) -> bool {
        self.locations.contains(location)
    }

    fn show_owner(&self) -> String {
        self.owner
            .as_ref()
            .map(|cluster| format!("🔒 {}", cluster))
            .unwrap_or_else(|| "🔓".to_string())
    }

    fn collect_volumes(&self) -> HashMap<String, HashMap<Location, &VolumeLocation>> {
        let aws = self
            .locations
            .aws
            .iter()
            .map(|location| {
                location
                    .volumes
                    .iter()
                    .map(move |volume| (volume, location.region.into()))
            })
            .flatten();
        let azure = self
            .locations
            .azure
            .iter()
            .map(|location| {
                location
                    .volumes
                    .iter()
                    .map(move |volume| (volume, location.region.into()))
            })
            .flatten();

        let mut volumes = HashMap::<_, HashMap<_, _>>::new();
        for (volume, location) in aws.chain(azure) {
            volumes
                .entry(volume.name.clone())
                .or_default()
                .insert(location, volume);
        }
        volumes
    }

    fn count_volumes(&self) -> String {
        let aws = self
            .locations
            .aws
            .iter()
            .map(|aws| aws.volumes.len())
            .max()
            .unwrap_or_default();
        let azure = self
            .locations
            .azure
            .iter()
            .map(|azure| azure.volumes.len())
            .max()
            .unwrap_or_default();
        let count = cmp::max(aws, azure);
        if count == 1 {
            String::from("(1 volume)")
        } else {
            format!("({} volumes)", count)
        }
    }

    fn show_volumes(&self) -> String {
        self.collect_volumes()
            .into_iter()
            .map(|(name, locations)| {
                format!(
                    "{}: {}",
                    name,
                    locations
                        .iter()
                        .map(|(location, volume)| format!(
                            "{}: {}",
                            location,
                            volume.status.value.show()
                        ))
                        .join(", ")
                )
            })
            .join("\n")
    }
}

impl StateLocationStatus {
    const OK: Emoji<'static, 'static> = Emoji("🆗", "V");
    const PROVISIONING: Emoji<'static, 'static> = Emoji("⤴ 🔜", "+");
    const RECOVERING: Emoji<'static, 'static> = Emoji("🔄", "~");
    const DELETING: Emoji<'static, 'static> = Emoji("⤵", "-");
    const ERROR: Emoji<'static, 'static> = Emoji("❌", "X");

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Provisioning => "provisioning",
            Self::Recovering => "recovering",
            Self::Deleting => "deleting",
            Self::Error => "error",
        }
    }

    pub fn is_final(&self) -> bool {
        match self {
            Self::Ok => true,
            Self::Provisioning => false,
            Self::Recovering => false,
            Self::Deleting => false,
            Self::Error => true,
        }
    }
}

impl Default for VolumeBindingMode {
    fn default() -> Self {
        Self::WaitForFirstConsumer
    }
}

impl From<AwsRegion> for CreateStateLocationAwsDto {
    fn from(region: AwsRegion) -> Self {
        Self { region }
    }
}

impl From<AzureRegion> for CreateStateLocationAzureDto {
    fn from(region: AzureRegion) -> Self {
        Self { region }
    }
}

impl From<Vec<Location>> for CreateStateLocationsDto {
    fn from(locations: Vec<Location>) -> Self {
        let mut aws = vec![];
        let mut azure = vec![];
        for location in locations {
            match location {
                Location::Aws(region) => aws.push(region.into()),
                Location::Azure(region) => azure.push(region.into()),
                // Location::Gcp(region) => gcp.push(region),
            }
        }
        Self { aws, azure }
    }
}

impl StateLocations {
    pub(crate) fn contains(&self, location: &Location) -> bool {
        match location {
            Location::Aws(region) => self.aws.iter().any(|aws| aws.region == *region),
            Location::Azure(region) => self.azure.iter().any(|azure| azure.region == *region),
        }
    }
}

impl Condition {
    pub const GREEN: Emoji<'static, 'static> = Emoji("🟢", "G");
    pub const YELLOW: Emoji<'static, 'static> = Emoji("🟡", "Y");
    pub const RED: Emoji<'static, 'static> = Emoji("🔴", "R");
}

impl Default for Condition {
    fn default() -> Self {
        Self::Green
    }
}
