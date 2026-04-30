pub mod conflict;

use anyhow::anyhow;
pub use conflict::ConflictAction;
use indexmap::IndexMap;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(default)]
struct Config {
    settings: Settings,
    global: Option<Global>,
    profiles: Option<IndexMap<String, Profile>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Settings {
    #[serde(default, deserialize_with = "conflict::deserialize_conflict_action")]
    pub on_conflict: Option<ConflictAction>,
    pub auto_detect_profile: Option<bool>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            on_conflict: None,
            auto_detect_profile: Some(false),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Global {
    pub links: IndexMap<String, String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Profile {
    pub links: IndexMap<String, String>,
}

pub struct ConfigManager {
    config: Config,
    config_path: Option<PathBuf>,
}

impl ConfigManager {
    pub fn load(path: Option<String>) -> anyhow::Result<Self, anyhow::Error> {
        let path_str = path.unwrap_or_else(|| "dotbee.toml".to_string());
        let config_path = Path::new(&path_str);

        if !config_path.exists() {
            return Ok(Self {
                config: Config::default(),
                config_path: None,
            });
        }

        let content = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&content)?;
        let config_path = Some(fs::canonicalize(config_path)?);

        Ok(Self { config, config_path })
    }

    pub fn get_profile(&self, name: &str) -> anyhow::Result<&Profile, anyhow::Error> {
        let profiles = self
            .config
            .profiles
            .as_ref()
            .ok_or(anyhow!("No profiles defined in configuration."))?;
        profiles.get(name).ok_or(anyhow!("Profile '{}' not found in configuration.", name))
    }

    pub fn list_profiles(&self) -> Vec<&str> {
        self.config
            .profiles
            .as_ref()
            .map(|p| p.keys().map(|k| k.as_str()).collect())
            .unwrap_or_default()
    }

    pub fn has_profiles(&self) -> bool {
        self.config.profiles.as_ref().map(|p| !p.is_empty()).unwrap_or(false)
    }

    pub fn get_global_links(&self) -> Option<&IndexMap<String, String>> {
        self.config.global.as_ref().map(|g| &g.links)
    }

    pub fn get_settings(&self) -> &Settings {
        &self.config.settings
    }

    pub fn get_config_path(&self) -> Option<&Path> {
        self.config_path.as_deref()
    }
}
