use crate::daemon::reaction::rules::FailedCommandRule;
use crate::daemon::reaction::traits::ReactionRule;
use crate::shared::types::Event;

#[test]
fn test_sudo_rule_matches() {
  let commands = vec!["sudo pacman -S", "please systemctl status buddy.service"];
  for cmd in commands {
    let event = Event::new_command(cmd.to_string());
    assert!(
      super::SudoRule.matches(&event),
      "Should match '{}'",
      cmd
    )
  }
}

#[test]
fn test_sudo_reaction() {
  let event = Event::new_command("sudo pacman -S".to_string());
  let response = super::SudoRule.react(&event);
  assert!(response.is_some());
  assert!(response.clone().unwrap().contains("Root has entered the chat"));
  assert!(response.unwrap().contains("(⚆_⚆)"));
}

#[test]
fn test_failed_sudo_reaction() {
  let failed_rule = FailedCommandRule;
  let event = Event::new_command("sudo pacman -S".to_string())
    .with_exit_code(1);
  let response = super::SudoRule.react(&event);
  let failed_response = failed_rule.react(&event);

  assert!(response.is_some());
  assert!(failed_rule.matches(&event));
  assert!(failed_response.is_some());
  assert!(failed_response.clone().unwrap().contains("It broke"));
  assert!(failed_response.unwrap().contains("(╯°□°）╯︵ ┻━┻"));
}