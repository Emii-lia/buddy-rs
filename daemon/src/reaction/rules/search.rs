use shared::types::Event;
use shared::style::wrap_in_bubble;
use crate::reaction::traits::ReactionRule;

pub struct SearchRule;

impl ReactionRule for SearchRule {
  fn matches(&self, event: &Event) -> bool {
    let search_commands: &[&'static str] = &[
      "search",
      "find",
      "lookup",
      "grep",
      "ag",
      "ack",
      "rg",
      "fd",
      "locate"
    ];
    
    matches!(
      event,
      Event::Command(cmd) if search_commands.contains(&cmd.command.split_whitespace().next().unwrap())
    )
  }

  fn react(&self, _event: &Event) -> Option<String> {
    let msg = "The answer was apparently nearby the entire time.";
    let buddy = "(ง'̀-'́)ง";
    Some(wrap_in_bubble(msg, buddy))
  }
}