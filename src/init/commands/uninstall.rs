use crate::init::config::service::remove_service;
use crate::init::shell::detect::detect_shell;

pub fn uninstall() -> anyhow::Result<(), anyhow::Error> {
  let shell = detect_shell().map_err(|e| anyhow::anyhow!("Failed to detect shell: {}", e))?;
  shell.uninstall();
  println!("Removing buddy service...");
  remove_service().map_err(|e| anyhow::anyhow!("Failed to remove service: {}", e))?;
  println!("Removing buddy config...");
  println!("Buddy uninstalled");
  Ok(())
}