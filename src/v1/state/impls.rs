use super::*;

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("name", &self.name.0)
            .field("aws", &self.locations.aws)
            .field("azure", &self.locations.azure)
            .finish()
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
}

impl Default for VolumeBindingMode {
    fn default() -> Self {
        Self::WaitForFirstConsumer
    }
}

impl Default for Condition {
    fn default() -> Self {
        Self::Green
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

impl Show for State {
    fn show(&self) -> String {
        format!(
            "☘{:>16} {} [{:#}] ({})",
            self.name,
            self.condition.show(),
            self.locations.show(),
            self.show_owner(),
        )
    }

    fn detailed_show(&self) -> String {
        let storage_class = self
            .storage_class
            .as_ref()
            .map(|sc| sc.name.as_str())
            .unwrap_or("");
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            format_args!("State:         {}", self.name),
            format_args!("Id:            {}", self.id),
            format_args!("Locations:     {}", self.locations.show()),
            format_args!("Storage Class: {}", storage_class),
            format_args!("Owner:         {}", self.show_owner()),
            format_args!("Created:       {}", self.created),
            format_args!("Modified:      {}", self.modified),
            format_args!("Condition:     {}", self.condition.show())
        )
    }
}

impl Show for Vec<State> {
    fn show(&self) -> String {
        self.iter().map(Show::show).join("\n")
    }
}

impl Show for StateLocations {
    fn show(&self) -> String {
        let aws = self
            .aws
            .iter()
            .map(|location| format!("{:#} {}", location.region, location.status.show()));
        let azure = self
            .azure
            .iter()
            .map(|location| format!("{:#} {}", location.region, location.status.show()));
        aws.chain(azure).join(", ")
    }
}

impl Show for StateLocationStatus {
    fn show(&self) -> String {
        let text = match self {
            Self::Ok => "\u{1f197}",
            Self::Provisioning => "\u{1f3c3} \u{1f51c}",
            Self::Recovering => "",
            Self::Deleting => "",
            Self::Error => "\u{274c}",
        };
        text.into()
    }
}

impl Show for Condition {
    fn show(&self) -> String {
        let condition = match self {
            Self::Green => "🟢",
            Self::Yellow => "🟡",
            Self::Red => "🔴",
        };
        String::from(condition)
    }
}
