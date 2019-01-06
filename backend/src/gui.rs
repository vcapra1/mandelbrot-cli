use crate::util::Config;

use crate::render::*;

use crate::math::*;
use crate::colors::*;

use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};

pub fn begin(config: Config) {
    // Build the address
    let address = if let Some(port) = config.port {
        // Try to use the port specified
        format!("127.0.0.1:{}", port)
    } else {
        // Let the OS assign a port
        String::from("127.0.0.1:0")
    };

    println!("Binding to address {}", address);

    // Connect via a socket on the given port
    let listener = TcpListener::bind(address).unwrap();

    // Begin comm loop
    for stream in listener.incoming().take(1) {
        match stream {
            Ok(stream) => handle_connection(stream).unwrap(),
            Err(e) => println!("{:?}", e)
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    // Create the Render object
    let mut render = Render::default();

    let mut current_operation: Option<RenderJob> = None;

    // Send a message on the stream to indicate that the backend is ready
    stream.write("ready\n".as_bytes())?;

    // Begin loop
    loop {
        // Read a line
        let mut buf = vec![0u8; 1024];
        stream.read(&mut buf)?;
        let line = String::from_utf8_lossy(&buf[..]);

        // Remove null bytes from end of string
        let line = line.trim_matches(char::from(0)).trim();

        println!("Received: {}", line);

        if line.starts_with("render") {
            // Make sure there isn't already an operatin in progress
            if let Some(ref mut operation) = current_operation {
                // See if the operation is done
                if let None = operation.progress() {
                    // It's done
                    current_operation = None;
                } else {
                    stream.write("error(5)\n".as_bytes())?;
                    continue
                }
            }

            // render [iterations] [width] [height] [supersampling] [centerx] [centery] [radius] \
            //   [colorfunc]
            let parts: Vec<_> = line.split(" ").collect();

            // Ensure there are 9 parts
            if parts.len() != 9 {
                stream.write("error(1)\n".as_bytes())?;
                continue
            }

            // Parse iterations
            let iterations = match parts[1].parse::<u32>() {
                Ok(value) => value,
                Err(_) => {
                    stream.write("error(2.1)\n".as_bytes())?;
                    continue
                }
            };

            // Parse width
            let width = match parts[2].parse::<u32>() {
                Ok(value) => value,
                Err(_) => {
                    stream.write("error(2.2)\n".as_bytes())?;
                    continue
                }
            };
            
            // Parse height
            let height = match parts[3].parse::<u32>() {
                Ok(value) => value,
                Err(_) => {
                    stream.write("error(2.3)\n".as_bytes())?;
                    continue
                }
            };

            // Parse supersampling
            let supersampling = match parts[4].parse::<u32>() {
                Ok(value) => value,
                Err(_) => {
                    stream.write("error(2.4)\n".as_bytes())?;
                    continue
                }
            };

            // Parse centerx
            let centerx = match parts[5].parse::<Real>() {
                Ok(value) => value,
                Err(_) => {
                    stream.write("error(2.5)\n".as_bytes())?;
                    continue
                }
            };

            // Parse centery
            let centery = match parts[6].parse::<Real>() {
                Ok(value) => value,
                Err(_) => {
                    stream.write("error(2.6)\n".as_bytes())?;
                    continue
                }
            };

            // Parse radius
            let radius = match parts[7].parse::<Real>() {
                Ok(value) => value,
                Err(_) => {
                    stream.write("error(2.7)\n".as_bytes())?;
                    continue
                }
            };

            // Parse colorfunction
            let colorfunc = match parts[8].parse::<ColorFunction>() {
                Ok(value) => value,
                Err(e) => {
                    println!("{}", e);
                    stream.write("error(2.8)\n".as_bytes())?;
                    continue
                }
            };

            // Validate range of supersampling
            if supersampling == 0 {
                stream.write("error(3.1)\n".as_bytes())?;
            }

            // Validate range of radius
            if radius < 0.0 {
                stream.write("error(3.2)\n".as_bytes())?;
            }

            // DEBUG: Log values for debugging
            println!("Iterations:     {}", iterations);
            println!("Image Width:    {}", width);
            println!("Image Height:   {}", height);
            println!("Supersampling:  {}", supersampling);
            println!("Center X:       {}", centerx);
            println!("Center Y:       {}", centery);
            println!("Radius:         {}", radius);
            println!("Color function: {}", colorfunc.info());

            // Apply the given values to the render params
            let new_params = Parameters {
                image_size: (width, height),
                supersampling,
                center: Complex(centerx, centery),
                radius,
                max_iter: iterations,
            };

            // Update the render data
            render.recalc(&new_params);

            // Tell the front end that the request was valid, and we'll begin rendering
            stream.write("ok\n".as_bytes())?;

            // Begin rendering, and set the current operation
            current_operation = Some(render.clone().run());
        } else if line == "progress" {
            if let Some(ref operation) = current_operation {
                // Get the progress
                let progress = format!("{}\n", operation.progress().unwrap_or(101.0));

                // Write it back
                stream.write((&progress[..]).as_bytes())?;
            } else {
                // No current operation
                stream.write("error(4)\n".as_bytes())?;
            }
        } else if line == "output" {
            // Make sure an opearation exists
            if let Some(ref mut operation) = current_operation {
                // Make sure the operation is complete
                if let Some(_) = operation.progress() {
                    // Operation is still in progress
                    stream.write("error(6.2)\n".as_bytes())?;
                } else {
                    // We good, fetch the result
                    match current_operation.take().unwrap().join() {
                        Ok(new_render) => {
                            // Save the render
                            render = new_render;

                            // 
                        }
                        Err(e) => {
                            stream.write("error(6.3)\n".as_bytes())?;
                        }
                    }
                }

            } else {
                // No operation
                stream.write("error(6.1)\n".as_bytes())?;
            }
            // Return /tmp file location of exported image

            stream.write("error(0)\n".as_bytes())?;
        } else if line == "exit" {
            stream.write("ok\n".as_bytes())?;
            break;
        } else {
            stream.write("error(0)\n".as_bytes())?;
        }

        // TODO: accept requests for progress, exporting, (maybe even canceling a render?)
    }

    Ok(())
}
