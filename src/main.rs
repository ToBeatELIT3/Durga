// FalseGhost

use durga;
use std::env;
use std::time::Duration;

use tokio;
#[macro_use]
extern crate clap;
use clap::App;

mod extensions;

#[tokio::main]
async fn main() {

    let yaml = load_yaml!("cli.yaml");
    let argv = App::from_yaml(yaml).get_matches();
    
    let unresolved_target = argv.value_of("target_ip").unwrap();
    let full_scan = argv.occurrences_of("full-scan");

    let my_target = durga::resolve_target(&unresolved_target.to_string());

    println!("[*] Scanning {} -> {}", argv.value_of("target_ip").unwrap(), durga::resolve_target(&unresolved_target.to_string()).unwrap());

    durga::scan(my_target.unwrap(), false, extensions::COMMON_PORTS.len(), Duration::from_secs(1))
        .await;
}

