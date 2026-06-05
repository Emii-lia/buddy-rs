use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelResponse {
  pub text: String,
  pub raw: String,
  pub finish_reason: Option<String>,
}