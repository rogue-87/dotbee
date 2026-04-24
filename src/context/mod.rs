pub mod manager;

use manager::Manager;
use std::error::Error;

pub struct Context {
    pub manager: Manager,
    pub dry_run: bool,
}

impl Context {
    pub fn new(path_to_config: Option<String>, dry_run: bool) -> Result<Self, Box<dyn Error>> {
        let manager = Manager::new(path_to_config)?;
        Ok(Context { manager, dry_run })
    }
}
