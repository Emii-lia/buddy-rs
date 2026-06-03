use crate::shared::types::{CommandEvent, Event};
use crate::daemon::reaction::rules::{get_rules, FailedCommandRule};
use crate::daemon::reaction::traits::ReactionRule;

pub mod traits;
pub mod rules;


pub fn react_to_command(command_event: CommandEvent) -> Vec<String> {
  let mut responses = Vec::new();
  let event = Event::Command(command_event);
  let rules = get_rules();

  if FailedCommandRule.matches(&event) {
    if let Some(response) = FailedCommandRule.react(&event) {
      responses.push(response);
    }
  } else {
    for rule in rules {
      if rule.matches(&event) {
        if let Some(response) = rule.react(&event) {
          responses.push(response);
        }
      }
    }
  }

  responses
}