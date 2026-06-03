use buddy_rs::daemon::run_daemon;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run_daemon().await
}