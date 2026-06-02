use shared::style::wrap_in_bubble;
use crate::config::create_config;
use crate::config::service::create_service;
use crate::shell::detect::detect_shell;

pub fn install() -> anyhow::Result<(), anyhow::Error> {
  let shell = detect_shell().map_err(|e| anyhow::anyhow!("Failed to detect shell: {}", e))?;
  println!("Detected shell: {}", shell.to_string());
  println!();
  println!("Creating buddy config...");
  create_config().map_err(|e| anyhow::anyhow!("Failed to create config: {}", e))?;
  create_service().map_err(|e| anyhow::anyhow!("Failed to create service: {}", e))?;
  println!();
  shell.install();
  println!();
  println!("Buddy installed!");
  println!("{}", wrap_in_bubble(
    "Buddy will now observe your actions",
    "(｡•̀ᴗ-)✧"
  ));
  Ok(())
}