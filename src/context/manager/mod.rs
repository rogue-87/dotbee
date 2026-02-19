pub mod config;
pub mod state;
pub mod symlink;

use config::Config;
use state::State;
use std::error::Error;
pub use symlink::*;

/// A part of the context singleton. Responsible for managing the following:
/// - Symlinks
/// - State file
/// - Config file
pub struct Manager {
    pub symlink: Symlink,
    pub state: State,
    pub config: Config,
}

impl Manager {
    pub fn new(path_to_config: Option<String>) -> Result<Self, Box<dyn Error>> {
        let symlink = Symlink::new();
        let mut state = State::load()?;

        // Determine effective config path
        let effective_config_path = match path_to_config.as_ref() {
            Some(p) => Some(p.clone()),
            None => state
                .dotfiles_path
                .as_ref()
                .map(|p| p.join("dotsy.toml").to_string_lossy().to_string()),
        };

        let config = Config::load(effective_config_path)?;

        // If a config was loaded from a file, update dotfiles_path in state
        if let Some(path) = &config.path {
            if let Some(parent) = path.parent() {
                let dotfiles_path = parent.to_path_buf();
                if state.dotfiles_path.as_ref() != Some(&dotfiles_path) {
                    state.dotfiles_path = Some(dotfiles_path);
                    state.save()?;
                }
            }
        }

        Ok(Self { symlink, state, config })
    }
}
