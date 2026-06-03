use clap::Parser;
use crate::init::commands::{Cli, Command};
use crate::init::commands::install::install;
use crate::init::commands::uninstall::uninstall;

pub mod shell;
pub mod config;
pub mod commands;

pub fn run_init() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Install => {
            install().unwrap_or_else(|e| {
                eprintln!("Error during installation: {}", e);
                std::process::exit(1);
            });
        }
        Command::Uninstall => {
            uninstall().unwrap_or_else(|e| {
                eprintln!("Error during uninstallation: {}", e);
                std::process::exit(1);
            });
        }
    }

    Ok(())
}

fn main() {}