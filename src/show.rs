//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::collections::HashMap;
use std::process::Command;

use itertools::{join, Itertools};

use crate::traits::Show;

pub(crate) use detailed::Detailed;
pub(crate) use fun::get_label;
pub(crate) use quiet::Quiet;

mod detailed;
mod fun;
mod quiet;

impl Show for String {
    fn show(&self) -> String {
        self.to_string()
    }
}

impl Show for () {
    fn show(&self) -> String {
        String::new()
    }
}

impl Show for HashMap<String, Vec<String>> {
    fn show(&self) -> String {
        let mut out = String::new();

        for (region, nodes) in self {
            let region = if region.is_empty() {
                "No region"
            } else {
                region.as_str()
            };
            out += &format!("{}:    {}\n", region, join(nodes, ", "));
        }
        out
    }
}

impl Show for Command {
    fn show(&self) -> String {
        format!("{:?}", self).replace("\"", "")
    }

    fn detailed_show(&self) -> String {
        format!("             {:?}", self).replace("\"", "")
    }
}

impl Show for Vec<Command> {
    fn show(&self) -> String {
        self.iter().map(Show::show).join("\n")
    }

    fn detailed_show(&self) -> String {
        self.iter().map(Show::detailed_show).join("\n")
    }
}
