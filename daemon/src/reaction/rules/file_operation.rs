use shared::types::Event;
use shared::style::wrap_in_bubble;
use crate::reaction::traits::ReactionRule;

pub struct FileOperationRule;

impl ReactionRule for FileOperationRule {
  fn matches(&self, event: &Event) -> bool {
    let fileops: &[&str] = &["touch", "mkdir", "rm", "mv", "cp", "ln", "rmdir"];
    matches!(
      event,
      Event::Command(cmd) if fileops.contains(&cmd.command.split_whitespace().next().unwrap())
    )
  }

  fn react(&self, event: &Event) -> Option<String> {
    let Event::Command(cmd) = event;
    if ["rm", "rmdir"].contains(&cmd.command.split_whitespace().next().unwrap()) {
      let msg = "Deleting evidence";
      let buddy = "(・_・;)";
      Some(wrap_in_bubble(msg, buddy))
    } else if ["mv", "cp", "ln"].contains(&cmd.command.split_whitespace().next().unwrap()) {
      let msg = "Everything is exactly where you left it. Probably.";
      let buddy = "╮(╯_╰)╭";
      Some(wrap_in_bubble(msg, buddy))
    } else {
      let msg = "New files created. Where? No idea.";
      let buddy = "┐(˘_˘)┌";
      Some(wrap_in_bubble(msg, buddy))
    }
  }
}