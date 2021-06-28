//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use super::*;

impl Show for State {
    fn show(&self) -> String {
        format!(
            "{} {:>24} {} {:<12} [{:#}] ({})",
            Self::STATE,
            self.name,
            self.condition.show(),
            self.count_volumes(),
            self.locations.show(),
            self.show_owner(),
        )
    }

    fn detailed_show(&self) -> String {
        let storage_class = self
            .storage_class
            .as_ref()
            .map(|sc| format!("{} ({})", sc.name, sc.fs_type))
            .unwrap_or_default();
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            format_args!("State:         {}", self.name),
            format_args!("Id:            {}", self.id),
            format_args!("Storage Class: {}", storage_class),
            format_args!("Owner:         {}", self.show_owner()),
            format_args!("Created:       {}", HumanTime::from(self.created)),
            format_args!("Modified:      {}", HumanTime::from(self.modified)),
            format_args!("Condition:     {}", self.condition.show()),
            format_args!("Locations:\n{}", self.locations.detailed_show()),
            format_args!("Volumes:\n{}", self.show_volumes())
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

    fn detailed_show(&self) -> String {
        let none = || String::from("None");
        let aws = self.aws.iter().map(|location| {
            format!(
                " {:#}:\n  {}\n  {}",
                location.region,
                format_args!("Status: {}", location.status.show()),
                format_args!(
                    "PLS   : {}",
                    location
                        .private_link_service
                        .as_ref()
                        .map_or_else(none, |pls| pls.detailed_show())
                ),
            )
        });
        let azure = self.azure.iter().map(|location| {
            format!(
                " {:#}:\n  {}\n  {}",
                location.region,
                format_args!("Status: {}", location.status.show()),
                format_args!(
                    "PLS   : {}",
                    location
                        .private_link_service
                        .as_ref()
                        .map_or_else(none, |pls| pls.detailed_show())
                )
            )
        });

        aws.chain(azure).join("\n")
    }
}

impl Show for StateLocationStatus {
    fn show(&self) -> String {
        let text = match self {
            Self::Ok => Self::OK,
            Self::Provisioning => Self::PROVISIONING,
            Self::Recovering => Self::RECOVERING,
            Self::Deleting => Self::DELETING,
            Self::Error => Self::ERROR,
        };
        text.to_string()
    }
}

impl Show for PrivateLinkServiceAws {
    fn show(&self) -> String {
        format!("{} / {}", self.id, self.name)
    }
}

impl Show for PrivateLinkServiceAzure {
    fn show(&self) -> String {
        self.id.clone()
    }
}

impl Show for Condition {
    fn show(&self) -> String {
        let condition = match self {
            Self::Green => Self::GREEN,
            Self::Yellow => Self::YELLOW,
            Self::Red => Self::RED,
        };
        format!("{}", condition)
    }
}
