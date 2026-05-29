use clap::{Parser, Subcommand};

pub mod install;
pub mod uninstall;

#[derive(Subcommand)]
pub enum Command {
    #[command(
        name = "install",
        alias = "init",
        about = "Installs and configures buddy",
        long_about = "Installs and configures buddy by detecting the user's shell, creating necessary configuration files, and setting up the environment for command tracking."
    )]
    Install,
    #[command(
        name = "uninstall",
        about = "Uninstalls buddy",
        long_about = "Uninstalls buddy by removing the configuration files."
    )]
    Uninstall,
}

#[derive(Parser)]
#[command(
    name = "buddy",
    about = "Buddy is a command-line tool that observes and comments on your commands.",
    version,
    author = "Emii-lia"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command
}