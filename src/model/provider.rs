use serde_json::json;
use crate::model::client::LlmClient;
use crate::model::error::ModelError;
use crate::model::request::ModelRequest;
use crate::model::response::ModelResponse;

pub struct GroqClient {
  pub api_key: String,
  pub base_url: String,
  pub model: String,
}

#[async_trait::async_trait]
impl LlmClient for GroqClient {
  async fn generate(&self, req: ModelRequest) -> anyhow::Result<ModelResponse, ModelError> {
    let payload = json!({
      "model": self.model,
      "messages": [
        {
          "role": "system",
          "content": req.system,
        },
        {
          "role": "user",
          "content": req.user,
        }
      ],
      "temperature": req.temperature,
      "max_tokens": req.max_tokens,
    });


    let res = reqwest::Client::new()
      .post(&self.base_url)
      .bearer_auth(&self.api_key)
      .json(&payload)
      .send()
      .await?;


    if !res.status().is_success() {
      return Err(ModelError::Network(format!("API error: {}", res.status())));
    }

    let json: serde_json::Value = res.json().await?;

    let text = json["choices"][0]["message"]["content"]
      .as_str()
      .ok_or(ModelError::InvalidResponse)?
      .to_string();

    let finish_reason = json["choices"][0]["finish_reason"]
      .as_str()
      .map(|s| s.to_string());

    Ok(ModelResponse {
      text,
      raw: json.to_string(),
      finish_reason,
    })
  }
}

impl GroqClient {
  pub fn new() -> Self {
    let env = include_str!("../../.env");
    let mut config: GroqClient = GroqClient {
      api_key: "".to_string(),
      base_url: "".to_string(),
      model: "".to_string()
    };
    for line in env.lines() {
      let (key, value) = line.split_once("=").unwrap();
      if key == "AI_API_KEY" {
        config.api_key = value.to_string();
      } else if key == "AI_BASE_URL" {
        config.base_url = value.to_string();
      } else if key == "AI_MODEL" {
        config.model = value.to_string();
      }
    }
    config
  }
}