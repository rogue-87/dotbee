mod cli;
mod commands;
mod config;
mod util;
use clap::Parser;
use cli::{Cli, Command};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match args.command {
        Command::Doctor {} => commands::doctor::run(args.config)?,
        Command::Init { dry_run } => commands::init::run(args.config, dry_run)?,
        Command::List {} => commands::list::run(args.config)?,
        Command::Purge { dry_run } => commands::purge::run(args.config, dry_run)?,
        Command::Repair { dry_run } => commands::repair::run(args.config, dry_run)?,
        Command::Switch { profile, dry_run } => commands::switch::run(profile, args.config, dry_run)?,
    }

    Ok(())
}
