use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};
use tokio::net::UnixStream;
use crate::shared::constant::SOCKET_PATH;
use crate::shared::types::CommandEvent;

pub async fn send_event(event: CommandEvent) -> anyhow::Result<()> {
  let json = serde_json::to_string(&event)?;
  let mut stream = UnixStream::connect(SOCKET_PATH).await?;
  stream.write_all(json.as_bytes()).await?;
  stream.write_all(b"\n").await?;
  
  stream.shutdown().await?;

  let mut reader = BufReader::new(stream);
  let mut line = String::new();
  while reader.read_line(&mut line).await? > 0 {
    print!("{}", line);
    line.clear();
  }

  Ok(())
}