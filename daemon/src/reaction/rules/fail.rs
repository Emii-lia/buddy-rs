use colored::Color;
use shared::types::Event;
use crate::reaction::style::{style_icon, wrap_in_bubble};
use crate::reaction::traits::ReactionRule;

pub struct FailedCommandRule;
impl ReactionRule for FailedCommandRule {
  fn matches(&self, event: &Event) -> bool {
    matches!(event, Event::Command(cmd) if cmd.exit_code != 0)
  }
  fn react(&self, _event: &Event) -> Option<String> {
    let msg = "It broke. As expected. Moving on emotionally.";
    let buddy = format!("{} (╯°□°)╯", style_icon("", Color::BrightRed));
    Some(wrap_in_bubble(msg, &buddy))
  }
}
