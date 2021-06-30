//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::borrow::Cow;
use std::collections::HashMap;

use k8s_openapi::api::core::v1::{ConfigMap, Namespace, Node, Pod, Secret};
use kube::api::{self, Api};
// use kube::api::{Api, ListParams, PostParams, Resource, WatchEvent};
use kube::config::Kubeconfig;
use kube::{Client, ResourceExt};
use serde_json as json;

use crate::v0;
use crate::Location;

pub(crate) use helm::Helm;
use helper::{group_nodes_by_region, group_nodes_by_zone, is_aks, is_eks};
use kubeconfig::KubeconfigExt;

mod helm;
mod helper;
mod kubeconfig;
mod show;

const DEFAULT_NS: &str = "default";
const KUBE_SYSTEM_NS: &str = "kube-system";
const STATEHUB_CLUSTER_TOKEN_SECRET_TYPE: &str = "statehub.io/cluster-token";
const STATEHUB_CLUSTER_TOKEN_SECRET_NAME: &str = "statehub-cluster-token";
const STATEHUB_CLUSTER_CONFIGMAP_NAME: &str = "statehub";
const STATEHUB_DEFAULT_CLEANUP_GRACE: &str = "600s";

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

    async fn create_configmap(
        &self,
        name: &str,
        cluster_name: &v0::ClusterName,
        default_state: &str,
        api: &str,
    ) -> anyhow::Result<ConfigMap> {
        let configmaps = self.configmaps();
        let configmap = json::from_value(json::json!({
            "apiVerion": "v1",
            "kind": "ConfigMap",
            "metadata": {
                "name": name,
                "namespace": self.namespace,
            },
            "data": {
                "cluster-name": cluster_name,
                "default-state": default_state,
                "api-url": api,
                "cleanup-grace": STATEHUB_DEFAULT_CLEANUP_GRACE,
            }
        }))?;
        let pp = self.post_params();
        let configmap = configmaps.create(&pp, &configmap).await?;

        Ok(configmap)
    }

    async fn create_secret(
        &self,
        r#type: &str,
        name: &str,
        secret: &str,
    ) -> anyhow::Result<Secret> {
        let secrets = self.secrets();
        let secret = json::from_value(json::json!({
            "apiVerion": "v1",
            "kind": "Secret",
            "metadata": {
                "name": name,
            },
            "type": r#type,
            "data": {
                "cluster-token": base64::encode(secret),
            }
        }))?;
        let pp = self.post_params();
        let secret = secrets.create(&pp, &secret).await?;
        Ok(secret)
    }

    async fn delete_secret(&self, secret: &str) -> anyhow::Result<()> {
        log::info!("Deleting secret {}", secret);
        let secrets = self.secrets();
        let dp = self.delete_params();
        secrets
            .delete(secret, &dp)
            .await?
            .map_left(|secret| log::info!("Delete in progress {:#?}", secret))
            .map_right(|status| log::info!("Delete succeeded: {:#?}", status));

        Ok(())
    }

    async fn delete_configmap(&self, configmap: &str) -> anyhow::Result<()> {
        log::info!("Deleting configmap {}", configmap);
        let configmaps = self.configmaps();
        let dp = self.delete_params();
        configmaps
            .delete(configmap, &dp)
            .await?
            .map_left(|cm| log::info!("Delete in progress {:#?}", cm))
            .map_right(|cm| log::info!("Delete succeeded: {:#?}", cm));

        Ok(())
    }

    fn delete_params(&self) -> api::DeleteParams {
        api::DeleteParams::default()
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

    fn configmaps(&self) -> Api<ConfigMap> {
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

pub(crate) async fn store_configmap(
    namespace: &str,
    cluster_name: &v0::ClusterName,
    default_state: &str,
    api: &str,
) -> anyhow::Result<ConfigMap> {
    let kube = Kubectl::with_namespace(namespace).await?;

    if kube
        .delete_configmap(STATEHUB_CLUSTER_CONFIGMAP_NAME)
        .await
        .is_ok()
    {
        log::trace!("Removing previous configmap");
    }

    kube.create_configmap(
        STATEHUB_CLUSTER_CONFIGMAP_NAME,
        cluster_name,
        default_state,
        api,
    )
    .await
}

pub(crate) async fn store_cluster_token(namespace: &str, token: &str) -> anyhow::Result<Secret> {
    let kube = Kubectl::with_namespace(namespace).await?;

    if kube
        .delete_secret(STATEHUB_CLUSTER_TOKEN_SECRET_NAME)
        .await
        .is_ok()
    {
        log::trace!("Removing previous cluster token");
    }

    kube.create_secret(
        STATEHUB_CLUSTER_TOKEN_SECRET_TYPE,
        STATEHUB_CLUSTER_TOKEN_SECRET_NAME,
        token,
    )
    .await
}

pub(crate) fn helm_is_found() -> bool {
    which::which("helm").is_ok()
}

pub(crate) fn extract_cluster_token(secret: &Secret) -> Option<Cow<'_, str>> {
    secret
        .data
        .get("cluster-token")
        .map(|bytes| bytes.0.as_slice())
        .map(String::from_utf8_lossy)
}

// TODO: implement provider fetch out of cluster
pub(crate) async fn get_cluster_provider(
    _cluster: &v0::ClusterName,
) -> anyhow::Result<v0::Provider> {
    let kube = Kubectl::default().await?;
    let nodes = kube.all_nodes().await?;
    if is_aks(nodes) {
        Ok(v0::Provider::Aks)
    } else if is_eks() {
        Ok(v0::Provider::Eks)
    } else {
        Ok(v0::Provider::Generic)
    }
}

pub(crate) fn get_current_cluster_name() -> Option<v0::ClusterName> {
    Kubeconfig::read()
        .ok()?
        .default_context()
        .map(kubeconfig::normalize_name)
        .map(v0::ClusterName::from)
}
