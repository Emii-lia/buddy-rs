use crate::daemon::reaction::rules::FailedCommandRule;
use crate::daemon::reaction::traits::ReactionRule;
use crate::shared::types::Event;

#[test]
fn test_fail_rule_matches() {
  let rule = FailedCommandRule;
  let event = Event::new_command("ls".to_string())
    .with_exit_code(1);

  assert!(rule.matches(&event));
}

#[test]
fn test_fail_reaction() {
  let rule = FailedCommandRule;
  let event = Event::new_command("ls".to_string())
    .with_exit_code(1);

  let response = rule.react(&event);
  assert!(response.is_some());
  assert!(response.clone().unwrap().contains("It broke"));
  assert!(response.unwrap().contains("(╯°□°）╯︵ ┻━┻"));
}