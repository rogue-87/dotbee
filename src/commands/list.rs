use crate::config::{Config, Profile};
use crate::util::expand_path;
use colored::Colorize;
use std::{error::Error, fs, path::Path};

pub fn run() -> Result<(), Box<dyn Error>> {
    let config_path = Path::new("dotsy.toml");
    let config = Config::load_from_path(config_path).unwrap();
    let cwd = std::env::current_dir()?;

    if let Some(profiles) = config.profiles {
        for (name, profile) in profiles {
            let active = is_profile_active(&profile, &cwd);
            let title = if active {
                format!("{} (active)", name).green().bold()
            } else {
                name.bold()
            };

            println!("{}", title);

            let mut links: Vec<_> = profile.links.iter().collect();
            links.sort_by_key(|(k, _)| k.as_str());

            for (i, (target, source)) in links.iter().enumerate() {
                let is_last = i == links.len() - 1;
                let branch = if is_last { "└──" } else { "├──" };
                println!("{} {} -> {}", branch, target, source);
            }
        }
    } else {
        println!("No profiles defined in config.");
    }

    Ok(())
}

fn is_profile_active(profile: &Profile, cwd: &Path) -> bool {
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
