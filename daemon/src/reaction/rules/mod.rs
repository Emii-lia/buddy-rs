mod build;
mod fail;
mod dev;
mod pacman;
pub mod git;

pub use fail::FailedCommandRule;
pub use build::BuildRule;
pub use dev::DevelopmentRule;
use crate::reaction::rules::git::GitRule;
use crate::reaction::rules::pacman::PackageManagerRule;
use crate::reaction::traits::ReactionRule;


pub fn get_rules() -> Vec<Box<dyn ReactionRule>> {
  vec![
    Box::new(BuildRule),
    Box::new(DevelopmentRule),
    Box::new(PackageManagerRule),
    Box::new(GitRule),
  ]
}