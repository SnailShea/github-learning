use tracing::info;
use tracing_subscriber::fmt::init;

fn main() {
    init();
    let hello = hello();
    info!("{hello}");
}

fn hello() -> &'static str {
    "Hello, world!"
}