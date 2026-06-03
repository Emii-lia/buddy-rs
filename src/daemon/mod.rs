use crate::daemon::reaction::react_to_command;
use colored::*;
use crate::shared::constant::SOCKET_PATH;
use crate::shared::types::CommandEvent;
use std::path::Path;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixListener;

pub mod reaction;

pub async fn run_daemon() -> anyhow::Result<()> {
    control::set_override(true);
    if Path::new(SOCKET_PATH).exists() {
        std::fs::remove_file(SOCKET_PATH)?;
    }

    let listener = UnixListener::bind(SOCKET_PATH)?;
    println!("Buddy daemon is listening...");
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut reader = BufReader::new(stream);
            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(bytes) if bytes == 0 => break,
                    Ok(_) => match serde_json::from_str::<CommandEvent>(&line.trim()) {
                        Ok(event) => {
                            let responses = react_to_command(event);
                            for response in responses {
                                let _ = reader.get_mut().write_all(response.as_bytes()).await;
                                let _ = reader.get_mut().write_all(b"\n").await;
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to parse command event: {}", e);
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to read line: {}", e);
                        break;
                    }
                }
            }
        }); 
    }
}
fn main() {}
