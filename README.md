# Durga

## Extensible Recon Tool

```
Don't Try Harder, Enumerate Harder
```

```
Durga is an enumeration tool, designed to make it easy to preform automatic and rapid enumeration that fits with *your* workflow. At it's core is a fast and effiecient 
portscanner, that runs custom "Modules" specific to the open ports of the target.
```

```
I am by no means a great programmer, and there are lots of places where this tool can be 
improved, if you have any neat additions, or pherhaps streamlined systems from the tool, 
feel free to send a Push Request. It would be greatly appreciated :)

    - FalseGhost
```

## Creating Modules


```rust
// src/extensions.rs

pub fn run_extensions(open_port: u16, unresolved_target: &str) {

    // Modules Go Here

    let ffuf_execution = CommandModule { // Create a "CommandModule"
        title: "FFUF".to_string(), // Create a "title", simply name your module
        command_exec: format!( // The "format!" allows us to insert args 
            // The Command that will be run when the module is invoked
            "ffuf -u {}/FUZZ -w directory-list-2.3-medium.txt", unresolved_target 
        )
    };

    // This will iterate for every open port
    match open_port {
        // Create the bracket containing the Port that will invoke your module, in this case: 80
        80 => {
            // Run the Module :D
            ffuf_execution.start();
        }
        _ => ()
        
    }
}

// Its That Easy!

```

## Usage

```
> Compile:
    cargo build --release && sudo mv target/release/durga /bin/durga

(Reccomended Way)
> From Cargo:
    cargo run <target> [FLAGS]
    
    ex: cargo run developer.htb -f

```

## Notes

```
I Recommend you run this tool via "cargo", as it makes it a lot easy to modify your
modules on the fly, if you so need to. Create a bash-shortcut or somthing similar 
to make it easier to call durga from your command line.

```

### TODO

- A "Project System" so you can order and save all command outputs into a Created Directory for the Target

- Refactor Fuction Calls to streamline the Control Flow

- Create executable to put in /bin, that can call "cargo run etc etc" *for* us

- Create executable to put in /bin, that can open the extensions file with vim
