use crate::daemon::reaction::rules::{FailedCommandRule};
use crate::daemon::reaction::traits::ReactionRule;
use crate::shared::types::{CommandEvent, Event};

#[test]
fn test_dev_rule_matches() {
  let commands: &[&str] = &[
    "cargo check",
    "rustc --version",
    "npm run dev",
    "yarn dev",
    "pnpm run dev",
    "bun run dev",
    "python -m pip install",
    "pip install requests",
    "uv init",
    "java -version",
    "nvm use 24",
    "gradle init --type basic",
    "go main.go",
    "gcc main.c -o main",
    "vite init",
    "next dev"
  ];
  for cmd in commands {
    let event = Event::new_command(cmd.to_string());
    assert!(super::DevelopmentRule.matches(&event), "Should match '{}'", cmd);
  }
}

#[test]
fn test_dev_reaction() {
  let event = Event::new_command("cargo check".to_string());
  let response = super::DevelopmentRule.react(&event);
  assert!(response.is_some());
  assert!(response.clone().unwrap().contains("Yes, it works!"));
  assert!(response.unwrap().contains("(o_O)"));
}

#[test]
fn test_failed_dev_reaction() {
  let failed_rule = FailedCommandRule;
  let event = Event::new_command("cargo check".to_string())
    .with_exit_code(1);

  assert!(super::DevelopmentRule.react(&event).is_some());
  assert!(failed_rule.matches(&event));
  assert!(failed_rule.react(&event).is_some());
  assert!(failed_rule.react(&event).unwrap().contains("It broke"));
  assert!(failed_rule.react(&event).unwrap().contains("(╯°□°）╯︵ ┻━┻"));
}