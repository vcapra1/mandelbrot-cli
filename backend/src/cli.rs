use std::fs;
use std::io::{self, prelude::*};
use std::path::Path;

use crate::colors::*;
use crate::image::*;
use crate::math::*;
use crate::render::*;
use crate::util::Config;

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
enum State {
    Prompt(Render, Parameters),
    Set(Render, Parameters, Field, String),
    Get(Render, Parameters, Field),
    Render(Render, Parameters),
    Export(Render, Parameters, String),
    SaveConfig(Render, Parameters, String), // TODO: add load config option
    Dead,
}

impl State {
    fn execute(self) -> State {
        match self {
            State::Prompt(render, params) => {
                // Print a prompt
                print!("> ");
                io::stdout().flush().unwrap();

                // Read user input
                let reader = io::stdin();
                let mut input = String::new();
                reader.read_line(&mut input).unwrap();
                if input == "" {
                    println!("exit");
                    return State::Dead;
                }

                input = input.trim().to_string();

                // Parse the input
                if input == "" {
                    println!("Enter `help` for possible commands");
                    State::Prompt(render, params)
                } else if input == "quit" || input == "exit" {
                    State::Dead
                } else if input.starts_with("set ") {
                    let input = (&input[4..]).to_string().trim().to_string();
                    let first_word = input.split_whitespace().next().unwrap().to_string();

                    let field_strs = vec![
                        "iterations",
                        "width",
                        "height",
                        "center:x",
                        "center:y",
                        "radius",
                        "supersampling",
                        "colorfunc",
                    ];
                    let fields = vec![
                        Field::Iterations,
                        Field::Width,
                        Field::Height,
                        Field::CenterX,
                        Field::CenterY,
                        Field::Radius,
                        Field::Supersampling,
                        Field::ColorFunc,
                    ];

                    for (idx, field) in field_strs.iter().enumerate() {
                        if first_word == *field {
                            let n = field.len() + 1;
                            if input.len() <= n - 1 {
                                println!("Must specify a value.");
                                return State::Prompt(render, params);
                            }
                            return State::Set(
                                render,
                                params,
                                fields[idx].clone(),
                                (&input[n..]).to_string().trim().to_string(),
                            );
                        }
                    }

                    println!(
                        "{} is not a valid field.",
                        input.split_whitespace().next().unwrap()
                    );
                    State::Prompt(render, params)
                } else if input.starts_with("get ") {
                    let input = (&input[4..]).to_string().trim().to_string();

                    let field_strs = vec![
                        "iterations",
                        "width",
                        "height",
                        "center:x",
                        "center:y",
                        "radius",
                        "supersampling",
                        "colorfunc",
                    ];
                    let fields = vec![
                        Field::Iterations,
                        Field::Width,
                        Field::Height,
                        Field::CenterX,
                        Field::CenterY,
                        Field::Radius,
                        Field::Supersampling,
                        Field::ColorFunc,
                    ];

                    for (idx, field) in field_strs.iter().enumerate() {
                        if input == *field {
                            return State::Get(render, params, fields[idx].clone());
                        }
                    }

                    println!(
                        "{} is not a valid field.",
                        input.split_whitespace().next().unwrap()
                    );
                    State::Prompt(render, params)
                } else if input == "render" {
                    State::Render(render, params)
                } else if input.starts_with("export ") || input.starts_with("saveconfig ") {
                    let path = Path::new(if input.starts_with("export ") {
                        &input[7..]
                    } else {
                        &input[11..]
                    });

                    if path.exists() {
                        if path.is_dir() {
                            // If it's a directory, this won't work
                            println!("\"{}\" is a directory.", path.display());
                            State::Prompt(render, params)
                        } else if path.is_file() {
                            print!(
                                "\"{}\" already exists.  Do you want to overwrite? [Y/n] ",
                                path.display()
                            );
                            io::stdout().flush().unwrap();

                            let mut conf = String::new();
                            reader.read_line(&mut conf).unwrap();
                            conf = conf.trim().to_string().to_lowercase();

                            if conf == "y" || conf == "yes" {
                                if input.starts_with("export ") {
                                    State::Export(render, params, path.display().to_string())
                                } else {
                                    State::SaveConfig(render, params, path.display().to_string())
                                }
                            } else {
                                // Don't do anything
                                println!("No action taken.");
                                State::Prompt(render, params)
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
                                State::Prompt(render, params)
                            } else {
                                // We're good
                                if input.starts_with("export ") {
                                    State::Export(render, params, path.display().to_string())
                                } else {
                                    State::SaveConfig(render, params, path.display().to_string())
                                }
                            }
                        } else {
                            // The parent doesn't exist, this won't work
                            println!("No such directory: \"{}\".", parent.display());
                            State::Prompt(render, params)
                        }
                    }
                } else if input == "help" {
                    // Print help information
                    show_help();

                    State::Prompt(render, params)
                } else if input == "get" {
                    println!("Must specify a field.");

                    State::Prompt(render, params)
                } else if input == "set" {
                    println!("Must specify a field and a value.");

                    State::Prompt(render, params)
                } else if input == "export" || input == "saveconfig" {
                    println!("Must specify a path.");

                    State::Prompt(render, params)
                } else {
                    println!("{} is not a valid command.", input);
                    State::Prompt(render, params)
                }
            }
            State::Get(render, params, field) => {
                // Get the specified field of the render
                match field {
                    Field::Iterations => println!("{}", params.max_iter),
                    Field::Width => println!("{}", params.image_size.0),
                    Field::Height => println!("{}", params.image_size.1),
                    Field::CenterX => println!("{}", params.center.0),
                    Field::CenterY => println!("{}", params.center.1),
                    Field::Radius => println!("{}", params.radius),
                    Field::Supersampling => println!("{}", params.supersampling),
                    Field::ColorFunc => println!("{}", params.colorfunction.info()),
                };

                State::Prompt(render, params)
            }
            State::Set(render, mut params, field, value) => {
                // Set the specified field of the render
                match field {
                    Field::Iterations => {
                        match value.parse::<u32>() {
                            Ok(value) => params.max_iter = value,
                            Err(_) => println!("Invalid value: {}", value),
                        };
                    }
                    Field::Width => {
                        match value.parse::<u32>() {
                            Ok(value) => params.image_size.0 = value,
                            Err(_) => println!("Invalid value: {}", value),
                        };
                    }
                    Field::Height => {
                        match value.parse::<u32>() {
                            Ok(value) => params.image_size.1 = value,
                            Err(_) => println!("Invalid value: {}", value),
                        };
                    }
                    Field::CenterX => {
                        match value.parse::<Real>() {
                            Ok(value) => params.center.0 = value,
                            Err(_) => println!("Invalid value: {}", value),
                        };
                    }
                    Field::CenterY => {
                        match value.parse::<Real>() {
                            Ok(value) => params.center.1 = value,
                            Err(_) => println!("Invalid value: {}", value),
                        };
                    }
                    Field::Radius => {
                        match value.parse::<Real>() {
                            Ok(value) => params.radius = value,
                            Err(_) => println!("Invalid value: {}", value),
                        };
                    }
                    Field::Supersampling => {
                        match value.parse::<u32>() {
                            Ok(value) => params.supersampling = value,
                            Err(_) => println!("Invalid value: {}", value),
                        };
                    }
                    Field::ColorFunc => match value.parse::<ColorFunction>() {
                        Ok(value) => params.colorfunction = value,
                        Err(e) => println!("Invalid value: {} ({})", value, e),
                    },
                };

                State::Prompt(render, params)
            }
            State::Render(mut render, params) => {
                // Recalculate render pixels if necessary
                render.recalc(&params);

                // Render
                match render.run(true) {
                    Ok(_) => println!("Success!"),
                    Err(e) => println!("There was an error: {}", e),
                };

                // Return to prompt
                State::Prompt(render, params)
            }
            State::Export(render, params, path) => {
                // Create a new image
                let image = Image::new(&render, params.colorfunction.clone());

                // Export the image
                match image.export(path) {
                    Ok(_) => println!("Success!"),
                    Err(_) => println!("There was an error saving the image"),
                };

                // Return to the prompt
                State::Prompt(render, params)
            }
            State::SaveConfig(render, params, path) => {
                // Create the config string
                let mut config = String::new();

                // Add each configuration parameter
                config.push_str(&format!("set iterations {}\n", params.max_iter));
                config.push_str(&format!("set width {}\n", params.image_size.0));
                config.push_str(&format!("set height {}\n", params.image_size.1));
                config.push_str(&format!("set center:x {}\n", params.center.0));
                config.push_str(&format!("set center:y {}\n", params.center.1));
                config.push_str(&format!("set radius {}\n", params.radius));
                config.push_str(&format!("set supersampling {}\n", params.supersampling));
                config.push_str(&format!(
                    "set colorfunc {}\n",
                    params.colorfunction.info()
                ));

                // Save the string to the file
                match fs::write(path, config) {
                    Ok(_) => println!("Configuration saved."),
                    Err(e) => println!("Couldn't save file: {:?}", e),
                };

                // Return to the prompt
                State::Prompt(render, params)
            }
            State::Dead => State::Dead,
        }
    }
}

pub fn begin(_config: Config) {
    let render = Render::default();
    let params = render.params.clone();
    let mut state: State = State::Prompt(render, params);

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
