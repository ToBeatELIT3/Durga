// FalseGhost

use clap::App;
use durga::*;
use termion::color;

mod extensions;

#[tokio::main]
async fn main() {
    banner();

    let yaml = clap::load_yaml!("../resources/cli.yaml");
    let argv = App::from_yaml(yaml).get_matches();

    let unresolved_target = argv.value_of("target_ip").unwrap();
    let my_target = resolve_target(&unresolved_target.to_string());

    println!(
        "[*] Scanning {}{}{} -> {}{}{}",
        color::Fg(color::LightBlue),
        argv.value_of("target_ip").unwrap(),
        color::Fg(color::Reset),
        color::Fg(color::LightBlue),
        resolve_target(&unresolved_target.to_string()).unwrap(),
        color::Fg(color::Reset)
    );

    run_command(format!("rm -f -- /tmp/{}.txt", unresolved_target).as_str());

    match argv.occurrences_of("full_scan") {
        0 => {
            scan(my_target.unwrap(), false, unresolved_target).await;
        }
        1 => {
            println!("[*] Running Full TCP Scan");
            scan(my_target.unwrap(), true, unresolved_target).await;
        }
        _ => (),
    }

    let open_ports_list =
        std::fs::read_to_string(format!("/tmp/{}.txt", unresolved_target)).unwrap();

    for line in open_ports_list.lines() {
        extensions::run_extensions(line.parse::<u16>().unwrap(), unresolved_target);
    }

    run_command(format!("rm -f -- /tmp/{}.txt", unresolved_target).as_str());
}
