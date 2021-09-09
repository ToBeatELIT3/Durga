// FalseGhost

use durga::CommandModule;

pub fn auto_run(unresolved_target: &str) {
    let nmap_standard = CommandModule{title: "NMAP SCAN".to_string(), command_exec: format!("sudo nmap -vv -sC -sV -p- {}", unresolved_target)};
    let nmap_short = CommandModule{title: "NMAP SHORT".to_string(), command_exec: format!("sudo nmap -vv -sC -sV {}", unresolved_target)};
    let nmap_vuln = CommandModule{title: "NMAP VULN".to_string(), command_exec: format!("sudo nmap {} --script vuln", unresolved_target)};

    nmap_standard.start();
    nmap_short.start();
    nmap_vuln.start();
}

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
