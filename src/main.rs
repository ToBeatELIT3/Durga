//FalseGhost

use durga;

// For CLI Arguments
use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();

    durga::banner();
    durga::command_loop(argv);
}

