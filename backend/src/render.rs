extern crate rand;

use rand::{thread_rng, Rng, distributions::Alphanumeric};

use std::thread::*;
use std::sync::{Arc, Mutex};
use std::io::{self, prelude::*};

use crate::cuda::*;
use crate::math::*;
use crate::image::*;
use crate::colors::*;

#[derive(Clone, Copy, PartialEq)]
pub struct Parameters {
    pub image_size: (u32, u32),
    pub supersampling: u32,
    pub center: Complex,
    pub radius: Real,
    pub max_iter: u32,
}

#[derive(Clone)]
pub struct Render {
    pub params: Parameters,
    pub iterations: u32,
    pub pixels: Vec<(u32, Complex, Complex, bool)>,
}

pub struct RenderJob {
    thread: JoinHandle<std::result::Result<(Render, Option<String>), String>>,
    progress: Arc<Mutex<Option<f64>>>,
}

impl Render {
    pub fn default() -> Render {
        Render::new(Parameters {
            image_size: (1000, 1000),
            supersampling: 1,
            center: Complex(0.0, 0.0),
            radius: 2.0,
            max_iter: 500,
        })
    }

    pub fn new(mut params: Parameters) -> Render {
        // Verify supersampling
        if params.supersampling == 0 {
            params.supersampling = 1;
        }

        // Multiply image size by supersampling factor
        params.image_size.0 *= params.supersampling;
        params.image_size.1 *= params.supersampling;

        // Create the list of pixels
        let mut pixels = Vec::with_capacity((params.image_size.0 * params.image_size.1) as usize);

        // Prepare the mapping (for faster calculations later)
        let mapping = Complex::get_mapping(params.image_size, (params.center, params.radius));

        // Populate the list
        for idx in 0..pixels.capacity() {
            let x = idx as u32 % params.image_size.0;
            let y = idx as u32 / params.image_size.0;

            // Convert the (x, y) image coords to complex coords based on the window
            let complex = mapping(x, y);

            // Insert Pixel into vector
            pixels.push((0, complex, Complex(0.0, 0.0), false));
        }

        Render {
            params,
            iterations: 0,
            pixels,
        }
    }

    // Using the params, recalculate the pixel array
    pub fn recalc(&mut self, params: &Parameters) {
        if self.params == *params {
            // We won't need to recalculate the pixel array
            self.params = params.clone();
        } else {
            // We do need to recaluclate
            *self = Render::new(params.clone());
        }
    }

    // Run a specified number of iterations on the Render
    pub fn run(self) -> RenderJob {
        // Create a RenderJob and return it
        RenderJob::new(self, "".into())
    }

    pub fn run_and_export(self, colorfunc: ColorFunction) -> RenderJob {
        // Create a RenderJob and return it
        RenderJob::new(self, colorfunc.info())
    }
}

impl RenderJob {
    fn new(mut render: Render, colorfunc: String) -> RenderJob {
        let progress = Arc::new(Mutex::new(Some(0.0)));

        let thread = {
            let progress = Arc::clone(&progress);

            std::thread::spawn(move || {
                // Call the CUDA code, passing the render struct
                let result = compute(render.clone(), Arc::clone(&progress));

                match result {

                    Ok(result) => {
                        // Update the data in the Render with the new data
                        render.pixels = result.pixels;
                        render.iterations = result.iterations;

                        if colorfunc != "" {
                            // Export the image
                            let image = Image::new(&render, colorfunc.parse::<ColorFunction>().unwrap());

                            let rand_string: String = thread_rng()
                                .sample_iter(&Alphanumeric)
                                .take(32)
                                .collect();
                            let path = format!("{}/mandelbrot{}.png", std::env::temp_dir().as_path().display(), rand_string);

                            // Export the image
                            match image.export(path.clone()) {
                                Ok(_) => {
                                    println!("Image exported to {}", path);
                                    // Update progress
                                    *progress.lock().unwrap() = None;
                                    Ok((render, Some(path)))
                                }
                                Err(_) => {
                                    // Update progress
                                    *progress.lock().unwrap() = None;
                                    Err("Could not export image".into())
                                }
                            }
                        } else {
                            *progress.lock().unwrap() = None;
                            Ok((render, None))
                        }
                    }
                    Err(RenderError(message)) => {
                        // There was an error, return the message
                        Err(message)
                    }
                }
            })
        };

        // Return the created job
        RenderJob { thread, progress }
    }

    /// Wait for the thread to finish.  This method blocks, and returns the render or error message
    /// when the thread is finished.  It also prints out the progress until it returns.
    pub fn join_with_progress(self) -> std::result::Result<(Render, Option<String>), String> {
        // Progress loop until 100 is returned
        while let Some(progress) = self.progress() {
            // Print the progress
            print!("\rProgress: {:.*}% ", 2, progress);
            io::stdout().flush().unwrap();
        }

        // Print 100%
        println!("\rProgress: 100.00%");

        // Join thread
        self.thread.join().unwrap()
    }

    /// Wait for the thread to finish.  This method blocks, and returns the render or error message
    /// when the thread is finished.
    pub fn join(self) -> std::result::Result<(Render, Option<String>), String> {
        self.thread.join().unwrap()
    }

    /// Get the progress of the job at the current time.  This method may block very briefly if the
    /// progress mutex is locked.  Prints None if the job is complete
    pub fn progress(&self) -> Option<f64> {
        *self.progress.lock().unwrap()
    }
}
