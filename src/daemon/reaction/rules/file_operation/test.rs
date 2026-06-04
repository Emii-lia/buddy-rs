use crate::daemon::reaction::rules::FailedCommandRule;
use crate::daemon::reaction::rules::file_operation::FileOperationRule;
use crate::daemon::reaction::traits::ReactionRule;
use crate::shared::types::Event;

#[test]
fn test_file_operation_rule_matches() {
  let commands: &[&str] = &[
    "touch file.txt",
    "mkdir new_folder",
    "rm file.txt",
    "mv file.txt new_folder/file.txt",
    "cp new_folder/file.txt file.txt",
    "ln file.txt -s new_folder/file",
    "rmdir -r new_folder"
  ];
  let rule = FileOperationRule;

  for cmd in commands {
    let event = Event::new_command(cmd.to_string());
    assert!(rule.matches(&event), "Should match '{}'", cmd);
  }
}

#[test]
fn test_file_removal_reaction() {
  let rule = FileOperationRule;
  let commands: &[&str] = &[
    "rm file.txt",
    "rmdir new_folder",
  ];

  for cmd in commands {
    let event = Event::new_command(cmd.to_string());
    assert!(rule.react(&event).is_some());
    assert!(rule.react(&event).unwrap().contains("Deleting evidence"));
    assert!(rule.react(&event).unwrap().contains("(・_・;)"));
  }
}

#[test]
fn test_file_edit_reaction() {
  let rule = FileOperationRule;
  let commands: &[&str] = &[
    "mv file.txt new_folder/file.txt",
    "cp new_folder/file.txt file.txt",
    "ln file.txt -s new_folder/file",
  ];

  for cmd in commands {
    let event = Event::new_command(cmd.to_string());
    assert!(rule.react(&event).is_some());
    assert!(rule.react(&event).unwrap().contains("Everything is exactly where you left it. Probably."));
    assert!(rule.react(&event).unwrap().contains("╮(╯_╰)╭"));
  }
}

#[test]
fn test_file_creation_reaction() {
  let rule = FileOperationRule;
  let event = Event::new_command("touch file.txt".to_string());
  assert!(rule.react(&event).is_some());
  assert!(rule.react(&event).unwrap().contains("New files created. Where? No idea."));
  assert!(rule.react(&event).unwrap().contains("┐(˘_˘)┌"));
}

#[test]
fn test_failed_file_operation_reaction() {
  let rule = FileOperationRule;
  let failed_rule = FailedCommandRule;
  let event = Event::new_command("touch file.txt".to_string())
    .with_exit_code(1);
  assert!(rule.react(&event).is_some());
  assert!(failed_rule.matches(&event));
  assert!(failed_rule.react(&event).is_some());
  assert!(failed_rule.react(&event).unwrap().contains("It broke"));
  assert!(failed_rule.react(&event).unwrap().contains("(╯°□°）╯︵ ┻━┻"));
}