use tokio::io::AsyncWriteExt;
use tokio::net::UnixStream;
use shared::constant::SOCKET_PATH;
use shared::types::CommandEvent;

pub async fn send_event(event: CommandEvent) -> anyhow::Result<()> {
  let json = serde_json::to_string(&event)?;
  let mut stream = UnixStream::connect(SOCKET_PATH).await?;
  stream.write_all(json.as_bytes()).await?;
  stream.write_all(b"\n").await?;
  Ok(())
}