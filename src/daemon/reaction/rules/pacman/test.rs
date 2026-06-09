use crate::daemon::reaction::rules::FailedCommandRule;
use crate::daemon::reaction::rules::pacman::PackageManagerRule;
use crate::daemon::reaction::traits::ReactionRule;
use crate::shared::types::Event;

#[test]
fn test_pacman_rule_matches() {
  let commands: &[&str] = &[
    "pacman -Syu",
    "yay -Syu",
    "paru -Syu",
    "apt update && apt upgrade -y",
    "flatpak update && flatpak upgrade -y",
    "snap refresh",
    "dnf upgrade -y",
    "brew upgrade"
  ];

  for cmd in commands {
    let event = Event::new_command(cmd.to_string());
    assert!(PackageManagerRule.matches(&event), "Should match '{}'", cmd);
  }
}

#[test]
fn test_pacman_reaction() {
  let event = Event::new_command("pacman -Syu".to_string());
  assert!(PackageManagerRule.react(&event).is_some());
  assert!(PackageManagerRule.react(&event).unwrap().contains("It succeeded"));
  assert!(PackageManagerRule.react(&event).unwrap().contains("<( ￣ ︶ ￣ )>"));
}

#[test]
fn test_pacman_failed_reaction() {
  let failed_rule = FailedCommandRule;

  let event = Event::new_command("pacman -Syu".to_string())
    .with_exit_code(1);

  assert!(PackageManagerRule.react(&event).is_some());
  assert!(failed_rule.matches(&event));
  assert!(failed_rule.react(&event).is_some());
  assert!(failed_rule.react(&event).unwrap().contains("It broke"));
  assert!(failed_rule.react(&event).unwrap().contains("(╯°□°）╯︵ ┻━┻"));
}