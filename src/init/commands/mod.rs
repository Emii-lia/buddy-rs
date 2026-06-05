use clap::{Parser, Subcommand};

pub mod install;
pub mod uninstall;
pub mod explain;

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
    #[command(
        name = "explain",
        about = "Gives a brief explanation of the command",
    )]
    Explain{
        command: String,
        #[arg(short, long, required = false, default_value = "buddy", help = "Specify the assistant to use for explanation (default: buddy)")]
        assistant: Option<String>,
    },
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