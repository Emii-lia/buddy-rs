pub mod test;

use crate::shared::types::Event;
use crate::shared::style::wrap_in_bubble;
use crate::daemon::reaction::traits::ReactionRule;

pub struct SearchRule;

impl ReactionRule for SearchRule {
  fn matches(&self, event: &Event) -> bool {
    let search_commands: &[&'static str] = &[
      "find",
      "lookup",
      "grep",
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