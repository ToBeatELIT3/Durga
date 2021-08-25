// FalseGhost

use std::process::Command;

pub fn run_extentions(open_port: u16, arguments: &Vec<String>) {
    match open_port {
        22 => {
            println!("hi ssh");        
        },
        80 => {
            println!("web server ooh");
        }
        _ => ()
        
    }
}