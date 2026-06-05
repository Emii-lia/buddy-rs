use regex::Regex;
use crate::init::config::api::init_api_config;

pub fn init_config() -> anyhow::Result<(), String> {
  init_api_config().map_err(|e| format!("Failed to initialize API config: {}", e))
}

pub fn set_config(key: &str, value: &str) -> anyhow::Result<(), String> {
  let config_path = std::env::home_dir().unwrap().join(".config/buddy/api.conf");
  let mut config_content = std::fs::read_to_string(&config_path)
    .map_err(|e| format!("Failed to read API config: {}", e))?;
  let re = Regex::new(&format!(r"{}\s*=\s*.*", key))
    .map_err(|e| format!("Invalid regex: {}", e))?;
  config_content = re.replace_all(&config_content, format!("{}={}", key, value)).into_owned();

  std::fs::write(&config_path, config_content)
    .map_err(|e| format!("Failed to write API config: {}", e))?;
  println!("API config updated successfully");
  Ok(())
}