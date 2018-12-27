pub struct Config {
    pub gui: bool,
    pub port: Option<u16>,
}

pub fn parse_args(args: Vec<String>) -> Config {
    // Application config according to the arguments
    let mut config = Config {
        gui: true,
        port: Some(37228)
    };

    // What we're expecting for the next iteration (-1 if anything)
    let mut expecting = -1;

    // Parse command-line arguments
    for arg in args.iter() {
        if expecting == -1 {
            if arg == "-g" || arg == "--gui" {
                // Set the GUI flag
                config.gui = true;
            } else if arg == "--no-gui" {
                // Clear the GUI flag
                config.gui = false;
            } else if arg == "-p" || arg == "--port" {
                // Specifying the port
                expecting = 1;
            }
        } else {
            match expecting {
                1 => {
                    // Parse integer
                    let num = if let Ok(num) = arg.parse::<u32>() {
                        num
                    } else {
                        panic!("Must specify port with --port (-p) flag.");
                    };
                    if num > 65535 {
                        panic!("Port must be less than or equal to 65535.");
                    }
                    config.port = Some(num as u16);
                },
                _ => unreachable!()
            }
        }
    }

    // If expecting something, there's an error
    match expecting {
        1 => panic!("Must specify port with --port (-p) flag."),
        _ => ()
    }

    // If there's no GUI, then the port is None
    if !config.gui {
        config.port = None;
    }

    config
}
