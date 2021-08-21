//FalseGhost

use std::io::{self, Write};

// For Reading Files
use std::fs;

// For CLI Arguments
use std::env;

// For Scanning Ports
use std::net::{IpAddr, SocketAddr, TcpStream, Shutdown};
use std::time::Duration;
use std::str::FromStr;

// Get Username
extern crate username;
use username::get_user_name;

// CLI Color Support
extern crate termion;
use termion::color;

fn main() {
    let argv: Vec<String> = env::args().collect();

    banner();
    command_loop(argv);


}

fn command_loop(args: Vec<String>) {
    let my_username = get_user_name().expect("[-] Error Getting Username");

    loop {
        let mut my_command = String::new();
        print!("{}{}@DURGA >{} ", color::Fg(color::Yellow), my_username, color::Fg(color::Reset));
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut my_command)
            .expect("[-] Error Getting Input");

        println!("{}", my_command);

        if my_command == "s" {
            println!("sdfd");
            let my_target = IpAddr::from_str(&args[1])
                .expect("[-] Invalid IP Address");
            
                for x in 1..1001 {

                if scan_port(my_target, x) == 1 {
                    println!("{} OPEN", x);
                }
            }
        }

    }
}

fn scan_port(target_ip: IpAddr, target_port: u16) -> u16 {
    let timeout = Duration::from_secs(1);
    let socket_address = SocketAddr::new(target_ip.clone(), target_port);
    let result = TcpStream::connect_timeout(&socket_address, timeout);

    if let Ok(stream) = result {
        stream.shutdown(Shutdown::Both)
            .expect("[-] Error Closing TCP Stream");
        return 1
    }

    return 0
    
}

fn banner() {
    let my_banner = fs::read_to_string("resources/banner.txt")
        .expect("[-] Error Getting Banner");

    println!("{}{}{}", color::Fg(color::Red), my_banner, color::Fg(color::Reset));
}
