use crate::daemon::reaction::rules::{BuildRule, FailedCommandRule};
use crate::daemon::reaction::traits::ReactionRule;
use crate::shared::types::{CommandEvent, Event};

#[test]
fn test_build_rule_matches() {
  let rule = BuildRule;
  let commands = vec!["yarn build", "cargo build", "make build"];

  for cmd in commands {
    let event = Event::new_command(cmd.to_string());
    assert!(rule.matches(&event), "Should match '{}'", cmd);
  }
}

#[test]
fn test_build_reaction() {
  let rule = BuildRule;
  let event = Event::new_command("yarn build".to_string());
  assert!(rule.react(&event).is_some());
  assert!(rule.react(&event).unwrap().contains("Done"));
  assert!(rule.react(&event).unwrap().contains("(•‿•)"));
}

#[test]
fn test_long_build_reaction() {
  let rule = BuildRule;
  let event = Event::new_command("yarn build".to_string())
    .with_duration_ms(11000);
  
  let response = rule.react(&event);
  assert!(response.is_some());
  assert!(response.clone().unwrap().contains("Finally"));
  assert!(response.unwrap().contains("(~_~メ)"));
}

#[test]
fn test_failed_build_reaction() {
  let rule = BuildRule;
  let failed_rule = FailedCommandRule;
  let event = Event::new_command("yarn build".to_string())
    .with_exit_code(1);
  
  let failed_response = failed_rule.react(&event);
  assert!(rule.react(&event).is_some());
  assert!(failed_rule.matches(&event));
  assert!(failed_response.is_some());
  assert!(failed_response.clone().unwrap().contains("It broke"));
  assert!(failed_response.unwrap().contains("(╯°□°）╯︵ ┻━┻"));
}