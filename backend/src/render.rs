use crate::colors::*;
use crate::cuda::*;
use crate::math::*;

#[derive(Clone)]
pub struct Parameters {
    pub image_size: (u32, u32),
    pub supersampling: u32,
    pub center: Complex,
    pub radius: Real,
    pub max_iter: u32,
    pub colorfunction: ColorFunction,
}

#[derive(Clone)]
pub struct Render {
    pub params: Parameters,
    pub iterations: u32,
    pub pixels: Vec<(u32, Complex, Complex, bool)>,
}

impl Render {
    pub fn default() -> Render {
        Render::new(Parameters {
            image_size: (1000, 1000),
            supersampling: 1,
            center: Complex(0.0, 0.0),
            radius: 2.0,
            max_iter: 500,
            colorfunction: ColorFunction::greyscale(),
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
        if self.params.image_size == params.image_size
            && self.params.supersampling == params.supersampling
            && self.params.center == params.center
            && self.params.radius == params.radius
            && self.params.max_iter >= params.max_iter
        {
            // We won't need to recalculate the pixel array
            self.params = params.clone();
        } else {
            // We do need to recaluclate
            *self = Render::new(params.clone());
        }
    }

    // Run a specified number of iterations on the Render
    pub fn run(&mut self, show_progress: bool) -> Result<(), String> {
        // Call the C code, passing the data struct and the number of iterations to do
        let result = compute(self.clone(), show_progress);

        // Update self with the results
        match result {
            Ok(result) => {
                self.pixels = result.pixels;
                self.iterations = result.iterations;
                Ok(())
            }
            Err(RenderError(message)) => Err(message),
        }
    }

    // Render, but in a new thread
    pub fn run_thread(&mut self) -> std::thread::JoinHandle<()> {
        std::thread::spawn(|| {
            // TODO: implement rendering here
        })
    }
}
