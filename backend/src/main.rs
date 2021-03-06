use backend::{cli, gui, util::*};
use std::env;

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

    // Once we've reached here, either the CLI or GUI has exited
    println!("Exiting...");
}
