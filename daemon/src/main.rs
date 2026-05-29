use std::path::Path;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::UnixListener;
use shared::constant::SOCKET_PATH;
use shared::types::{CommandEvent};
use crate::reaction::{react_to_command};

pub mod reaction;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
                    Ok(_) => {
                        match serde_json::from_str::<CommandEvent>(&line.trim()) {
                            Ok(event) => {
                                react_to_command(event);
                                let _ = std::io::Write::flush(&mut std::io::stdout());
                            }
                            Err(e) => {
                                eprintln!("Failed to parse command event: {}", e);
                            }
                        }
                    }
                    Err(e) => { 
                        eprintln!("Failed to read line: {}", e);
                        break;
                    },
                }
            }
        });
    }
}
