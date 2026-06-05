use buddy_rs::init::run_init;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run_init().await
}