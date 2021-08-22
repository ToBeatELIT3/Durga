// FalseGhost

pub fn run_extentions(open_port: u16) {
    match open_port {
        22 => {
            println!("ssh lmao");
        },
        80 => {
            println!("web server ooh");
        }
        _ => ()
        
    }
}