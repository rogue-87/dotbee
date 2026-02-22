pub mod conflict;
pub mod icons;

use self::icons::IconStyle;
pub use conflict::ConflictAction;
use indexmap::IndexMap;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(default)]
struct Config {
    #[serde(skip)]
    path: Option<PathBuf>,
    settings: Settings,
    global: Option<Global>,
    profiles: Option<IndexMap<String, Profile>>,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(default)]
pub struct Settings {
    #[serde(default, deserialize_with = "conflict::deserialize_conflict_action")]
    pub on_conflict: Option<ConflictAction>,
    pub icon_style: Option<IconStyle>,
    pub auto_detect_profile: Option<bool>,
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
}

impl ConfigManager {
    pub fn load(path: Option<String>) -> Result<Self, Box<dyn Error>> {
        let path_str = path.unwrap_or_else(|| "dotsy.toml".to_string());
        let config_path = Path::new(&path_str);

        if !config_path.exists() {
            return Ok(Self { config: Config::default() });
        }

        let content = fs::read_to_string(config_path)?;
        let mut config: Config = toml::from_str(&content)?;
        config.path = Some(fs::canonicalize(config_path)?);

        Ok(Self { config })
    }

    pub fn get_profile(&self, name: &str) -> Result<&Profile, Box<dyn Error>> {
        let profiles = self.config.profiles.as_ref().ok_or("No profiles defined in configuration.")?;
        profiles
            .get(name)
            .ok_or_else(|| format!("Profile '{}' not found in configuration.", name).into())
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
        self.config.path.as_deref()
    }
}
