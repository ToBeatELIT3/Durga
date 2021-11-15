// FalseGhost

use std::ffi::CString;
use std::io::{stdin, stdout, Write};
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::os::raw::c_char;
use std::time::Duration;

use futures::{stream, StreamExt};
use termion::color;
use tokio::net::TcpStream;

mod ports;

pub struct CommandModule {
    pub title: String,
    pub command_exec: String,
}

impl CommandModule {
    pub async fn start(self) {
        print!(
            "[*] {} > {}(Y/n){} ",
            self.title,
            color::Fg(color::LightYellow),
            color::Fg(color::Reset)
        );
        stdout().flush().unwrap();
        let mut result = String::new();

        stdin()
            .read_line(&mut result)
            .expect("[-]Error Getting Input");

        if result == "Y\n" || result == "\n" {
            run_command(&self.command_exec);
        }
    }
}

pub fn run_command(my_command: &str) {
    extern "C" {
        fn system(my_command: *const c_char) -> *const c_char;
    }
    let cstring_command = CString::new(my_command).expect("CString::new failed");

    unsafe {
        system(cstring_command.as_ptr());
    }
}


pub fn resolve_target(target: &str) -> Result<IpAddr, Box<dyn std::error::Error>> {
    match target.parse() {
        Ok(actual_ip) => Ok(actual_ip),
        Err(_) => {
            let fakesocketaddr = format!("{}:80", target);
            Ok(fakesocketaddr.to_socket_addrs()?.next().unwrap().ip())
        }
    }
}

pub async fn scan(target: IpAddr, full: bool, target_name: &str) {
    let ports = stream::iter(get_ports(full));
    run_command(format!("touch /tmp/{}.txt", target_name).as_str());

    ports
        .for_each_concurrent(1000, |port| scan_port(target, port, full, target_name))
        .await;
}

async fn scan_port(target: IpAddr, current_port: u16, full: bool, target_name: &str) {
    let socket_address = SocketAddr::new(target, current_port);
    let connection_status =
        tokio::time::timeout(Duration::from_secs(1), TcpStream::connect(&socket_address)).await;

    if full && current_port % 10000 == 0 {
        println!(
            "[-] Status {}{}{}",
            color::Fg(color::LightMagenta),
            current_port,
            color::Fg(color::Reset)
        );
    }

    if let Ok(Ok(_current_stream)) = connection_status {
        run_command(format!("echo {} >> /tmp/{}.txt", current_port, target_name).as_str());
        println! {"[*] OPEN PORT {}{}{}",color::Fg(color::LightGreen), current_port, color::Fg(color::Reset)};
    }
}

fn get_ports(all_ports: bool) -> Box<dyn Iterator<Item = u16>> {
    if all_ports {
        Box::new(1..=u16::MAX)
    } else {
        Box::new(ports::COMMON_PORTS.to_owned().into_iter())
    }
}

pub fn banner() {
    let my_banner = include_str!("../resources/banner.txt");
    println!("{}", my_banner)
}
