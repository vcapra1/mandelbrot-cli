use std::fs;
use std::io::{self, prelude::*};
use std::path::Path;

use crate::colors::*;
use crate::image::*;
use crate::math::*;
use crate::render::*;
use crate::util::{self, *};

#[derive(Copy, Clone)]
enum Field {
    Iterations,
    Width,
    Height,
    CenterX,
    CenterY,
    Radius,
    Supersampling,
    ColorFunc,
}

#[derive(Clone)]
struct Data {
    render: Render,
    params: Parameters,
    colorfunc: ColorFunction,
}

#[derive(Clone)]
enum State {
    Prompt(Data),
    Set(Data, Field, String),
    Get(Data, Field),
    Render(Data),
    Export(Data, String),
    SaveConfig(Data, String), // TODO: add load config option
    Dead,
}

const FIELDS: [(&str, Field); 8] = [
    ("iterations", Field::Iterations),
    ("width", Field::Width),
    ("height", Field::Height),
    ("center:x", Field::CenterX),
    ("center:y", Field::CenterY),
    ("radius", Field::Radius),
    ("supersampling", Field::Supersampling),
    ("colorfunc", Field::ColorFunc),
];

impl State {
    //////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////// State execution loop //////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////
    fn execute(self) -> State {
        match self {
            //////////////////////////////////////////////////////
            /////////////////////// Prompt /////////////////////// 
            //////////////////////////////////////////////////////
            State::Prompt(data) => {
                // Print a prompt
                print!("> ");
                io::stdout().flush().unwrap();

                // Read user input
                let reader = io::stdin();
                let mut input = String::new();
                reader.read_line(&mut input).unwrap();

                // If the input is empty, exit
                if input == "" {
                    println!("exit");
                    return State::Dead;
                }

                // Remove whitespace (including newlines) from beginning and end of input
                input = input.trim().to_string();

                // Parse the input
                if input == "" {
                    // Input was all whitespace, user probably just hit enter, show help hint
                    println!("Enter `help` for possible commands");
                    State::Prompt(data)
                } else if input == "quit" || input == "exit" {
                    // Enter a dead state, causing the program to stop
                    State::Dead
                } else if input.starts_with("set ") {
                    // Requesting to set a field

                    // Get the part of the input after the "set", and trim whitespace
                    let input = (&input[4..]).to_string().trim().to_string();

                    // The first word is the name of the field
                    let first_word = input.split_whitespace().next().unwrap().to_string();

                    // Find the Field enum that matches
                    for (string, field) in FIELDS.iter() {
                        if first_word == *string {
                            // Remove the first part of the string, to isolate the provided value
                            let n = string.len() + 1;

                            // Ensure the string is long enough, to prevent illegal index panics
                            if input.len() <= n - 1 {
                                println!("Must specify a value.");
                                return State::Prompt(data);
                            }

                            return State::Set(
                                data,
                                field.clone(),
                                (&input[n..]).to_string().trim().to_string(),
                            );
                        }
                    }

                    println!(
                        "{} is not a valid field.",
                        input.split_whitespace().next().unwrap()
                    );
                    State::Prompt(data)
                } else if input.starts_with("get ") {
                    // Requesting to read the value of a field

                    // Get the part of the input after the "get", and trim whitesapce
                    let input = (&input[4..]).to_string().trim().to_string();

                    // Find the Field enum that matches
                    for (string, field) in FIELDS.iter() {
                        if input == *string {
                            return State::Get(data, field.clone());
                        }
                    }

                    // If none match, the field requested must not exist, so print error and return
                    // to a prompt
                    println!(
                        "{} is not a valid field.",
                        input.split_whitespace().next().unwrap()
                    );
                    State::Prompt(data)
                } else if input == "render" {
                    State::Render(data)
                } else if input.starts_with("export ") || input.starts_with("saveconfig ") {
                    let path = Path::new(if input.starts_with("export ") {
                        &input[7..]
                    } else {
                        &input[11..]
                    });

                    let valid = util::can_make_file_here(&path);

                    if !valid {
                        State::Prompt(data)
                    } else if input.starts_with("export ") {
                        State::Export(data, path.display().to_string())
                    } else {
                        State::SaveConfig(data, path.display().to_string())
                    }
                } else if input == "help" {
                    // Print help information
                    show_help();

                    State::Prompt(data)
                } else if input == "get" {
                    println!("Must specify a field.");

                    State::Prompt(data)
                } else if input == "set" {
                    println!("Must specify a field and a value.");

                    State::Prompt(data)
                } else if input == "export" || input == "saveconfig" {
                    println!("Must specify a path.");

                    State::Prompt(data)
                } else {
                    println!("{} is not a valid command.", input);
                    State::Prompt(data)
                }
            }
            //////////////////////////////////////////////////////
            ///////////////////////// Get //////////////////////// 
            //////////////////////////////////////////////////////
            State::Get(data, field) => {
                // Get the specified field of the render
                match field {
                    Field::Iterations => println!("{}", data.params.max_iter),
                    Field::Width => println!("{}", data.params.image_size.0),
                    Field::Height => println!("{}", data.params.image_size.1),
                    Field::CenterX => println!("{}", data.params.center.0),
                    Field::CenterY => println!("{}", data.params.center.1),
                    Field::Radius => println!("{}", data.params.radius),
                    Field::Supersampling => println!("{}", data.params.supersampling),
                    Field::ColorFunc => println!("{}", data.colorfunc.info()),
                };

                State::Prompt(data)
            }
            //////////////////////////////////////////////////////
            ///////////////////////// Set //////////////////////// 
            //////////////////////////////////////////////////////
            State::Set(mut data, field, value) => {
                // Set the specified field of the render
                match field {
                    Field::Iterations => {
                        match value.parse::<u32>() {
                            Ok(value) => data.params.max_iter = value,
                            Err(_) => println!("Invalid value: {}", value),
                        };
                    }
                    Field::Width => {
                        match value.parse::<u32>() {
                            Ok(value) => data.params.image_size.0 = value,
                            Err(_) => println!("Invalid value: {}", value),
                        };
                    }
                    Field::Height => {
                        match value.parse::<u32>() {
                            Ok(value) => data.params.image_size.1 = value,
                            Err(_) => println!("Invalid value: {}", value),
                        };
                    }
                    Field::CenterX => {
                        match value.parse::<Real>() {
                            Ok(value) => data.params.center.0 = value,
                            Err(_) => println!("Invalid value: {}", value),
                        };
                    }
                    Field::CenterY => {
                        match value.parse::<Real>() {
                            Ok(value) => data.params.center.1 = value,
                            Err(_) => println!("Invalid value: {}", value),
                        };
                    }
                    Field::Radius => {
                        match value.parse::<Real>() {
                            Ok(value) => data.params.radius = value,
                            Err(_) => println!("Invalid value: {}", value),
                        };
                    }
                    Field::Supersampling => {
                        match value.parse::<u32>() {
                            Ok(value) => data.params.supersampling = value,
                            Err(_) => println!("Invalid value: {}", value),
                        };
                    }
                    Field::ColorFunc => match value.parse::<ColorFunction>() {
                        Ok(value) => data.colorfunc = value,
                        Err(e) => println!("Invalid value: {} ({})", value, e),
                    },
                };

                State::Prompt(data)
            }
            //////////////////////////////////////////////////////
            /////////////////////// Render /////////////////////// 
            //////////////////////////////////////////////////////
            State::Render(mut data) => {
                // Recalculate render pixels if necessary
                data.render.recalc(&data.params);

                // Render
                let job = data.render.clone().run();
                match job.join_with_progress() {
                    Ok((render, _)) => {
                        // Update the render and return
                        State::Prompt(Data { render, ..data })
                    }
                    Err(msg) => {
                        // Print error message and return
                        println!("Error rendering: {}", msg);
                        State::Prompt(data)
                    }
                }
            }
            //////////////////////////////////////////////////////
            /////////////////////// Export /////////////////////// 
            //////////////////////////////////////////////////////
            State::Export(data, path) => {
                // Create a new image
                let image = Image::new(&data.render, data.colorfunc.clone());

                // Export the image
                match image.export(path) {
                    Ok(_) => println!("Success!"),
                    Err(_) => println!("There was an error saving the image"),
                };

                // Return to the prompt
                State::Prompt(data)
            }
            //////////////////////////////////////////////////////
            ///////////////////// Save Config //////////////////// 
            //////////////////////////////////////////////////////
            State::SaveConfig(data, path) => {
                // Create the config string
                let mut config = String::new();

                // Add each configuration parameter
                let params = data.params;
                config.push_str(&format!("set iterations {}\n", params.max_iter));
                config.push_str(&format!("set width {}\n", params.image_size.0));
                config.push_str(&format!("set height {}\n", params.image_size.1));
                config.push_str(&format!("set center:x {}\n", params.center.0));
                config.push_str(&format!("set center:y {}\n", params.center.1));
                config.push_str(&format!("set radius {}\n", params.radius));
                config.push_str(&format!("set supersampling {}\n", params.supersampling));
                config.push_str(&format!(
                    "set colorfunc {}\n",
                    data.colorfunc.info()
                ));

                // Save the string to the file
                match fs::write(path, config) {
                    Ok(_) => println!("Configuration saved."),
                    Err(e) => println!("Couldn't save file: {:?}", e),
                };

                // Return to the prompt
                State::Prompt(Data { params, ..data })
            }
            //////////////////////////////////////////////////////
            //////////////////////// Dead //////////////////////// 
            //////////////////////////////////////////////////////
            State::Dead => State::Dead,
        }
    }
}

pub fn begin(_config: Config) {
    let render = Render::default();
    let params = render.params.clone();
    let colorfunc = ColorFunction::greyscale();
    let mut state: State = State::Prompt(Data { render, params, colorfunc });

    // CLI Loop
    loop {
        // Execute the current state's action
        state = state.execute();

        if let State::Dead = state {
            break;
        }
    }
    // End CLI Loop
}

fn show_help() {
    let mut help = String::from("Mandelbrot Set Explorer: Help Message\n\n");
    help.push_str("  Commands:\n");
    help.push_str("    get <field>            Get the current value of a field\n");
    help.push_str("    set <field> <value>    Set the value of field to the provided value\n");
    help.push_str("    render                 Render the image with the current configuration\n");
    help.push_str(
        "    export <path>          Export the rendered image to the provided path, if valid\n",
    );
    help.push_str("    saveconfig <path>      Save the current configuration to a file\n");
    help.push_str("    quit, exit             Exit the program\n\n");
    help.push_str("  Fields:\n");
    help.push_str(
        "    iterations    (positive integer)  The maximum number of iterations to compute\n",
    );
    help.push_str("    width         (positive integer)  Output image width\n");
    help.push_str("    height        (positive integer)  Output image height\n");
    help.push_str("    center:x      (floating-point)    Center x coordinate of window\n");
    help.push_str("    center:y      (floating-point)    Center y coordinate of window\n");
    help.push_str(
        "    radius        (floating-point)    Radius of the window (in the smaller dimension)\n",
    );
    help.push_str("    supersampling (positive integer)  Factor (in both dimensions) to increase number of pixels for computation\n");
    help.push_str(
        "    colorfunc     (string)            The color function to use when exporting image\n\n",
    );
    help.push_str("  Color Functions:\n");
    help.push_str(
        "    greyscale            Black center, value determined by number of iterations\n",
    );
    help.push_str(
        "    rgreyscale           White center, value determined by number of iterations\n",
    );
    help.push_str(
        "    color(shift, scale)  Colorized, with given shift (pos. int.) and scale (float)\n",
    );
    help.push_str(
        "    red(shift, scale)    Red colorized, with given shift (pos. int.) and scale (float)\n",
    );
    println!("{}", help);
}
