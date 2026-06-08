pub mod test;

use crate::daemon::reaction::traits::ReactionRule;
use crate::shared::style::wrap_in_bubble;
use crate::shared::types::Event;

pub struct DangerRule;

impl ReactionRule for DangerRule {
  fn matches(&self, event: &Event) -> bool {
    let danger_commands: &[&str] = &[
      "--force",
      "dd",
      "parted",
      "fdisk",
      "wipefs",
      "mkfs"
    ];
    let special_danger_commands: &[&str] = &["rm -rf", "chmod -R 777", "mkfs."];

    matches!(
      event,
      Event::Command(cmd) if
      danger_commands.iter().any(|c| cmd.command.split_whitespace().any(|s| s == *c))
      || special_danger_commands.iter().any(|c| cmd.command.contains(*c))
    )
  }

  fn react(&self, _event: &Event) -> Option<String> {
    let msg = "Everything is fine. That's the problem";
    let buddy = "(;ŏ﹏ŏ)";
    Some(wrap_in_bubble(msg, buddy))
  }
}