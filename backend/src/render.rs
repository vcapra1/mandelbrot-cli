use crate::math::*;
use crate::cuda::*;

#[derive(Clone, Copy)]
pub struct Parameters {
    pub image_size: (u32, u32),
    pub supersampling: u32,
    pub center: Complex,
    pub radius: Real,
}

#[derive(Clone)]
pub struct Render {
    pub params: Parameters,
    pub iterations: u32,
    pub pixels: Vec<(u32, Complex, Complex, bool)>,
}

impl Render {
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

            let diverged = complex.abs() > 2.0;

            // Insert Pixel into vector
            pixels.push((0, complex, Complex(0.0, 0.0), diverged));
        }

        Render {
            params,
            iterations: 0,
            pixels
        }
    }

    // Run a specified number of iterations on the Render
    pub fn run(&mut self, iterations: u32) -> Result<(), String> {
        // Call the C code, passing the data struct and the number of iterations to do
        let result = compute(self.clone(), iterations);
        
        // Update self with the results
        match result {
            Ok(result) => {
                self.pixels = result.pixels;
                self.iterations = result.iterations;
                Ok(())
            },
            Err(RenderError(message)) => Err(message)
        }
    }
}
