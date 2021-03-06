//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::collections::HashMap;

use k8s_openapi::api::core::v1::Node;
use kube::api::ResourceExt;

const K8S_TOPOLOGY_REGION: &str = "topology.kubernetes.io/region";
const K8S_TOPOLOGY_ZONE: &str = "topology.kubernetes.io/zone";
const AKS_CLUSTER: &str = "kubernetes.azure.com/cluster";

trait LabelsExt: ResourceExt {
    fn label(&self, label: impl AsRef<str>) -> Option<&str> {
        self.labels().get(label.as_ref()).map(String::as_str)
    }

    fn region(&self) -> Option<&str> {
        self.label(K8S_TOPOLOGY_REGION)
    }

    fn zone(&self) -> Option<&str> {
        self.label(K8S_TOPOLOGY_ZONE)
    }
}

impl<R> LabelsExt for R where R: ResourceExt {}

pub(super) fn group_nodes_by_region(
    nodes: impl IntoIterator<Item = Node>,
) -> HashMap<Option<String>, Vec<String>> {
    nodes
        .into_iter()
        .map(|node| (node.name(), node.region().map(ToString::to_string)))
        .fold(HashMap::new(), |mut regions, (name, region)| {
            regions.entry(region).or_default().push(name);
            regions
        })
}

pub(super) fn group_nodes_by_zone(
    nodes: impl IntoIterator<Item = Node>,
) -> HashMap<Option<String>, Vec<String>> {
    nodes
        .into_iter()
        .map(|node| (node.name(), node.zone().map(ToString::to_string)))
        .fold(HashMap::new(), |mut zones, (name, zone)| {
            zones.entry(zone).or_default().push(name);
            zones
        })
}

pub(super) fn is_aks(nodes: impl IntoIterator<Item = Node>) -> bool {
    nodes
        .into_iter()
        .filter_map(|node| node.label(AKS_CLUSTER).map(String::from))
        .any(|label| label.starts_with("MC_"))
}

pub(super) fn is_eks() -> bool {
    true
}
