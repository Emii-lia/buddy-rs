use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandEvent {
  pub command: String,
  pub exit_code: i32,
  pub duration_ms: u128,
  pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
  Command(CommandEvent),
}

pub enum CommandType {
  Build,
  Fail,
  Unknown,
}

impl CommandType {
  pub fn classify(command: &CommandEvent) -> Self {
    if command.exit_code != 0 {
      CommandType::Fail
    } else if command.command.contains("build") {
      CommandType::Build
    } else {
      CommandType::Unknown
    }
  }
}