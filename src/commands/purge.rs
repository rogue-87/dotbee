use crate::config::Config;
use crate::util::{is_profile_active, unlink_profile_links};
use colored::Colorize;
use std::error::Error;
use std::path::Path;

pub fn run() -> Result<(), Box<dyn Error>> {
    let config_path = Path::new("dotsy.toml");
    if !config_path.exists() {
        return Err("dotsy.toml not found. Run 'dotsy init' first.".into());
    }

    let config = Config::load_from_path(config_path)?;
    let cwd = std::env::current_dir()?;

    println!("{}", "Purging active links...".bold().red());

    if let Some(global) = &config.global {
        println!("Unlinking global links...");
        unlink_profile_links(&global.links, &cwd)?;
    }

    if let Some(profiles) = &config.profiles {
        for (name, profile) in profiles {
            if is_profile_active(profile, &cwd) {
                println!("Unlinking profile '{}'...", name.yellow());
                unlink_profile_links(&profile.links, &cwd)?;
            }
        }
    }

    println!("{}", "Purge complete.".green());

    Ok(())
}
