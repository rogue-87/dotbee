use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Dotsy")]
#[command(about = "Easy to use dotfiles manager", version, author)]
pub struct Cli {
    /// custom config file path
    #[clap(short, long, value_name = "FILE", global = true)]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// show currently used configs and symlinks status
    Doctor {},

    /// init dotsy :3
    Init {
        /// do not perform any actions
        #[clap(long)]
        dry_run: bool,
    },

    /// list all available configs
    List {},

    /// purge symlinks
    Purge {
        /// do not perform any actions
        #[clap(long)]
        dry_run: bool,
    },

    /// attempt to fix broken symlinks
    Repair {
        /// do not perform any actions
        #[clap(long)]
        dry_run: bool,
    },

    /// select profile
    Switch {
        /// profile to switch to
        profile: String,

        /// do not perform any actions
        #[clap(long)]
        dry_run: bool,
    },
}
}
