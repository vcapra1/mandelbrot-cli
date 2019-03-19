use std::path::Path;
use std::io::{self, prelude::*};

// Holds information about the program configuration for this execution,
// including whether or not to run in GUI mode and what port to use to
// communicate to the GUI if applicable.
pub struct Config {
    pub gui: bool,
    pub port: Option<u16>,
}

// Parse the command line arguments into a Config instance
pub fn parse_args(args: Vec<String>) -> Config {
    // Application config according to the arguments
    let mut config = Config {
        gui: false,
        port: None,
    };

    // What we're expecting for the next iteration (-1 if anything)
    let mut expecting = -1;

    println!("{:?}", args);

    // Parse command-line arguments
    for arg in args.iter().skip(1) {
        if expecting == -1 {
            // Not expecting anything in particular, so look for flags
            if arg == "-g" || arg == "--gui" {
                // Set the GUI flag
                config.gui = true;
            } else if arg == "-p" || arg == "--port" {
                // Specifying the port
                expecting = 1;
            } else {
                panic!(format!("Unknown option: {}", arg));
            }
        } else {
            match expecting {
                1 => {
                    // Expecting to find a port

                    // Parse integer
                    let num = if let Ok(num) = arg.parse::<u32>() {
                        num
                    } else {
                        println!("Arg: {}", arg);
                        panic!("Must specify port with --port (-p) flag.");
                    };

                    // Make sure the port is valid
                    if num > 65535 {
                        panic!("Port must be less than or equal to 65535.");
                    }

                    // Save specified port
                    config.port = Some(num as u16);

                    // Nothing else to expect
                    expecting = -1;
                }
                _ => unreachable!(),
            }
        }
    }

    // If expecting something, there's an error
    match expecting {
        1 => panic!("Must specify port with --port (-p) flag."),
        _ => (),
    }

    // If there's no GUI, then the port is None
    if !config.gui {
        config.port = None;
    }

    config
}

pub fn can_make_file_here(path: &Path) -> bool {
    if path.exists() {
        if path.is_dir() {
            // If it's a directory, this won't work
            println!("\"{}\" is a directory.", path.display());
            false
        } else if path.is_file() {
            print!(
                "\"{}\" already exists.  Do you want to overwrite? [y/N] ",
                path.display()
            );
            io::stdout().flush().unwrap();

            let mut conf = String::new();
            io::stdin().read_line(&mut conf).unwrap();
            conf = conf.trim().to_string().to_lowercase();

            if conf == "y" || conf == "yes" {
                true
            } else {
                println!("No action taken.");
                false
            }
        } else {
            unreachable!()
        }
    } else {
        // The file doesn't exist, see if the parent does
        let parent = path.parent().unwrap();

        if parent.exists() {
            // Make sure the parent is a dir and not a file
            if parent.is_file() {
                // This won't work
                println!("Invalid path: \"{}\" is a file.", parent.display());
                false
            } else {
                // We're good
                true
            }
        } else {
            // The parent doesn't exist, this won't work
            println!("No such directory: \"{}\".", parent.display());
            true
        }
    }
}
