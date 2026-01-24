use crate::config::Config;
use crate::util::{
    expand_path, get_destination_status, is_profile_active, symlink_with_parents, DestinationStatus,
};
use colored::Colorize;
use std::error::Error;
use std::path::Path;

pub fn run() -> Result<(), Box<dyn Error>> {
    let config = Config::load(None)?;
    let cwd = std::env::current_dir()?;

    println!("{}", "Repairing symlinks...".bold().blue());

    if let Some(global) = &config.global {
        println!("Checking global links...");
        repair_links(&global.links, &cwd)?;
    }

    if let Some(profiles) = &config.profiles {
        let mut active_found = false;
        for (name, profile) in profiles {
            if is_profile_active(profile, &cwd) {
                println!("Checking profile '{}'...", name.green());
                repair_links(&profile.links, &cwd)?;
                active_found = true;
            }
        }
        if !active_found {
             println!("No active profile detected. Only global links were checked.");
        }
    }

    println!("{}", "Repair complete.".green());
    Ok(())
}

fn repair_links(links: &std::collections::HashMap<String, String>, cwd: &Path) -> Result<(), Box<dyn Error>> {
    for (target_str, source_str) in links {
        let source_path = cwd.join(source_str);
        let target_path = expand_path(target_str)?;

        if !source_path.exists() {
            println!("  {} Source missing: {}", " ".yellow(), source_path.display());
            continue;
        }

        let status = get_destination_status(&source_path, &target_path)?;

        match status {
            DestinationStatus::AlreadyLinked => {
                // All good, do nothing
            }
            DestinationStatus::NonExistent => {
                println!("  {} Linking {} -> {}", " ".green(), source_str, target_str);
                symlink_with_parents(&source_path, &target_path)?;
            }
            DestinationStatus::ConflictingSymlink => {
                println!("  {} Relinking {} -> {}", " ".green(), source_str, target_str);
                // Safe to remove incorrect symlink and replace
                if target_path.exists() || target_path.is_symlink() {
                    std::fs::remove_file(&target_path)?;
                }
                symlink_with_parents(&source_path, &target_path)?;
            }
            DestinationStatus::ConflictingFileOrDir => {
                println!("  {} Conflict at {} (File/Dir exists). Manual intervention required.", " ".red(), target_str);
            }
        }
    }
    Ok(())
}
