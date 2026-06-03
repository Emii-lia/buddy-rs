use shell_hook::run_shell_hook;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  run_shell_hook().await
}