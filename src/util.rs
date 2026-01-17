use crate::config::Profile;
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_config<'a>() -> Result<&'a Path, Box<dyn std::error::Error>> {
    let config_path = Path::new("dotsy.toml");
    return match config_path.exists() {
        true => Ok(config_path),
        false => Err(format!("{} {}", " ".red(), "dotsy.toml already exists in the current directory.").into()),
    };
}

pub fn expand_path(path_str: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if path_str.starts_with("~") {
        if let Some(home) = dirs::home_dir() {
            if path_str == "~" {
                return Ok(home);
            }
            // safely strip prefix
            return Ok(home.join(path_str.trim_start_matches("~/")));
        }
    }
    Ok(PathBuf::from(path_str))
}

pub fn is_profile_active(profile: &Profile, cwd: &Path) -> bool {
    if profile.links.is_empty() {
        return false;
    }

    for (target_str, source_str) in &profile.links {
        let target_path = match expand_path(target_str) {
            Ok(p) => p,
            Err(_) => return false,
        };
        let source_path = cwd.join(source_str);

        if !target_path.is_symlink() {
            return false;
        }

        match fs::read_link(&target_path) {
            Ok(p) => {
                if p != source_path {
                    return false;
                }
            }
            Err(_) => return false,
        }
    }
    true
}
