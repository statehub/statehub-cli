//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::collections::HashMap;
use std::process::Command;

use itertools::{join, Itertools};

use crate::v1;

pub(crate) trait Show {
    fn show(&self) -> String;
}

impl Show for String {
    fn show(&self) -> String {
        self.to_string()
    }
}

impl Show for v1::State {
    fn show(&self) -> String {
        let owner = if let Some(ref cluster) = self.owner {
            format!("🔒 {}", cluster)
        } else {
            "🔓".to_string()
        };
        format!(
            "☘{:>16} {} [{:#}] ({})",
            self.name,
            self.condition.show(),
            self.locations.show(),
            owner,
        )
    }
}

impl Show for Vec<v1::State> {
    fn show(&self) -> String {
        self.iter().map(Show::show).join("\n")
    }
}

impl Show for v1::Cluster {
    fn show(&self) -> String {
        format!("☸ {} [{:#}]", self.name, self.locations.show())
    }
}

impl Show for Vec<v1::Cluster> {
    fn show(&self) -> String {
        self.iter().map(Show::show).join("\n")
    }
}

impl Show for v1::ClusterLocations {
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

impl Show for v1::StateLocations {
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

impl Show for v1::StateLocationStatus {
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

impl Show for v1::Condition {
    fn show(&self) -> String {
        let condition = match self {
            Self::Green => "🟢",
            Self::Yellow => "🟡",
            Self::Red => "🔴",
        };
        String::from(condition)
    }
}

impl Show for () {
    fn show(&self) -> String {
        String::new()
    }
}

impl Show for v1::Volume {
    fn show(&self) -> String {
        volume(self)
    }
}

pub(crate) fn volume(volume: &v1::Volume) -> String {
    let mut out = String::new();

    out += &format!("Volume  :{:>60}\n", volume.name);
    out += &format!("Size    :{:>56} GiB\n", volume.size_gi);
    out += &format!("FS Type :{:>60}\n", volume.fs_type);

    out
}

impl Show for HashMap<Option<String>, Vec<String>> {
    fn show(&self) -> String {
        let mut out = String::new();

        for (region, nodes) in self {
            let region = region.as_deref().unwrap_or("No region");
            out += &format!("{}:    {}\n", region, join(nodes, ", "));
        }
        out
    }
}

impl Show for Command {
    fn show(&self) -> String {
        format!("{:?}", self).replace(r#" "#, " ")
    }
}

impl Show for Vec<Command> {
    fn show(&self) -> String {
        self.iter().map(|cmd| cmd.show()).join("\n\n")
    }
}
