// FalseGhost

use durga::CommandModule;

pub fn run_extensions(open_port: u16, unresolved_target: &str) {

    // Modules Go Here
    
    // Examples:
    let smb_nse = CommandModule{title: "SMB NSE".to_string(), command_exec: format!("sudo nmap -sV -p 445 --script=smb-enum-shares.nse,smb-enum-users.nse {}", unresolved_target)};
    let ldap_nse = CommandModule{title: "LDAP NSE".to_string(), command_exec: format!("nmap -sC -p 636 --script 'ldap* and not brute' {}", unresolved_target)};

    match open_port {
        445 => {
            smb_nse.start()
        }
        636 => {
            ldap_nse.start()
        }
        _ => ()
        
    }
}
