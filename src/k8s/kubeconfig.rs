//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use super::*;

const CLUSTER_DELIMITER: char = '/';

pub(super) trait KubeconfigExt {
    fn default_context(&self) -> Option<&str>;
    // fn validate_contexts(&self, names: Vec<String>) -> anyhow::Result<Vec<String>>;
    fn contains(&self, name: impl AsRef<str>) -> bool;
    fn current_context(&self) -> Option<&str>;
    fn all_contexts(&self) -> Vec<&str>;
}

impl KubeconfigExt for Kubeconfig {
    fn default_context(&self) -> Option<&str> {
        self.current_context
            .as_deref()
            .or_else(|| self.contexts.get(0).map(|context| context.name.as_str()))
            .or_else(|| self.clusters.get(0).map(|cluster| cluster.name.as_str()))
    }

    // fn validate_contexts(&self, contexts: Vec<String>) -> anyhow::Result<Vec<String>> {
    //     let all_contexts = self.contexts.iter().map(|context| &context.name);
    //     let all_clusters = self.clusters.iter().map(|cluster| &cluster.name);
    //     let all_names = all_contexts.chain(all_clusters).collect::<Vec<_>>();
    //     for name in &contexts {
    //         if !all_names.contains(&name) {
    //             anyhow::bail!("No such context or cluster: '{}'", name);
    //         }
    //     }
    //     Ok(contexts)
    // }

    fn contains(&self, name: impl AsRef<str>) -> bool {
        let name = name.as_ref();
        let all_contexts = self.contexts.iter().map(|context| &context.name);
        let all_clusters = self.clusters.iter().map(|cluster| &cluster.name);
        all_contexts.chain(all_clusters).any(|this| this == name)
    }

    fn current_context(&self) -> Option<&str> {
        self.current_context.as_deref()
    }

    fn all_contexts(&self) -> Vec<&str> {
        self.contexts
            .iter()
            .map(|context| context.name.as_str())
            .collect()
    }
}

pub(super) fn normalize_name(name: &str) -> v0::ClusterName {
    if let Some((_, tail)) = name.rsplit_once(CLUSTER_DELIMITER) {
        tail.into()
    } else {
        name.into()
    }
}
