use crate::prompt::types::{PromptIntent, RiskLevel, Verbosity};

pub struct CommandInput {
  pub raw: String,
  pub binary: Option<String>,
  pub args: Vec<String>,
  pub risk_level: RiskLevel,
}

pub struct SystemContext {
  pub os: String,
  pub shell: String,
  pub cwd: Option<String>,
}
pub struct PromptContext {
  pub system: SystemContext,
  pub intent: PromptIntent,
  pub command: CommandInput,
  pub verbosity: Verbosity,
}