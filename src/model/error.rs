use std::fmt::Display;

#[derive(Debug)]
pub enum  ModelError {
  Network(String),
  Timeout,
  InvalidResponse,
  Serialization(String),
  ProviderUnavailable,
}

impl From<reqwest::Error> for ModelError {
  fn from(err: reqwest::Error) -> Self {
    if err.is_timeout() {
      ModelError::Timeout
    } else {
      ModelError::Network(err.to_string())
    }
  }
}

impl Display for ModelError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ModelError::Network(error) => {
        write!(f, "Network error: {}", error)
      }
      ModelError::Timeout => {
        write!(f, "Request timed out")
      }
      ModelError::InvalidResponse => {
        write!(f, "Invalid response from model")
      }
      ModelError::Serialization(error) => {
        write!(f, "Error serializing response: {}", error)
      }
      ModelError::ProviderUnavailable => {
        write!(f, "Model provider is unavailable")
      }
    }
  }
}