use colored::Colorize;
use std::error::Error;
use std::fs;
use std::path::Path;

const DEFAULT_CONFIG: &str = include_str!("../config/dotsy.toml");

pub fn run() -> Result<(), Box<dyn Error>> {
    let config_path = Path::new("dotsy.toml");

    if config_path.exists() {
        println!("{} {}", " ".red(), "dotsy.toml already exists in the current directory.");
        return Ok(());
    }

    fs::write(config_path, DEFAULT_CONFIG)?;

    println!("{} {}", " ".green(), "Successfully initialized dotsy.toml");
    println!(
        "Edit the file to configure your dotfiles, then run {} to apply.",
        "dotsy switch <profile>".yellow()
    );

    Ok(())
}
