use crate::prompt::builder::Prompt;
use crate::prompt::types::OutputFormat;

pub struct ModelRequest {
  pub system: String,
  pub user: String,
  pub format: OutputFormat,
  pub temperature: f32,
  pub max_tokens: Option<u32>,
}

impl From<Prompt> for ModelRequest {
  fn from(prompt: Prompt) -> Self {
    Self {
      system: prompt.system,
      user: prompt.user,
      format: OutputFormat::Json,
      temperature: 0.2,
      max_tokens: Some(800),
    }
  }
}