use crate::shared::types::Event;
use crate::shared::style::wrap_in_bubble;
use crate::daemon::reaction::traits::ReactionRule;

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
      "gcc",
      "vite",
      "next"
    ];
    matches!(
      event,
      Event::Command(cmd) if dev_commands.contains(&cmd.command.split_whitespace().next().unwrap())
    )
  }
  fn react(&self, _event: &Event) -> Option<String> {
    let msg = "Yes, it works! Try not to touch it.";
    let buddy = "(o_O)";
    Some(wrap_in_bubble(msg, buddy))
  }
}
