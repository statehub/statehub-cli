//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::fmt;

use crate::show::Show;

use super::*;

impl Show for Node {
    fn show(self) -> String {
        let condition = self
            .status
            .as_ref()
            .and_then(|status| status.conditions.as_deref())
            .unwrap_or_default()
            .iter()
            .find(|condition| condition.status == "True")
            .map(|condition| condition.type_.as_str())
            .unwrap_or_default();

        self.namespace()
            .map(|ns| {
                fmt::format(format_args!(
                    "Node: {} ({}) status: {}",
                    self.name(),
                    ns,
                    condition
                ))
            })
            .unwrap_or_else(|| {
                fmt::format(format_args!("Node: {} status: {}", self.name(), condition))
            })
    }
}

impl Show for Pod {
    fn show(self) -> String {
        let status = self
            .status
            .as_ref()
            .and_then(|status| status.phase.as_deref())
            .unwrap_or_default();
        self.namespace()
            .map(|ns| {
                fmt::format(format_args!(
                    "Pod: {} ({}) status: {}",
                    self.name(),
                    ns,
                    status
                ))
            })
            .unwrap_or_else(|| fmt::format(format_args!("Pod: {} status: {}", self.name(), status)))
    }
}

impl Show for Namespace {
    fn show(self) -> String {
        format!("Namespace: {}", self.name())
    }
}
