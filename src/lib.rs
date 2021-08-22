// FalseGhost

// Clean Inputs
use std::io::{self, Write};

// For Reading Files
use std::fs;

// For Scanning Ports
use std::net::{IpAddr, Shutdown, SocketAddr, TcpStream};
use std::time::Duration;
use std::str::FromStr;

// CLI Color Support
extern crate termion;
use termion::color;

// Get Username
extern crate username;
use username::get_user_name;

use crate::extensions::run_extentions;
use crate::ports::MOST_COMMON_PORTS;

// Get Common Ports List
mod ports;

// Port Extensions
mod extensions;

// --------------------

pub fn command_loop(args: Vec<String>) {
    let my_username = get_user_name().expect("[-] Error Getting Username");

    // Main Command Loop
    loop {

        // Get Command Input
        let mut my_command = String::new();
        print!("{}{}@DURGA >{} ", color::Fg(color::Yellow), my_username, color::Fg(color::Reset));
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut my_command)
            .expect("[-] Error Getting Input");

        // Compare Command Input to Commands
        match my_command.as_ref() {

            // "s" for Port Scan
            "s\n" => {
                println!("[*] Scanning {}", &args[1]);
                let my_target = IpAddr::from_str(&args[1])
                    .expect("[-] Invalid IP Address");
            
                // Iterate through list of Common Ports defined in ports.rs
                for x in MOST_COMMON_PORTS {

                    if scan_port(my_target, x) == 1 {
                        println!("[*] {} OPEN", x);
                    }
                }
            },

            "fs\n" => {
                println!("[*] Scanning {}", &args[1]);
                let my_target = IpAddr::from_str(&args[1])
                    .expect("[-] Invalid IP Address");
            
                // Iterate through list of ALL Ports
                for x in 1..=65535 {

                    if scan_port(my_target, &x) == 1 {
                        println!("[*] {} OPEN", x);
                    }
                }
            },

            "es\n" => {
                println!("[*] Scanning {}", &args[1]);
                let my_target = IpAddr::from_str(&args[1])
                    .expect("[-] Invalid IP Address");
            
                // Iterate through list of ALL Ports
                for x in MOST_COMMON_PORTS {

                    if scan_port(my_target, x) == 1 {
                        println!("[*] {} OPEN", x);
                        run_extentions(*x);
                    }
                }
            },
            
            "fes\n" => {
                println!("[*] Scanning {}", &args[1]);
                let my_target = IpAddr::from_str(&args[1])
                    .expect("[-] Invalid IP Address");
            
                // Iterate through list of ALL Ports
                for x in 1..=65535 {

                    if scan_port(my_target, &x) == 1 {
                        println!("[*] {} OPEN", x);
                        run_extentions(x);
                    }
                }
            },

            "q\n" => std::process::exit(1),

            // Help Message
            "h\n" => {
                println!("
                    Durga Help:
                    s : scan common ports
                    fs : full port scan
                    es : scan common ports with extensions
                    fes : scan all ports with extensions"
                );
            },

            _ => ()
        }
    }
}

pub fn scan_port(target_ip: IpAddr, target_port: &u16) -> u16 {
    let timeout = Duration::from_secs(1);
    let socket_address = SocketAddr::new(target_ip.clone(), *target_port);
    let result = TcpStream::connect_timeout(&socket_address, timeout);

    if let Ok(stream) = result {
        stream.shutdown(Shutdown::Both)
            .expect("[-] Error Closing TCP Stream");

        return 1
    }

    return 0   
}

pub fn banner() {
    let my_banner = fs::read_to_string("resources/banner.txt")
        .expect("[-] Error Getting Banner");

    println!("{}{}{}", color::Fg(color::Red), my_banner, color::Fg(color::Reset));
}
