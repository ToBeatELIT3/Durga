// FalseGhost

use durga;
use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();
    
    durga::banner();
    durga::command_loop(argv);
}

