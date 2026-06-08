mod build;
mod fail;
mod dev;
mod pacman;
pub mod git;
pub mod file_operation;
pub mod search;
pub mod sudo;
pub mod danger;

pub use fail::FailedCommandRule;
pub use build::BuildRule;
pub use dev::DevelopmentRule;
use crate::daemon::reaction::rules::danger::DangerRule;
use crate::daemon::reaction::rules::file_operation::FileOperationRule;
use crate::daemon::reaction::rules::git::GitRule;
use crate::daemon::reaction::rules::pacman::PackageManagerRule;
use crate::daemon::reaction::rules::search::SearchRule;
use crate::daemon::reaction::rules::sudo::SudoRule;
use crate::daemon::reaction::traits::ReactionRule;

pub fn get_rules() -> Vec<Box<dyn ReactionRule>> {
  vec![
    Box::new(BuildRule),
    Box::new(DevelopmentRule),
    Box::new(PackageManagerRule),
    Box::new(GitRule),
    Box::new(FileOperationRule),
    Box::new(SearchRule),
    Box::new(SudoRule),
    Box::new(DangerRule),
  ]
}