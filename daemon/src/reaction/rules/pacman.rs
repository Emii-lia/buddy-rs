use shared::types::Event;
use crate::reaction::traits::ReactionRule;

pub struct PackageManagerRule;

impl ReactionRule for PackageManagerRule {
  fn matches(&self, event: &Event) -> bool {
    let pacman_commands = &[
      "pacman",
      "yay",
      "paru",
      "apt",
      "flatpak",
      "snap",
      "dnf",
      "brew"
    ];

    matches!(event, Event::Command(cmd) if pacman_commands.contains(&cmd.command.split_whitespace().next().unwrap()))
  }
  fn react(&self, _event: &Event) -> Option<String> {
    let buddy = "<⁠(⁠￣⁠︶⁠￣⁠)⁠>";
    let msg = "It succeeded. The consequences are scheduled later.";
    Some(format!("{} {}", msg, buddy))
  }
}