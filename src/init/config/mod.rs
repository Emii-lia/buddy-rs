pub mod service;
pub mod api;

use std::env::home_dir;

pub fn create_config() -> anyhow::Result<()> {
  let config_path = home_dir().unwrap().join(".config/buddy");
  std::fs::create_dir_all(config_path)?;
  Ok(())
}