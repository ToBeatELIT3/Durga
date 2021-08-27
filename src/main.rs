// FalseGhost

use durga::{self, banner};
use std::time::Duration;

use tokio;
#[macro_use]
extern crate clap;
use clap::App;

mod extensions;
#[tokio::main]
async fn main() {

    banner();
    let yaml = load_yaml!("../resources/cli.yaml");
    let argv = App::from_yaml(yaml).get_matches();
    
    let unresolved_target = argv.value_of("target_ip").unwrap();
 
    let my_target = durga::resolve_target(&unresolved_target.to_string());
    println!("[*] Scanning {} -> {}", argv.value_of("target_ip").unwrap(), durga::resolve_target(&unresolved_target.to_string()).unwrap());

    
    match argv.occurrences_of("full_scan") {
        0 => {
            durga::scan(my_target.unwrap(), false, extensions::COMMON_PORTS.len(), Duration::from_secs(1))
                .await;
        },
        1 => {
            println!("[*] Running Full TCP Scan");
            durga::scan(my_target.unwrap(), true, 1000, Duration::from_secs(1))
                .await;
        }
        _ => ()
    }
}

