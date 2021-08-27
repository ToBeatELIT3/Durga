// FalseGhost

use std::net::{IpAddr, ToSocketAddrs, SocketAddr};
use std::time::Duration;
use std::fs;

use tokio;
use tokio::net::TcpStream;

use futures::stream;
use futures::StreamExt;

mod extensions;

pub fn resolve_target(target: &String) -> Result<IpAddr, Box<dyn std::error::Error>> {
    match target.parse() {
        Ok(actual_ip) => Ok(actual_ip),
        Err(_) => {
            let fakesocketaddr = format!("{}:80", target);
            Ok(fakesocketaddr.to_socket_addrs()?.next().unwrap().ip())
        }
    }
}

pub async fn scan(target: IpAddr, full: bool, concurrency: usize, timeout: Duration) {
    let ports = stream::iter(get_ports(full));

    ports
        .for_each_concurrent(concurrency, |port| scan_port(target, port, timeout, full))
        .await;
}

async fn scan_port(target: IpAddr, current_port: u16, timeout: Duration, full: bool) {
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

                    println!{"[*] OPEN PORT {}", current_port};
                    extensions::run_extensions(current_port);
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
        Box::new(extensions::COMMON_PORTS.to_owned().into_iter())
    }
}

pub fn banner() {
    let my_banner = fs::read_to_string("resources/banner.txt")
        .expect("[-] Error Getting Banner");

    println!("{}", my_banner)
}
