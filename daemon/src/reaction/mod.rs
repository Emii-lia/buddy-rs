use colored::*;
use shared::types::{CommandEvent, Event};
use crate::reaction::traits::ReactionRule;

pub mod traits;

pub struct BuildRule;
pub struct FailedCommandRule;

fn style_icon(icon: &str, color: Color) -> String {
    icon.color(color).bold().to_string()
}

fn style_message(msg: &str) -> String {
    msg.italic().bright_white().to_string()
}

fn wrap_in_bubble(msg: &str, buddy: &str) -> String {
    let lines: Vec<&str> = msg.split('\n').collect();
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let width = max_len + 2;

    let mut bubble = String::new();
    bubble.push_str(&format!("  {}\n", "─".repeat(width).bright_white()));
    for line in lines {
        let padding = " ".repeat(width - line.len() - 1);
        bubble.push_str(&format!("  {} {} {} {}\n", 
            "│".bright_white(), 
            line.italic().bright_white(), 
            padding.bright_white(),
            "│".bright_white()
        ));
    }
    bubble.push_str(&format!("  {}\n", "─".repeat(width).bright_white()));
    bubble.push_str(&format!(" {} \n", buddy.bold()));
    bubble
}

impl ReactionRule for BuildRule {
  fn matches(&self, event: &Event) -> bool {
    matches!(event, Event::Command(cmd) if cmd.command.contains("build"))
  }

  fn react(&self, event: &Event) -> Option<String> {
    let Event::Command(cmd) = event;
    if cmd.duration_ms > 10000 {
      let msg = "Finally. After several identity crises, it compiled itself into submission.";
      let buddy = format!("{} ( -_-)", style_icon("󱇬", Color::BrightYellow));
      Some(wrap_in_bubble(msg, &buddy))
    } else {
      let msg = "Done. Suspiciously quick. No complaints yet, which is worrying";
      let buddy = format!("{} (•‿•)", style_icon("󰄬", Color::BrightGreen));
      Some(wrap_in_bubble(msg, &buddy))
    }
  }
}

impl ReactionRule for FailedCommandRule {
  fn matches(&self, event: &Event) -> bool {
    matches!(event, Event::Command(cmd) if cmd.exit_code != 0)
  }
  fn react(&self, event: &Event) -> Option<String> {
    let msg = "It broke. As expected. Moving on emotionally.";
    let buddy = format!("{} (╯°□°)╯", style_icon("", Color::BrightRed));
    Some(wrap_in_bubble(msg, &buddy))
  }
}

pub fn react_to_command(command_event: CommandEvent) -> Vec<String> {
  let mut responses = Vec::new();
  let event = Event::Command(command_event);
  let build_rule = BuildRule;
  if build_rule.matches(&event) {
    if let Some(response) = build_rule.react(&event) {
      responses.push(response);
    }
  }
  let failed_command_rule = FailedCommandRule;
  if failed_command_rule.matches(&event) {
    if let Some(response) = failed_command_rule.react(&event) {
      responses.push(response);
    }
  }
  responses
}