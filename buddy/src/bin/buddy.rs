pub mod buddyd;

use init::run_init;

fn main() -> anyhow::Result<()> {
  run_init()
}