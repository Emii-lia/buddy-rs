use shared::types::Event;

pub trait ReactionRule {
  fn matches(&self, event: &Event) -> bool;
  fn react(&self, event: &Event) -> Option<String>;
}