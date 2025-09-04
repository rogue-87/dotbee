use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs,
    io::{ErrorKind, Result},
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct State {
    pub active_host: Option<String>,
    pub symlinks: HashSet<PathBuf>,
}

impl State {
    /// return the path to the state file, `e.g., ~/.local/state/dotsy.json`
    pub fn get_state_file() -> Result<PathBuf> {
        let base_dir = dirs::data_local_dir().ok_or_else(|| {
            std::io::Error::new(ErrorKind::NotFound, "Could not find local data directory")
        })?;
        let state_dir = base_dir.join("dotsy");
        fs::create_dir_all(&state_dir)?;
        Ok(state_dir.join("state.json"))
    }

    /// load the state from the file or return default if missing
    pub fn load() -> Result<Self> {
        let path = Self::get_state_file()?;
        match fs::read_to_string(&path) {
            Ok(contents) => {
                let state = serde_json::from_str(&contents)?;
                Ok(state)
            }
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(Self::default()),
            Err(e) => Err(e),
        }
    }

    /// save the current state to disk
    pub fn save(&self) -> Result<()> {
        let path = Self::get_state_file()?;
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// add a symlink path to the state
    pub fn add_symlink<P: Into<PathBuf>>(&mut self, path: P) {
        let p = path.into();
        if !self.symlinks.contains(&p) {
            self.symlinks.insert(p);
        }
    }

    /// remove a symlink path from the state
    pub fn remove_symlink<P: AsRef<Path>>(&mut self, path: P) {
        self.symlinks.retain(|p| p != path.as_ref());
    }

    /// clear all state
    pub fn clear(&mut self) {
        self.active_host = None;
        self.symlinks.clear();
    }
}
