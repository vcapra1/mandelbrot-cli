use std::env;
use backend::{util::*, gui, cli};

fn main() {
    // Generate configuration from the arguments
    let config = parse_args(env::args().collect());

    // Start command-line interface or open connection to GUI, depending on 
    // the state of the gui flag.
    if config.gui {
        gui::begin(config);
    } else {
        cli::begin(config);
    }

    println!("Exiting...");
}
