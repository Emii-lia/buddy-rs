use crate::model::client::LlmClient;
use crate::model::error::ModelError;
use crate::model::request::ModelRequest;
use crate::model::response::ModelResponse;

pub struct AnthropicClient {
  pub api_key: String,
  pub base_url: String,
  pub model: String,
}

#[async_trait::async_trait]
impl LlmClient for AnthropicClient {
  async fn generate(&self, req: ModelRequest) -> anyhow::Result<ModelResponse, ModelError> {
    let payload = serde_json::json!({
      "model": self.model,
      "system": req.system,
      "messages": [
        {
          "role": "user",
          "content": req.user,
        }
      ],
      "temperature": req.temperature,
      "max_tokens": req.max_tokens.unwrap_or(1024),
    });

    let res = reqwest::Client::new()
      .post(&self.base_url)
      .header("x-api-key", &self.api_key)
      .header("anthropic-version", "2023-06-01")
      .header("content-type", "application/json")
      .json(&payload)
      .send()
      .await?;

    if !res.status().is_success() {
      return Err(ModelError::Network(format!("API error: {}", res.status())));
    }

    let json: serde_json::Value = res.json().await?;
    let text = json["content"][0]["text"]
      .as_str()
      .ok_or(ModelError::InvalidResponse)?
      .to_string();

    let finish_reason = json["stop_reason"]
      .as_str()
      .map(|s| s.to_string());

    Ok(ModelResponse {
      text,
      raw: json.to_string(),
      finish_reason,
    })
  }
}