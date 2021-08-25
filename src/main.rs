// FalseGhost

use durga;
use std::env;
use std::time::Duration;

use tokio;

#[tokio::main]
async fn main() {
    let argv: Vec<String> = env::args().collect();
    let my_target = durga::resolve_target(&argv[1]);

    durga::scan(my_target.unwrap(), false, 60000, Duration::from_secs(3))
        .await;
}

