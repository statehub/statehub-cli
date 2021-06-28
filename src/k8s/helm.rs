//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::io;
use std::process::{Command, ExitStatus};

use tokio::process::Command as AsyncCmd;

use crate::traits::Show;
use crate::v0;

#[derive(Debug)]
pub(crate) enum Helm {
    Skip {
        namespace: String,
        default_state: Option<String>,
    },
    Do {
        namespace: String,
        default_state: Option<String>,
    },
}

impl Helm {
    pub(crate) fn new(namespace: String, default_state: Option<String>, skip_helm: bool) -> Self {
        if skip_helm {
            Self::Skip {
                namespace,
                default_state,
            }
        } else {
            Self::Do {
                namespace,
                default_state,
            }
        }
    }

    pub(crate) fn skip(self) -> Self {
        match self {
            Self::Do {
                namespace,
                default_state,
            } => Self::Skip {
                namespace,
                default_state,
            },
            other => other,
        }
    }

    pub(crate) fn default_state(&self) -> Option<&str> {
        match self {
            Helm::Skip { default_state, .. } => default_state.as_deref(),
            Helm::Do { default_state, .. } => default_state.as_deref(),
        }
    }

    pub(crate) fn namespace(&self) -> &str {
        match self {
            Helm::Skip { namespace, .. } => namespace,
            Helm::Do { namespace, .. } => namespace,
        }
    }

    pub(crate) async fn execute(&self, cluster: &v0::Cluster) -> io::Result<(String, String)> {
        let commands = self.command(cluster);
        let (stdout, stderr) = match self {
            Helm::Skip { .. } => (String::new(), format!("Manually run\n{}", commands.show())),
            Helm::Do { .. } => {
                let mut success = String::new();
                let mut failure = String::new();
                for cmd in commands {
                    let input = cmd.show();
                    let (status, stdout, stderr) = self.exec(cmd).await?;
                    log::debug!("{}\n{}", input, stdout);
                    if status.success() {
                        success += &format!("{}\n{}\n", input, stdout);
                    } else {
                        failure += &format!("Running '{}' failed\n{}\n", input, stderr);
                    }
                }
                (success, failure)
            }
        };
        Ok((stdout, stderr))
    }

    pub(crate) fn command(&self, cluster: &v0::Cluster) -> Vec<Command> {
        cluster
            .helm
            .iter()
            .map(|helm| {
                let mut cmd = Command::new("helm");
                cmd.arg("install")
                    .arg(&helm.chart)
                    .arg("--namespace")
                    .arg(self.namespace())
                    .arg("--repo")
                    .arg(&helm.repo)
                    .arg("--version")
                    .arg(&helm.version)
                    .arg(&helm.chart);
                for (param, value) in &helm.parameters {
                    cmd.arg("--set").arg(format!("{}={}", param, value));
                }
                cmd
            })
            .collect()
    }

    async fn exec(&self, command: Command) -> io::Result<(ExitStatus, String, String)> {
        let output = AsyncCmd::from(command).output().await?;
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Ok((output.status, stdout, stderr))
    }
}
