use crate::init::shell::Shell;

pub fn detect_shell() -> anyhow::Result<Shell> {
  let shell = std::env::var("SHELL").unwrap_or_default();
  if shell.contains("fish") {
    Ok(Shell::Fish)
  } else if shell.contains("bash") {
    Ok(Shell::Bash)
  } else if shell.contains("zsh") {
    Ok(Shell::Zsh)
  } else {
    Err(anyhow::anyhow!("Unsupported shell: {}", shell))
  }
}