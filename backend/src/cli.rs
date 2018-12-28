use std::io::{self, prelude::*};
use std::path::Path;

use crate::util::Config;
use crate::render::*;
use crate::image::*;
use crate::colors::*;
use crate::math::*;

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
                    println!("");
                    return State::Dead;
                }
                
                input = input.trim().to_string();

                // Parse the input
                if input == "" {
                    State::Prompt(render, params)
                } else if input == "quit" || input == "exit" {
                    State::Dead
                } else if input.starts_with("set ") {
                    let input = (&input[4..]).to_string().trim().to_string();
                    let first_word = input.split_whitespace().next().unwrap().to_string();

                    let field_strs = vec!["iterations", "width", "height", "center:x", "center:y", "radius", "supersampling", "colorfunc"];
                    let fields = vec![Field::Iterations, Field::Width, Field::Height, Field::CenterX, Field::CenterY, Field::Radius, Field::Supersampling, Field::ColorFunc];

                    for (idx, field) in field_strs.iter().enumerate() {
                        if first_word == *field {
                            let n = field.len() + 1;
                            return State::Set(render, params, fields[idx].clone(), (&input[n..]).to_string().trim().to_string());
                        }
                    }

                    println!("{} is not a valid field.", input.split_whitespace().next().unwrap());
                    State::Prompt(render, params)
                } else if input.starts_with("get ") {
                    let input = (&input[4..]).to_string().trim().to_string();

                    let field_strs = vec!["iterations", "width", "height", "center:x", "center:y", "radius", "supersampling", "colorfunc"];
                    let fields = vec![Field::Iterations, Field::Width, Field::Height, Field::CenterX, Field::CenterY, Field::Radius, Field::Supersampling, Field::ColorFunc];

                    for (idx, field) in field_strs.iter().enumerate() {
                        if input == *field {
                            return State::Get(render, params, fields[idx].clone());
                        }
                    }

                    println!("{} is not a valid field.", input.split_whitespace().next().unwrap());
                    State::Prompt(render, params)
                } else if input == "render" {
                    State::Render(render, params)
                } else if input.starts_with("export ") {
                    let path = Path::new(&input[7..]);

                    if path.exists() {
                        if path.is_dir() {
                            // If it's a directory, this won't work
                            println!("\"{}\" is a directory.", path.display());
                            State::Prompt(render, params)
                        } else if path.is_file() {
                            print!("\"{}\" already exists.  Do you want to overwrite? [Y/n] ", path.display());
                            io::stdout().flush().unwrap();

                            let mut conf = String::new();
                            reader.read_line(&mut conf).unwrap();
                            conf = conf.trim().to_string().to_lowercase();

                            if conf == "y" || conf == "yes" {
                                State::Export(render, params, path.display().to_string())
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
                                State::Export(render, params, path.display().to_string())
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
                } else {
                    println!("{} is not a valid command.", input);
                    State::Prompt(render, params)
                }
            },
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
            },
            State::Set(render, mut params, field, value) => {
                // Set the specified field of the render
                match field {
                    Field::Iterations => {
                        match value.parse::<u32>() {
                            Ok(value) => params.max_iter = value,
                            Err(_) => println!("Invalid value: {}", value)
                        };
                    },
                    Field::Width => {
                        match value.parse::<u32>() {
                            Ok(value) => params.image_size.0 = value,
                            Err(_) => println!("Invalid value: {}", value)
                        };
                    },
                    Field::Height => {
                        match value.parse::<u32>() {
                            Ok(value) => params.image_size.1 = value,
                            Err(_) => println!("Invalid value: {}", value)
                        };
                    },
                    Field::CenterX => {
                        match value.parse::<Real>() {
                            Ok(value) => params.center.0 = value,
                            Err(_) => println!("Invalid value: {}", value)
                        };
                    },
                    Field::CenterY => {
                        match value.parse::<Real>() {
                            Ok(value) => params.center.1 = value,
                            Err(_) => println!("Invalid value: {}", value)
                        };
                    },
                    Field::Radius => {
                        match value.parse::<Real>() {
                            Ok(value) => params.radius = value,
                            Err(_) => println!("Invalid value: {}", value)
                        };
                    },
                    Field::Supersampling => {
                        match value.parse::<u32>() {
                            Ok(value) => params.supersampling = value,
                            Err(_) => println!("Invalid value: {}", value)
                        };
                    },
                    Field::ColorFunc => {
                        match value.parse::<ColorFunction>() {
                            Ok(value) => params.colorfunction = value,
                            Err(e) => println!("Invalid value: {} ({})", value, e)
                        }
                    }
                };

                State::Prompt(render, params)
            },
            State::Render(mut render, params) => {
                // Recalculate render pixels if necessary
                render.recalc(&params);

                // Render
                match render.run() {
                    Ok(_) => println!("Success!"),
                    Err(e) => println!("There was an error: {}", e)
                };

                // Return to prompt
                State::Prompt(render, params)
            },
            State::Export(render, params, path) => {
                // Create a new image
                let image = Image::new(&render, params.colorfunction.clone());

                // Export the image
                match image.export(path) {
                    Ok(_) => println!("Success!"),
                    Err(_) => println!("There was an error saving the image")
                };

                // Return to the prompt
                State::Prompt(render, params)
            },
            State::Dead => State::Dead
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

    return;
/*

    // Create a render object

    match render.run(10000) {
        Ok(_) => {
        },
        Err(s) => println!("Error: {}", s)
    };

    // export the image
    let cf = ColorFunction::new(Box::new(cf_greyscale));
    let image = Image::new(render, cf);

    image.export("/home/vinnie/Desktop/export.png");*/
}

fn show_help() {
    // TODO
    let help = String::from("You're on your own");
    println!("{}", help);
}
