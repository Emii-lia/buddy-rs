use shared::types::Event;
use crate::reaction::style::wrap_in_bubble;
use crate::reaction::traits::ReactionRule;

pub struct GitRule;

impl ReactionRule for GitRule {
  fn matches(&self, event: &Event) -> bool {
    matches!(event, Event::Command(cmd) if cmd.command.contains("git"))
  }
  fn react(&self, _event: &Event) -> Option<String> {
    let buddy = "(◠‿ o )";
    let msg = "Recording today's mistake for future reference.";
    Some(wrap_in_bubble(msg, buddy))
  }
}