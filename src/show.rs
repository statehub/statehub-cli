//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use crate::traits::Show;
use itertools::{join, Itertools};
use std::collections::HashMap;
use std::process::Command;

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
