//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    version: String,
    api: String,
    token: Option<String>,
}

impl Config {
    const FILENAME: &'static str = "config.toml";
    const VERSION: &'static str = "1";
    const DEFAULT_API: &'static str = "https://api.statehub.io";

    pub(crate) fn api(&self) -> &str {
        self.api.as_str()
    }

    pub(crate) fn optionally_management_api(self, api: Option<String>) -> Self {
        if let Some(api) = api {
            Self { api, ..self }
        } else {
            self
        }
    }
    pub(crate) fn set_token(self, token: Option<String>) -> Self {
        let token = token.or(self.token);
        Self { token, ..self }
    }

    // pub(crate) fn optionally_token<S, T>(self, token: T) -> Self
    // where
    //     for<'r> T: Into<Option<S>>,
    //     S: ToString,
    // {
    //     let token = token.into().map(|s| s.to_string());
    //     Self { token, ..self }
    // }

    pub(crate) fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub(crate) fn load() -> anyhow::Result<Self> {
        let path = Self::config_file()?;
        if path.exists() {
            fs::read(path)
                .context("Reading config file")
                .and_then(|bytes| toml::from_slice::<Self>(&bytes).context("Parsing config file"))
                .and_then(|config| config.validate_config())
        } else {
            Ok(Self::default())
        }
    }

    pub(crate) fn save(&self) -> anyhow::Result<PathBuf> {
        let dir = Self::config_dir()?;
        fs::create_dir_all(dir).context("Creating config filesystem hierarchy")?;
        let path = Self::config_file()?;
        let contents = toml::to_string_pretty(self).context("Serializing config")?;
        fs::write(&path, contents)
            .context("Writing config")
            .map(|_| path)
    }

    fn config_dir() -> anyhow::Result<PathBuf> {
        directories::ProjectDirs::from("io", "statehub", "statehub")
            .map(|project| project.config_dir().to_path_buf())
            .context("Config directory name")
    }

    fn config_file() -> anyhow::Result<PathBuf> {
        Self::config_dir().map(|dir| dir.join(Self::FILENAME))
    }

    fn validate_config(self) -> anyhow::Result<Self> {
        if self.version != Self::VERSION {
            anyhow::bail!(
                "Version mismatch; expecting {}, found {}",
                Self::VERSION,
                self.version
            )
        }
        Ok(self)
    }
}

impl Default for Config {
    fn default() -> Self {
        let version = Self::VERSION.to_string();
        let api = Self::DEFAULT_API.to_string();
        let token = None;
        Self {
            version,
            api,
            token,
        }
    }
}
