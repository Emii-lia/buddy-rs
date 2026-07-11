use crate::init::config::api::ApiConfig;
use crate::model::client::LlmClient;
use crate::model::provider::anthropic::AnthropicClient;
use crate::model::provider::openai_compatible::OpenAICompatibleClient;

pub mod openai_compatible;
pub mod anthropic;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LlmProvider {
  Groq,
  OpenAI,
  Anthropic,
}

impl From<&str> for LlmProvider {
  fn from(value: &str) -> Self {
    match value.to_lowercase().as_str() {
      "groq" => LlmProvider::Groq,
      "openai" => LlmProvider::OpenAI,
      "anthropic" | "claude" => LlmProvider::Anthropic,
      _ => LlmProvider::Groq,
    }
  }
}

impl LlmProvider {
  pub fn default_base_url(&self) -> &'static str {
    match self {
      LlmProvider::Groq => "https://api.groq.com/openai/v1/chat/completions",
      LlmProvider::OpenAI => "https://api.openai.com/v1/chat/completions",
      LlmProvider::Anthropic => "https://api.anthropic.com/v1/messages",
    }
  }

  pub fn default_model(&self) -> &'static str {
    match self {
      LlmProvider::Groq => "llama-3.3-70b-versatile",
      LlmProvider::OpenAI => "gpt-4o-mini",
      LlmProvider::Anthropic => "claude-sonnet-4-5",
    }
  }
}

pub fn build_client(cfg: &ApiConfig) ->  Box<dyn LlmClient + Send + Sync> {
  match LlmProvider::from(cfg.provider.as_str()) {
    LlmProvider::Anthropic => Box::new(AnthropicClient {
      api_key: cfg.api_key.clone(),
      base_url: cfg.base_url.clone(),
      model: cfg.model.clone(),
    }),
    _ => Box::new(OpenAICompatibleClient {
      api_key: cfg.api_key.clone(),
      base_url: cfg.base_url.clone(),
      model: cfg.model.clone(),
    })
  }
}

