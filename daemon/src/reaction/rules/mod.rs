mod build;
mod fail;
mod dev;
mod pacman;
pub mod git;
pub mod file_operation;
pub mod search;

pub use fail::FailedCommandRule;
pub use build::BuildRule;
pub use dev::DevelopmentRule;
use crate::reaction::rules::file_operation::FileOperationRule;
use crate::reaction::rules::git::GitRule;
use crate::reaction::rules::pacman::PackageManagerRule;
use crate::reaction::rules::search::SearchRule;
use crate::reaction::traits::ReactionRule;


pub fn get_rules() -> Vec<Box<dyn ReactionRule>> {
  vec![
    Box::new(BuildRule),
    Box::new(DevelopmentRule),
    Box::new(PackageManagerRule),
    Box::new(GitRule),
    Box::new(FileOperationRule),
    Box::new(SearchRule),
  ]
}