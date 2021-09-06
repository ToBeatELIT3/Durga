// FalseGhost

use std::io;
use std::io::Write; 

use std::ffi::CString;
use std::os::raw::c_char;

pub fn run_command(my_command: &str) {
    extern "C" { fn system(my_command: *const c_char) -> *const c_char; }
    let cstring_command = CString::new(my_command).expect("CString::new failed");

    unsafe {
        system(cstring_command.as_ptr());
    }
}


pub struct CommandModule {
    title: String,
    command_exec: String,
}

impl CommandModule {
    fn start(self) {
        print!("DURGA> ({}) Y/n >", self.title);
        io::stdout().flush().unwrap();
        let mut result = String::new();
    
        io::stdin().read_line(&mut result)
            .expect("[-]Error Getting Input");

        match result.as_str() {
            "Y" => {
                run_command(&self.command_exec);
            }

            _ => ()
        }
    }
}

#[allow(dead_code)]
pub fn run_extensions(open_port: u16, unresolved_target: &str) {

    let ffuf_execution = CommandModule{title: "FFUF".to_string(), command_exec: format!("ffuf -u {} -w /home/tobeatelite/HTB/Seclists/Discovery/Web-Content/directory-list-2.3-medium.txt", unresolved_target)};

    match open_port {
        22 => {
        
            println!("[*]lmaoo them mfs using ssh 💀 ");       
        },
        80 => {
            ffuf_execution.start();
        }
        _ => ()
        
    }
}
