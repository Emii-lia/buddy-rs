use crate::daemon::reaction::rules::FailedCommandRule;
use crate::daemon::reaction::rules::git::GitRule;
use crate::daemon::reaction::traits::ReactionRule;
use crate::shared::types::Event;

#[test]
fn test_git_rule_matches() {
  let rule = GitRule;
  let commands: &[&str] = &[
    "git status",
    "git add .",
    "git commit -m 'Initial commit'",
    "git push -u origin main"
  ];
  for cmd in commands {
    let event = Event::new_command(cmd.to_string());
    assert!(rule.matches(&event), "Should match '{}'", cmd);
  }
}

#[test]
fn test_git_reaction() {
  let rule = GitRule;
  let event = Event::new_command("git status".to_string());
  assert!(rule.react(&event).is_some());
  assert!(rule.react(&event).unwrap().contains("Recording today's mistake for future reference."));
  assert!(rule.react(&event).unwrap().contains("(◠‿ o )"));
}

#[test]
fn test_git_failed_reaction() {
  let rule = GitRule;
  let failed_rule = FailedCommandRule;
  
  let event = Event::new_command("git status".to_string())
    .with_exit_code(1);
  assert!(rule.react(&event).is_some());
  assert!(failed_rule.matches(&event));
  assert!(failed_rule.react(&event).is_some());
  assert!(failed_rule.react(&event).unwrap().contains("It broke"));
  assert!(failed_rule.react(&event).unwrap().contains("(╯°□°）╯︵ ┻━┻"));
}