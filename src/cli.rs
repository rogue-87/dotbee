use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Dotsy", about = "Easy to use dotfiles manager", version, author)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: SubCommand,

    /// custom config file path
    #[clap(short, long, value_name = "FILE", global = true)]
    pub config: Option<String>,

    /// do not perform any actions, just print what would be done
    #[clap(long, global = true)]
    pub dry_run: bool,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// show currently used configs and symlinks status
    #[command(visible_alias = "dr")]
    Doctor,

    /// init dotsy
    #[command(visible_alias = "i")]
    Init,

    /// list all available configs
    #[command(visible_alias = "ls")]
    List,

    /// purge symlinks
    #[command(visible_alias = "p")]
    Purge,

    /// attempt to fix broken symlinks
    #[command(visible_alias = "r")]
    Repair,

    /// select profile
    #[command(visible_alias = "s")]
    Switch {
        /// profile to switch to
        profile: Option<String>,
    },
}
