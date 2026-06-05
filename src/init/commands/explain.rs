use crate::model::client::LlmClient;
use crate::model::provider::GroqClient;
use crate::model::request::ModelRequest;
use crate::prompt::builder::PromptBuilder;
use crate::prompt::context::{CommandInput, PromptContext, SystemContext};
use crate::prompt::policies::PolicyType;
use crate::prompt::templates::ExplainCommandTemplate;
use crate::prompt::types::{PromptIntent, RiskLevel, Verbosity};

pub async  fn explain(command: &str) -> anyhow::Result<(), String> {
  let os: String = std::env::consts::OS.to_string();
  let shell: String = std::env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());

  let ctx = PromptContext {
    system: SystemContext { os, shell, cwd: None },
    intent: PromptIntent::ExplainCommand,
    command: CommandInput {
      raw: command.to_string(),
      binary: Some(command.split_whitespace().next().unwrap_or("").to_string()),
      args: command.split_whitespace().skip(1).map(|s| s.to_string()).collect(),
      risk_level: RiskLevel::Medium,
    },
    verbosity: Verbosity::Normal,
  };

  let prompt = PromptBuilder::new(ctx)
    .policy(PolicyType::Buddy.to_policy())
    .build(ExplainCommandTemplate);

  let request = ModelRequest::from(prompt);
  let groq = GroqClient::new();
  let response = groq.generate(request)
    .await
    .map_err(|e| format!("Model error: {}", e))?;
  
  println!("{}", response.text);
  Ok(())
}