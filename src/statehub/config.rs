//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use serde::{Deserialize, Serialize};

const STATEHUB_HOME: &str = "STATEHUB_HOME";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    version: String,
    api: String,
    console: String,
    token: Option<String>,
}

impl Config {
    const CONFIG_FILE: &'static str = "config.toml";
    const VERSION: &'static str = "2";
    const DEFAULT_API: &'static str = "https://api.statehub.io";
    const DEFAULT_CONSOLE: &'static str = "https://console.statehub.io";

    pub(crate) fn api(&self) -> &str {
        self.api.as_str()
    }

    pub(crate) fn console(&self) -> &str {
        self.console.as_str()
    }

    pub(crate) fn optionally_management_api(self, api: Option<String>) -> Self {
        if let Some(api) = api {
            Self { api, ..self }
        } else {
            self
        }
    }

    pub(crate) fn optionally_management_console(self, console: Option<String>) -> Self {
        if let Some(console) = console {
            Self { console, ..self }
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
        anyhow::ensure!(path.exists(), "Config file does not exist");
        let config = fs::read_to_string(path)
            .context("Reading config file")
            .and_then(|text| toml::from_str::<toml::Value>(&text).context("Parsing config file"))?;

        Ok(Self::rolling_load(config))
    }

    fn rolling_load(config: toml::Value) -> Self {
        if let Ok(v1) = ConfigV1::validate_config(&config) {
            v1.into()
        } else if let Ok(v2) = ConfigV2::validate_config(&config) {
            v2.into()
        } else {
            Self::default()
        }
    }

    pub(crate) fn save(&self) -> anyhow::Result<PathBuf> {
        let dir = Self::statehub_home()?;
        fs::create_dir_all(dir).context("Creating config filesystem hierarchy")?;
        let path = Self::config_file()?;
        let contents = toml::to_string_pretty(self).context("Serializing config")?;
        fs::write(&path, contents)
            .context("Writing config")
            .map(|_| path)
    }

    fn statehub_home() -> anyhow::Result<PathBuf> {
        let env = env::var(STATEHUB_HOME).ok().map(PathBuf::from);
        let default = directories::UserDirs::new()
            .map(|user| user.home_dir().to_path_buf().join(".statehub"));

        env.or(default).context("Config directory name")
    }

    fn config_file() -> anyhow::Result<PathBuf> {
        Self::statehub_home().map(|dir| dir.join(Self::CONFIG_FILE))
    }
}

impl Default for Config {
    fn default() -> Self {
        let version = Self::VERSION.to_string();
        let api = Self::DEFAULT_API.to_string();
        let console = Self::DEFAULT_CONSOLE.to_string();
        let token = None;
        Self {
            version,
            api,
            console,
            token,
        }
    }
}

impl From<ConfigV2> for Config {
    fn from(v2: ConfigV2) -> Self {
        Self {
            version: v2.version,
            api: v2.api,
            console: v2.console,
            token: v2.token,
        }
    }
}

impl From<ConfigV1> for Config {
    fn from(v1: ConfigV1) -> Self {
        Self {
            version: String::from(Self::VERSION),
            api: v1.api,
            console: String::from(Self::DEFAULT_CONSOLE),
            token: v1.token,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct ConfigV2 {
    version: String,
    api: String,
    console: String,
    token: Option<String>,
}

impl ConfigV2 {
    const VERSION: &'static str = "2";

    fn validate_config(config: &toml::Value) -> anyhow::Result<Self> {
        let version = config.get("version").and_then(|version| version.as_str());
        anyhow::ensure!(version == Some(Self::VERSION), "Unknown version");
        let config = config.clone();
        toml::Value::try_into(config).context("Converting to ConfigV2")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct ConfigV1 {
    version: String,
    api: String,
    token: Option<String>,
}

impl ConfigV1 {
    const VERSION: &'static str = "1";

    fn validate_config(config: &toml::Value) -> anyhow::Result<Self> {
        let version = config.get("version").and_then(|version| version.as_str());
        anyhow::ensure!(version == Some(Self::VERSION), "Unknown version");
        let config = config.clone();
        toml::Value::try_into(config).context("Converting to ConfigV1")
    }
}
