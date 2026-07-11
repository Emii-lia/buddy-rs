use std::env::home_dir;
use crate::model::provider::LlmProvider;

pub struct ApiConfig {
  pub provider: String,
  pub api_key: String,
  pub base_url: String,
  pub model: String,
}

impl ApiConfig {
  pub fn load_env() -> Self {
    dotenvy::dotenv().ok();
    let mode = std::env::var("APP_ENV").unwrap_or_else(|_| "prod".to_string());
    let mut cfg = if mode.as_str() == "local" || mode.as_str() == "ci" {
      ApiConfig {
        provider: std::env::var("BUDDY_PROVIDER").unwrap(),
        api_key: std::env::var("BUDDY_API_KEY").unwrap(),
        base_url: std::env::var("BUDDY_BASE_URL").unwrap(),
        model: std::env::var("BUDDY_MODEL").unwrap(),
      }
    } else if mode.as_str() == "prod" {
      let config_path = home_dir().unwrap().join(".config/buddy/api.conf");
      let config_content = std::fs::read_to_string(config_path)
        .expect("Failed to read API config: Please run `buddy config init` to initialize the API config");
      let mut api_config = ApiConfig {
        provider: "groq".to_string(),
        api_key: "".to_string(),
        base_url: "".to_string(),
        model: "".to_string(),
      };
      for line in config_content.lines() {
        let (key, value) = line.split_once("=").unwrap();
        if key == "BUDDY_API_KEY" {
          api_config.api_key = value.to_string();
        } else if key == "BUDDY_BASE_URL" {
          api_config.base_url = value.to_string();
        } else if key == "BUDDY_MODEL" {
          api_config.model = value.to_string();
        } else if key == "BUDDY_PROVIDER" {
          api_config.provider = value.to_string();
        } else {
          println!("Unknown key: {}", key);
        }
      }
      api_config
    } else {
      panic!("Unknown mode: {}", mode);
    };

    let provider = LlmProvider::from(cfg.provider.as_str());
    if cfg.base_url.trim().is_empty() {
      cfg.base_url = provider.default_base_url().to_string();
    }
    if cfg.model.trim().is_empty() {
      cfg.model = provider.default_model().to_string();
    }
    cfg
  }
}

pub fn init_api_config() -> anyhow::Result<()> {
  println!("Initializing api config...");
  let buddy_api_conf = home_dir().unwrap().join(".config/buddy/api.conf");
  if buddy_api_conf.exists() {
    println!("Buddy api config already exists");
    return Ok(());
  }
  std::fs::create_dir_all(buddy_api_conf.parent().unwrap()).expect("Failed to create buddy api config directory");
  std::fs::write(buddy_api_conf, include_str!("../../../assets/api.conf")).expect("Failed to write buddy api config");
  println!("Buddy api config initialized");
  Ok(())
}