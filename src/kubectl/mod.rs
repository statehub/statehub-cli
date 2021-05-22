//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::collections::HashMap;

use k8s_openapi::api::core::v1::{Node, Pod};
use kube::api::{Api, ListParams};
// use kube::api::{Api, ListParams, PostParams, Resource, WatchEvent};
use kube::Client;

use crate::Location;

use helper::{group_nodes_by_region, group_nodes_by_zone};

mod helper;

const DEFAULT_NS: &str = "default";
const KUBE_SYSTEM_NS: &str = "kube-system";

pub(crate) struct Kubectl {
    client: Client,
    namespace: String,
}

impl Kubectl {
    async fn with_namespace(namespace: &str) -> anyhow::Result<Self> {
        let client = Client::try_default().await?;
        let namespace = namespace.to_string();
        Ok(Self { client, namespace })
    }
    pub(crate) async fn default() -> anyhow::Result<Self> {
        Self::with_namespace(DEFAULT_NS).await
    }

    pub(crate) async fn kube_system() -> anyhow::Result<Self> {
        Self::with_namespace(KUBE_SYSTEM_NS).await
    }

    pub(crate) async fn list_nodes() -> anyhow::Result<()> {
        Self::default()
            .await?
            .all_nodes()
            .await?
            .into_iter()
            .for_each(|node| println!("{:#?}", node));
        Ok(())
    }

    pub(crate) async fn list_pods() -> anyhow::Result<()> {
        Self::kube_system()
            .await?
            .all_pods()
            .await?
            .into_iter()
            .for_each(|pod| println!("{:#?}", pod));
        Ok(())
    }

    async fn all_nodes(&self) -> anyhow::Result<impl IntoIterator<Item = Node>> {
        let nodes = self.nodes();
        let lp = ListParams::default();
        Ok(nodes.list(&lp).await?)
    }

    async fn all_pods(&self) -> anyhow::Result<impl IntoIterator<Item = Pod>> {
        let pods = self.pods();
        let lp = ListParams::default();
        Ok(pods.list(&lp).await?)
    }

    fn nodes(&self) -> Api<Node> {
        Api::all(self.client.clone())
    }

    fn pods(&self) -> Api<Pod> {
        Api::namespaced(self.client.clone(), &self.namespace)
    }
}

pub(crate) async fn get_regions(
    zone: bool,
) -> anyhow::Result<HashMap<Option<String>, Vec<String>>> {
    let group_nodes = |nodes| {
        if zone {
            group_nodes_by_zone(nodes)
        } else {
            group_nodes_by_region(nodes)
        }
    };

    Kubectl::default().await?.all_nodes().await.map(group_nodes)
}

pub(crate) async fn collect_node_locations() -> anyhow::Result<Vec<Location>> {
    get_regions(false)
        .await?
        .into_iter()
        .map(|(region, nodes)| {
            region.ok_or_else(|| anyhow::anyhow!("Cannot determine location for nodes {:?}", nodes))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|region| region.parse())
        .collect::<Result<Vec<Location>, _>>()
        .map_err(anyhow::Error::msg)
}

pub(crate) fn helm_is_found() -> bool {
    which::which("helm").is_ok()
}
