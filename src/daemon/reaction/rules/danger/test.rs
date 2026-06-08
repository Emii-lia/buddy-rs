use crate::daemon::reaction::rules::FailedCommandRule;
use crate::daemon::reaction::traits::ReactionRule;
use crate::shared::types::Event;

#[test]
fn test_danger_rule_matches() {
  let commands = [
    "rm -rf buddy-test",
    "chmod -R 777 buddy-test",
    "git push origin --force",
    "dd if=/dev/zero of=buddy-test.img bs=1M count=100",
    "mkfs.ext4 buddy-test.img",
    "parted buddy-test.img mklabel msdos",
    "parted buddy-test.img mkpart primary ext4 0% 100%",
    "wipefs -af buddy-test.img",
  ];
  for cmd in commands {
    let event = Event::new_command(cmd.to_string());
    assert!(
      super::DangerRule.matches(&event),
      "Should match '{}'",
      cmd
    )
  }
}

#[test]
fn test_danger_reaction() {
  let event = Event::new_command("rm -rf buddy-test".to_string());
  let response = super::DangerRule.react(&event);
  assert!(response.is_some());
  assert!(response.clone().unwrap().contains("Everything is fine. That's the problem"));
  assert!(response.unwrap().contains("(;ŏ﹏ŏ)"));
}

#[test]
fn test_failed_danger_reaction() {
  let failed_rule = FailedCommandRule;
  let event = Event::new_command("rm -rf buddy-test".to_string())
    .with_exit_code(1);
  let response = super::DangerRule.react(&event);
  let failed_response = failed_rule.react(&event);

  assert!(response.is_some());
  assert!(failed_rule.matches(&event));
  assert!(failed_response.is_some());
  assert!(failed_response.clone().unwrap().contains("It broke"));
  assert!(failed_response.unwrap().contains("(╯°□°）╯︵ ┻━┻"));
}