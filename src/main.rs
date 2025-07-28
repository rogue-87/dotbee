mod cli;
mod commands;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Init { name } => commands::init::run(name)?,
        Commands::List {} => todo!(),
        Commands::Purge {} => todo!(),
        Commands::Repair {} => todo!(),
        Commands::Status {} => todo!(),
        Commands::Switch { host } => todo!(),
    }

    Ok(())
}
