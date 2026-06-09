use crate::daemon::reaction::rules::{FailedCommandRule};
use crate::daemon::reaction::traits::ReactionRule;
use crate::shared::types::{CommandEvent, Event};

#[test]
fn test_build_rule_matches() {
  
  let commands = vec!["yarn build", "cargo build", "make build"];

  for cmd in commands {
    let event = Event::new_command(cmd.to_string());
    assert!(super::BuildRule.matches(&event), "Should match '{}'", cmd);
  }
}

#[test]
fn test_build_reaction() {
  let event = Event::new_command("yarn build".to_string());
  assert!(super::BuildRule.react(&event).is_some());
  assert!(super::BuildRule.react(&event).unwrap().contains("Done"));
  assert!(super::BuildRule.react(&event).unwrap().contains("(•‿•)"));
}

#[test]
fn test_long_build_reaction() {
  let event = Event::new_command("yarn build".to_string())
    .with_duration_ms(11000);
  
  let response = super::BuildRule.react(&event);
  assert!(response.is_some());
  assert!(response.clone().unwrap().contains("Finally"));
  assert!(response.unwrap().contains("(~_~メ)"));
}

#[test]
fn test_failed_build_reaction() {
  let failed_rule = FailedCommandRule;
  let event = Event::new_command("yarn build".to_string())
    .with_exit_code(1);
  
  let failed_response = failed_rule.react(&event);
  assert!(super::BuildRule.react(&event).is_some());
  assert!(failed_rule.matches(&event));
  assert!(failed_response.is_some());
  assert!(failed_response.clone().unwrap().contains("It broke"));
  assert!(failed_response.unwrap().contains("(╯°□°）╯︵ ┻━┻"));
}