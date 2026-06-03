use buddy_rs::init::run_init;
use buddy_rs::shared::style::wrap_in_bubble;

fn main() -> anyhow::Result<()> {
    println!("{}", wrap_in_bubble(
        "Hello from Buddy!",
        "(ﾉ◕ヮ◕)ﾉ*.✧"
    ));
    run_init()
}