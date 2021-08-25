// FalseGhost

use durga;
use std::env;
use std::time::Duration;

use tokio;
mod ports;
#[tokio::main]
async fn main() {
    let argv: Vec<String> = env::args().collect();
    let my_target = durga::resolve_target(&argv[1]);

    durga::scan(my_target.unwrap(), false, ports::COMMON_PORTS.len(), Duration::from_secs(1))
        .await;
}

