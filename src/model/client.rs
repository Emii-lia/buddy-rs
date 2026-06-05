use crate::model::error::ModelError;
use crate::model::request::ModelRequest;
use crate::model::response::ModelResponse;

#[async_trait::async_trait]
pub trait LlmClient: Send + Sync {
  async fn generate(&self, req: ModelRequest) -> anyhow::Result<ModelResponse, ModelError>;
}