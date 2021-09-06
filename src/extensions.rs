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
        print!("[*] {} > (Y/n) ", self.title);
        io::stdout().flush().unwrap();
        let mut result = String::new();
    
        io::stdin().read_line(&mut result)
            .expect("[-]Error Getting Input");
        
        if result == "Y\n" || result == "\n" {
            run_command(&self.command_exec);
        } 
    }
}

#[allow(dead_code)]
pub fn run_extensions(open_port: u16, unresolved_target: &str) {
    match open_port {
        22 => {
        
            ();       
        },
        80 => {
            let ffuf_execution = CommandModule {
                title: "FFUF".to_string(),
                command_exec: format!(
                    "ffuf -u {}/FUZZ -w /home/tobeatelite/HTB/SecLists/Discovery/Web-Content/directory-list-2.3-medium.txt", unresolved_target
                )};
            
                ffuf_execution.start();
        }
        _ => ()
        
    }
}
