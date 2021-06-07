//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::collections::HashMap;
use std::process::Command;

use itertools::{join, Itertools};

use crate::v1;

pub(crate) trait Show {
    fn show(self) -> String;
}

impl Show for String {
    fn show(self) -> String {
        self
    }
}

impl Show for v1::State {
    fn show(self) -> String {
        self.to_string()
    }
}

impl Show for Vec<v1::State> {
    fn show(self) -> String {
        self.into_iter().map(Show::show).join("\n")
    }
}

impl Show for v1::Cluster {
    fn show(self) -> String {
        self.to_string()
    }
}

impl Show for Vec<v1::Cluster> {
    fn show(self) -> String {
        self.into_iter()
            .map(|cluster| cluster.name)
            .map(|name| name.to_string())
            .collect()
    }
}

impl Show for () {
    fn show(self) -> String {
        String::new()
    }
}

impl Show for v1::Volume {
    fn show(self) -> String {
        volume(self)
    }
}

pub(crate) fn volume(volume: v1::Volume) -> String {
    let mut out = String::new();

    out += &format!("Volume  :{:>60}\n", volume.name);
    out += &format!("Size    :{:>56} GiB\n", volume.size_gi);
    out += &format!("FS Type :{:>60}\n", volume.fs_type);
    out += &format!("Status  :{:>60}\n", volume.status);

    out
}

impl Show for HashMap<Option<String>, Vec<String>> {
    fn show(self) -> String {
        let mut out = String::new();

        for (region, nodes) in self {
            let region = region.unwrap_or_else(|| String::from("No region"));
            out += &format!("{}:    {}\n", region, join(nodes, ", "));
        }
        out
    }
}

impl Show for Command {
    fn show(self) -> String {
        format!("{:?}", self).replace(r#" "#, " ")
    }
}

impl Show for Vec<Command> {
    fn show(self) -> String {
        self.into_iter().map(|cmd| cmd.show()).join("\n\n")
    }
}
