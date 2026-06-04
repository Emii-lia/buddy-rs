use crate::daemon::reaction::rules::FailedCommandRule;
use crate::daemon::reaction::rules::search::SearchRule;
use crate::daemon::reaction::traits::ReactionRule;
use crate::shared::types::Event;

#[test]
fn test_search_rule_matches() {
  let rule = SearchRule;
  let commands: &[&str] = &[
    "find . -name test.rs",
    "lookup test.rs",
    "grep 'test' *.rs",
    "rg 'test'",
    "fd 'test'",
    "locate test.rs"
  ];

  for cmd in commands {
    let event = Event::new_command(cmd.to_string());
    assert!(rule.matches(&event), "Should match '{}'", cmd);
  }
}

#[test]
fn test_search_reaction() {
  let rule = SearchRule;

  let event = Event::new_command("find . -name test.rs".to_string());
  assert!(rule.react(&event).is_some());
  assert!(rule.react(&event).unwrap().contains("The answer was apparently nearby the entire time."));
  assert!(rule.react(&event).unwrap().contains("(ง'̀-'́)ง"));
}

#[test]
fn test_search_failed_reaction() {
  let rule = SearchRule;
  let failed_rule = FailedCommandRule;

  let event = Event::new_command("find . -name test.rs".to_string())
    .with_exit_code(1);

  assert!(rule.react(&event).is_some());
  assert!(failed_rule.matches(&event));
  assert!(failed_rule.react(&event).is_some());
  assert!(failed_rule.react(&event).unwrap().contains("It broke"));
  assert!(failed_rule.react(&event).unwrap().contains("(╯°□°）╯︵ ┻━┻"));
}