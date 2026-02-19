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

        // Get the parent directory of the loaded config file (if any)
        if let Some(new_dotfiles_path) = config
            .path
            .as_ref()
            .and_then(|p| p.parent())
            // Only proceed if this path is DIFFERENT from the current state
            .filter(|p| state.dotfiles_path.as_deref() != Some(p))
        {
            state.dotfiles_path = Some(new_dotfiles_path.to_path_buf());
            state.save()?;
        } else if config.path.is_none() && state.dotfiles_path.is_some() {
            // If no config path was loaded but state has one, it means the state is stale.
            state.dotfiles_path = None;
            state.save()?;
        }

        Ok(Self { symlink, state, config })
    }
}
