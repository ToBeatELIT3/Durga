// FalseGhost

pub fn run_extensions(open_port: u16) {
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