use shared::types::Event;
use shared::style::{wrap_in_bubble};
use crate::reaction::traits::ReactionRule;

pub struct FailedCommandRule;
impl ReactionRule for FailedCommandRule {
  fn matches(&self, event: &Event) -> bool {
    matches!(event, Event::Command(cmd) if cmd.exit_code != 0)
  }
  fn react(&self, _event: &Event) -> Option<String> {
    let msg = "It broke. As expected. Moving on emotionally.";
    let buddy = "(╯°□°)╯";
    Some(wrap_in_bubble(msg, &buddy))
  }
}
