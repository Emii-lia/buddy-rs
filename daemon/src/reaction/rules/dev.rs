use shared::types::Event;
use crate::reaction::style::wrap_in_bubble;
use crate::reaction::traits::ReactionRule;

pub struct DevelopmentRule;

impl ReactionRule for DevelopmentRule {
  fn matches(&self, event: &Event) -> bool {
    let dev_commands: &[&str] = &[
      "cargo",
      "rustc",
      "npm",
      "yarn",
      "pnpm",
      "bun",
      "python",
      "pip",
      "uv",
      "java",
      "nvm",
      "gradle",
      "go",
      "gcc"
    ];
    matches!(
      event,
      Event::Command(cmd) if dev_commands.iter().any(|dev| cmd.command.contains(dev))
    )
  }
  fn react(&self, _event: &Event) -> Option<String> {
    let msg = "Yes, it works! Try not to touch it.";
    let buddy = "(⁠o⁠_⁠O⁠)";
    Some(wrap_in_bubble(msg, &buddy))
  }
}
