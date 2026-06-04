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

impl Event {
  pub fn new_command(command: String) -> Self {
    Event::Command(CommandEvent {
      command,
      exit_code: 0,
      duration_ms: 0,
      timestamp: 0,
    })
  }
  pub fn with_exit_code(self, exit_code: i32) -> Self {
    match self {
      Event::Command(cmd) => Event::Command(CommandEvent {
        command: cmd.command,
        exit_code,
        duration_ms: cmd.duration_ms,
        timestamp: cmd.timestamp,
      }),
    }
  }
  pub fn with_duration_ms(self, duration_ms: u128) -> Self {
    match self { Event::Command(cmd) =>
      Event::Command(CommandEvent {
        command: cmd.command,
        exit_code: cmd.exit_code,
        duration_ms,
        timestamp: cmd.timestamp,
      })
    }
  }
  pub fn with_timestamp(self, timestamp: u64) -> Self {
    match self { Event::Command(cmd) =>
      Event::Command(CommandEvent {
        command: cmd.command,
        exit_code: cmd.exit_code,
        duration_ms: cmd.duration_ms,
        timestamp,
      })
    }
  }
}