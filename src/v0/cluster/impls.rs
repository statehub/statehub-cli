//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::str;

use chrono_humanize::HumanTime;
use console::Emoji;

use crate::k8s;

use super::*;

impl Cluster {
    const CLUSTER: Emoji<'static, 'static> = Emoji("☸", "*");

    pub(crate) fn all_locations(&self) -> Vec<Location> {
        let aws = self.locations.aws.iter().map(|aws| aws.region.into());
        let azure = self.locations.azure.iter().map(|azure| azure.region.into());
        aws.chain(azure).collect()
    }
}

impl fmt::Display for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cluster").field("name", &self.name).finish()
    }
}

impl From<String> for ClusterName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl From<&str> for ClusterName {
    fn from(text: &str) -> Self {
        text.to_string().into()
    }
}

impl fmt::Display for ClusterName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl str::FromStr for ClusterName {
    type Err = Infallible;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(text.into())
    }
}

impl AsRef<str> for ClusterName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl ops::Deref for ClusterName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl PartialEq<&str> for ClusterName {
    fn eq(&self, other: &&str) -> bool {
        self.0.eq(other)
    }
}

impl From<AwsRegion> for ClusterLocationAws {
    fn from(region: AwsRegion) -> Self {
        Self {
            region,
            account_principal: None,
        }
    }
}

impl From<AzureRegion> for ClusterLocationAzure {
    fn from(region: AzureRegion) -> Self {
        Self { region }
    }
}

impl From<&[Location]> for ClusterLocations {
    fn from(locations: &[Location]) -> Self {
        let mut aws = vec![];
        let mut azure = vec![];
        for location in locations {
            match location {
                Location::Aws(region) => aws.push((*region).into()),
                Location::Azure(region) => azure.push((*region).into()),
                // Location::Gcp(region) => gcp.push(region),
            }
        }
        Self { aws, azure }
    }
}

impl Show for Cluster {
    fn show(&self) -> String {
        format!(
            "{} {:>24} [{:#}]",
            Self::CLUSTER,
            self.name,
            self.locations.show()
        )
    }

    fn detailed_show(&self) -> String {
        let helm = k8s::Helm::new("statehub-system".to_string(), None, true);
        let helm = helm.command(self);
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            format_args!("Cluster:     {}", self.name),
            format_args!("Id:          {}", self.id),
            format_args!("Locations:   {}", self.locations.show()),
            format_args!("Created:     {}", HumanTime::from(self.created)),
            format_args!("Modified:    {}", HumanTime::from(self.modified)),
            format_args!("Helm install:\n{}", helm.show())
        )
    }
}

impl Show for Vec<Cluster> {
    fn show(&self) -> String {
        self.iter().map(Show::show).join("\n")
    }
}

impl Show for ClusterLocations {
    fn show(&self) -> String {
        let aws = self
            .aws
            .iter()
            .map(|location| format!("{:#}", location.region));
        let azure = self
            .azure
            .iter()
            .map(|location| format!("{:#}", location.region));
        aws.chain(azure).join(", ")
    }
}

impl Provider {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Eks => "eks",
            Self::Aks => "aks",
            Self::Kops => "kops",
            Self::Generic => "generic",
        }
    }
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = self.as_str();
        if f.alternate() {
            text.to_uppercase().fmt(f)
        } else {
            text.fmt(f)
        }
    }
}

impl str::FromStr for Provider {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "eks" => Ok(Self::Eks),
            "aks" => Ok(Self::Aks),
            "kops" => Ok(Self::Kops),
            "genetic" => Ok(Self::Generic),
            other => anyhow::bail!("Invalid K8s provider: {}", other),
        }
    }
}
