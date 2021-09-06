// FalseGhost

use std::net::{IpAddr, ToSocketAddrs, SocketAddr};
use std::time::Duration;

use tokio;
use tokio::net::TcpStream;

use futures::stream;
use futures::StreamExt;

use std::ffi::CString;
use std::os::raw::c_char;

mod extensions;
mod ports;

pub fn run_command(my_command: &str) {
    extern "C" { fn system(my_command: *const c_char) -> *const c_char; }
    let cstring_command = CString::new(my_command).expect("CString::new failed");

    unsafe {
        system(cstring_command.as_ptr());
    }
}

pub fn resolve_target(target: &String) -> Result<IpAddr, Box<dyn std::error::Error>> {
    match target.parse() {
        Ok(actual_ip) => Ok(actual_ip),
        Err(_) => {
            let fakesocketaddr = format!("{}:80", target);
            Ok(fakesocketaddr.to_socket_addrs()?.next().unwrap().ip())
        }
    }
}

pub async fn scan(target: IpAddr, full: bool, concurrency: usize, timeout: Duration, target_name: &str) {
    let ports = stream::iter(get_ports(full));

    ports
        .for_each_concurrent(concurrency, |port| scan_port(target, port, timeout, full, target_name))
        .await;
}

async fn scan_port(target: IpAddr, current_port: u16, timeout: Duration, full: bool, target_name: &str) {
    let socket_address = SocketAddr::new(target.clone(), current_port);
    let connection_status = tokio::time::timeout(timeout, TcpStream::connect(&socket_address))
    .await;
  
    if full && current_port % 10000  == 0 {
        println!("[-] Status {}", current_port);
    } 

    match connection_status {

        Ok(current_stream_wrapper) => {

            match current_stream_wrapper {

                Ok(_current_stream) => {
                                 
                    run_command(format!("echo {} >> /tmp/{}.txt", current_port, target_name).as_str());
                    println!{"[*] OPEN PORT {}", current_port};
                }
                _ => ()   
            }
        }
        _ => ()
    }
}

fn get_ports(all_ports: bool) -> Box<dyn Iterator<Item = u16>> {
    if all_ports {
        Box::new((1..=u16::MAX).into_iter())
    } else {
        Box::new(ports::COMMON_PORTS.to_owned().into_iter())
    }
}

pub fn banner() {
    let my_banner = include_str!("../resources/banner.txt");
    println!("{}", my_banner)
}
