pub mod test;

use crate::daemon::reaction::traits::ReactionRule;
use crate::shared::style::wrap_in_bubble;
use crate::shared::types::Event;

pub struct SudoRule;

impl ReactionRule for SudoRule {
  fn matches(&self, event: &Event) -> bool {
    let sudos: &[&str] = &["sudo", "please"];
    matches!(
      event,
      Event::Command(cmd) if sudos.contains(&cmd.command.split_whitespace().next().unwrap())
    )
  }

  fn react(&self, _event: &Event) -> Option<String> {
    let msg = "Root has entered the chat. Everyone is nervous.";
    let buddy = "(⚆_⚆)";
    // let buddy = "(¬_◔)";
    Some(wrap_in_bubble(msg, buddy))
  }
}