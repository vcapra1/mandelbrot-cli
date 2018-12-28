use std::io::{self, prelude::*};
use std::path::Path;

use crate::util::Config;
use crate::render::*;
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
}

#[derive(Clone)]
enum State {
    Prompt(Render),
    Set(Render, Field, String),
    Get(Render, Field),
    Render(Render),
    Export(Render, String),
    Dead,
}

impl State {
    fn execute(self) -> State {
        match self {
            State::Prompt(render) => {
                // Print a prompt
                print!("> ");
                io::stdout().flush().unwrap();

                // Read user input
                let reader = io::stdin();
                let mut input = String::new();
                reader.read_line(&mut input).unwrap();
                input = input.trim().to_string();

                // Parse the input
                if input == "quit" || input == "exit" {
                    State::Dead
                } else if input.starts_with("set ") {
                    let input = (&input[4..]).to_string().trim().to_string();
                    let first_word = input.split_whitespace().next().unwrap().to_string();

                    let field_strs = vec!["iterations", "width", "height", "center:x", "center:y", "radius", "supersampling"];
                    let fields = vec![Field::Iterations, Field::Width, Field::Height, Field::CenterX, Field::CenterY, Field::Radius, Field::Supersampling];

                    for (idx, field) in field_strs.iter().enumerate() {
                        if first_word == *field {
                            let n = field.len() + 1;
                            return State::Set(render, fields[idx].clone(), (&input[n..]).to_string().trim().to_string());
                        }
                    }

                    println!("{} is not a valid field.", input.split_whitespace().next().unwrap());
                    State::Prompt(render)
                } else if input.starts_with("get ") {
                    let input = (&input[4..]).to_string().trim().to_string();

                    let field_strs = vec!["iterations", "width", "height", "center:x", "center:y", "radius", "supersampling"];
                    let fields = vec![Field::Iterations, Field::Width, Field::Height, Field::CenterX, Field::CenterY, Field::Radius, Field::Supersampling];

                    for (idx, field) in field_strs.iter().enumerate() {
                        if input == *field {
                            return State::Get(render, fields[idx].clone());
                        }
                    }

                    println!("{} is not a valid field.", input.split_whitespace().next().unwrap());
                    State::Prompt(render)
                } else if input == "render" {
                    State::Render(render)
                } else if input.starts_with("export ") {
                    let path = Path::new(&input[7..]);

                    if path.exists() {
                        if path.is_dir() {
                            // If it's a directory, this won't work
                            println!("\"{}\" is a directory.", path.display());
                            State::Prompt(render)
                        } else if path.is_file() {
                            print!("\"{}\" already exists.  Do you want to overwrite? [Y/n] ", path.display());
                            io::stdout().flush().unwrap();

                            let mut conf = String::new();
                            reader.read_line(&mut conf).unwrap();
                            conf = conf.trim().to_string().to_lowercase();

                            if conf == "y" || conf == "yes" {
                                State::Export(render, path.display().to_string())
                            } else {
                                // Don't do anything
                                println!("No action taken.");
                                State::Prompt(render)
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
                                State::Prompt(render)
                            } else {
                                // We're good
                                State::Export(render, path.display().to_string())
                            }
                        } else {
                            // The parent doesn't exist, this won't work
                            println!("No such directory: \"{}\".", parent.display());
                            State::Prompt(render)
                        }
                    }
                } else if input == "help" {
                    // TODO
                    unimplemented!()
                } else {
                    println!("{} is not a valid command.", input);
                    State::Prompt(render)
                }
            },
            State::Dead => State::Dead,
            _ => State::Dead
        }
    }
}

pub fn begin(_config: Config) {
    let render = Render::new(Parameters {
        image_size: (1000, 1000),
        supersampling: 1,
        center: Complex(0.0, 0.0),
        radius: 2.0
    });
    let mut state: State = State::Prompt(render);

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
