pub mod sender;

use shared::types::CommandEvent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let args: Vec<String> = std::env::args().collect();

  let command = args
    .get(1)
    .cloned()
    .unwrap_or_default();

  let exit_code = args
    .get(2)
    .and_then(|s| s.parse().ok())
    .unwrap_or(0);

  let duration_ms = args
    .get(3)
    .and_then(|s| s.parse().ok())
    .unwrap_or(0);
  let timestamp = args
    .get(4)
    .and_then(|s| s.parse().ok())
    .unwrap_or(0);

  let event = CommandEvent {
    command,
    exit_code,
    duration_ms,
    timestamp,
  };

  sender::send_event(event).await?;
  Ok(())
}
