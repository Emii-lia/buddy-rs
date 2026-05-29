use crate::config::service::remove_service;
use crate::shell::detect::detect_shell;

pub fn uninstall() -> anyhow::Result<(), anyhow::Error> {
  let shell = detect_shell().map_err(|e| anyhow::anyhow!("Failed to detect shell: {}", e))?;
  println!("Removing buddy service...");
  remove_service().map_err(|e| anyhow::anyhow!("Failed to remove service: {}", e))?;
  println!("Removing buddy config...");
  shell.uninstall();
  println!("Buddy uninstalled");
  Ok(())
}