use shared::types::{CommandEvent, Event};
use crate::reaction::traits::ReactionRule;

pub mod traits;

pub struct BuildRule;
pub struct FailedCommandRule;

impl ReactionRule for BuildRule {
  fn matches(&self, event: &Event) -> bool {
    matches!(event, Event::Command(cmd) if cmd.command.contains("build"))
  }

  fn react(&self, event: &Event) -> Option<String> {
    let Event::Command(cmd) = event;
    if cmd.duration_ms > 10000 {
      Some("Finally. After several identity crises, it compiled itself into submission.".to_string())
    } else {
      Some("Done. Suspiciously quick. No complaints yet, which is worrying".to_string())
    }
  }
}

impl ReactionRule for FailedCommandRule {
  fn matches(&self, event: &Event) -> bool {
    matches!(event, Event::Command(cmd) if cmd.exit_code != 0)
  }
  fn react(&self, event: &Event) -> Option<String> {
    Some("It broke. As expected. Moving on emotionally.".to_string())
  }
}

pub fn react_to_command(command_event: CommandEvent) {
  let event = Event::Command(command_event);
  let build_rule = BuildRule;
  if build_rule.matches(&event) {
    if let Some(response) = build_rule.react(&event) {
      println!("{}", response);
    }
  }
  let failed_command_rule = FailedCommandRule;
  if failed_command_rule.matches(&event) {
    if let Some(response) = failed_command_rule.react(&event) {
      println!("{}", response);
    }
  }
  println!("Command: {}, Exit Code: {}, Duration: {}ms",
    match &event {
      Event::Command(cmd) => &cmd.command,
    },
    match &event {
      Event::Command(cmd) => cmd.exit_code,
    },
    match &event {
      Event::Command(cmd) => cmd.duration_ms,
    }
  );
}