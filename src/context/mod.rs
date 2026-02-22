pub mod manager;
pub mod message;

use manager::Manager;
use message::Message;
use std::error::Error;

pub struct Context {
    pub manager: Manager,
    pub message: Message,
    pub dry_run: bool,
}

impl Context {
    pub fn new(path_to_config: Option<String>, dry_run: bool) -> Result<Self, Box<dyn Error>> {
        let manager = Manager::new(path_to_config)?;

        let icon_style = manager.config.get_settings().icon_style.unwrap_or_default();
        let message = Message::new(icon_style);

        Ok(Context { manager, message, dry_run })
    }
}
