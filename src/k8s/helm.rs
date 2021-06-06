//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::process::Command;
use tokio::process::Command as AsyncCmd;

use crate::show::Show;
use crate::v1;

#[derive(Debug)]
pub(crate) enum Helm {
    Skip {
        namespace: String,
        default_storage_class: Option<String>,
    },
    Do {
        namespace: String,
        default_storage_class: Option<String>,
    },
}

impl Helm {
    pub(crate) fn new(
        namespace: String,
        default_storage_class: Option<String>,
        skip_helm: bool,
    ) -> Self {
        if skip_helm {
            Self::Skip {
                namespace,
                default_storage_class,
            }
        } else {
            Self::Do {
                namespace,
                default_storage_class,
            }
        }
    }

    pub(crate) fn default_storage_class(&self) -> Option<&str> {
        match self {
            Helm::Skip {
                namespace: _,
                default_storage_class,
            } => default_storage_class.as_deref(),
            Helm::Do {
                namespace: _,
                default_storage_class,
            } => default_storage_class.as_deref(),
        }
    }

    pub(crate) fn namespace(&self) -> &str {
        match self {
            Helm::Skip { namespace, .. } => namespace,
            Helm::Do { namespace, .. } => namespace,
        }
    }

    pub(crate) async fn execute(&self, cluster: &v1::Cluster, verbose: bool) -> anyhow::Result<()> {
        let commands = self.command(cluster);
        match self {
            Helm::Skip { .. } => println!("Manually run\n{}", commands.show()),
            Helm::Do { .. } => {
                for cmd in commands {
                    let output = AsyncCmd::from(cmd).output().await?;
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if verbose {
                        println!("{}", stdout);
                    }
                }
            }
        };
        Ok(())
    }

    fn command(&self, cluster: &v1::Cluster) -> Vec<Command> {
        let (namespace, default_storage_class) = match self {
            Helm::Skip {
                namespace,
                default_storage_class,
            } => (namespace, default_storage_class),
            Helm::Do {
                namespace,
                default_storage_class,
            } => (namespace, default_storage_class),
        };
        cluster
            .helm
            .iter()
            .map(|helm| {
                let mut cmd = Command::new("helm");
                cmd.arg("install")
                    .arg("--namespace")
                    .arg(namespace)
                    .arg("--repo")
                    .arg(&helm.repo)
                    .arg("--version")
                    .arg(&helm.version)
                    .arg(&helm.chart);
                for (param, value) in &helm.parameters {
                    cmd.arg("--set").arg(format!("{}={}", param, value));
                }
                if let Some(default_storage_class) = default_storage_class {
                    cmd.arg("--set").arg(format!(
                        "cluster.default_storage_class={}",
                        default_storage_class
                    ));
                }
                cmd.arg("--set")
                    .arg(format!("cluster.name={}", cluster.name));
                cmd
            })
            .collect()
    }
}
