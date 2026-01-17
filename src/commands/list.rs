use std::{error::Error, path::Path};

use crate::config::Config;

pub fn run() -> Result<(), Box<dyn Error>> {
    let config_path = Path::new("dotsy.toml");
    let config = Config::load_from_path(config_path).unwrap();

    match config.profiles {
        Some(profiles) => {
            dbg!(profiles);
        }
        _ => println!("No profiles defined in config."),
    }

    return Ok(());
}
