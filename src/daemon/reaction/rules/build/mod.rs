pub mod test;

use crate::shared::types::Event;
use crate::shared::style::{wrap_in_bubble};
use crate::daemon::reaction::traits::ReactionRule;

pub struct BuildRule;
impl ReactionRule for BuildRule {
  fn matches(&self, event: &Event) -> bool {
    matches!(event, Event::Command(cmd) if cmd.command.contains("build"))
  }

  fn react(&self, event: &Event) -> Option<String> {
    let Event::Command(cmd) = event;
    if cmd.duration_ms > 10000 {
      let msg = "Finally. After several identity crises, it compiled itself into submission.";
      let buddy = "(~_~メ)";
      Some(wrap_in_bubble(msg, buddy))
    } else {
      let msg = "Done. Suspiciously quick. No complaints yet, which is worrying";
      let buddy = "(•‿•)";
      Some(wrap_in_bubble(msg, buddy))
    }
  }
}
