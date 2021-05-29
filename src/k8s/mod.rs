//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::collections::HashMap;

use k8s_openapi::api::core::v1::{Namespace, Node, Pod, Secret};
use kube::api::{self, Api};
// use kube::api::{Api, ListParams, PostParams, Resource, WatchEvent};
use kube::{Client, ResourceExt};
use serde_json as json;

use crate::Location;

pub(crate) use helm::Helm;
use helper::{group_nodes_by_region, group_nodes_by_zone};

mod helm;
mod helper;
mod show;

const DEFAULT_NS: &str = "default";
const KUBE_SYSTEM_NS: &str = "kube-system";
const STATEHUB_CLUSTER_TOKEN_SECRET_TYPE: &str = "statehub.io/cluster-token";

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

    async fn all_namespaces(&self) -> anyhow::Result<impl IntoIterator<Item = Namespace>> {
        let namespaces = self.namespaces();
        let lp = self.list_params();
        Ok(namespaces.list(&lp).await?)
    }

    async fn all_nodes(&self) -> anyhow::Result<impl IntoIterator<Item = Node>> {
        let nodes = self.nodes();
        let lp = self.list_params();
        Ok(nodes.list(&lp).await?)
    }

    async fn all_pods(&self) -> anyhow::Result<impl IntoIterator<Item = Pod>> {
        let pods = self.pods();
        let lp = self.list_params();
        Ok(pods.list(&lp).await?)
    }

    async fn create_namespace(&self, namespace: &str) -> anyhow::Result<Namespace> {
        let namespaces = self.namespaces();
        let namespace = json::from_value(json::json!({
            "apiVerion": "v1",
            "kind": "Namespace",
            "metadata": {
                "name": namespace,
            }
        }))?;
        let pp = self.post_params();
        let namespace = namespaces.create(&pp, &namespace).await?;
        Ok(namespace)
    }

    async fn create_secret(&self, r#type: &str, secret: &str) -> anyhow::Result<Secret> {
        let secrets = self.secrets();
        let secret = json::from_value(json::json!({
            "apiVerion": "v1",
            "kind": "Secret",
            "metadata": {
                "name": "statehub-cluster-token",
            },
            "type": r#type,
            "data": {
                "token": base64::encode(secret),
            }
        }))?;
        let pp = self.post_params();
        let secret = secrets.create(&pp, &secret).await?;
        Ok(secret)
    }

    fn list_params(&self) -> api::ListParams {
        api::ListParams::default()
    }

    fn post_params(&self) -> api::PostParams {
        api::PostParams::default()
    }

    fn namespaces(&self) -> Api<Namespace> {
        Api::all(self.client.clone())
    }

    fn nodes(&self) -> Api<Node> {
        Api::all(self.client.clone())
    }

    fn pods(&self) -> Api<Pod> {
        Api::namespaced(self.client.clone(), &self.namespace)
    }

    fn secrets(&self) -> Api<Secret> {
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

pub(crate) async fn validate_namespace(namespace: impl AsRef<str>) -> anyhow::Result<Namespace> {
    let namespace = namespace.as_ref();
    let kube = Kubectl::kube_system().await?;
    let existing = kube
        .all_namespaces()
        .await?
        .into_iter()
        .find(|ns| ns.name() == namespace);

    if let Some(existing) = existing {
        log::info!("Using existing namespace {}", existing.name());
        Ok(existing)
    } else {
        log::info!("Creating new namespace {}", namespace);
        kube.create_namespace(namespace).await
    }
}

pub(crate) async fn list_namespaces() -> anyhow::Result<impl IntoIterator<Item = Namespace>> {
    Kubectl::kube_system().await?.all_namespaces().await
}

pub(crate) async fn list_nodes() -> anyhow::Result<impl IntoIterator<Item = Node>> {
    Kubectl::default().await?.all_nodes().await
}

pub(crate) async fn list_pods() -> anyhow::Result<impl IntoIterator<Item = Pod>> {
    Kubectl::kube_system().await?.all_pods().await
}

pub(crate) async fn store_cluster_token(namespace: &str, token: &str) -> anyhow::Result<Secret> {
    Kubectl::with_namespace(namespace)
        .await?
        .create_secret(STATEHUB_CLUSTER_TOKEN_SECRET_TYPE, token)
        .await
}

pub(crate) fn helm_is_found() -> bool {
    which::which("helm").is_ok()
}
