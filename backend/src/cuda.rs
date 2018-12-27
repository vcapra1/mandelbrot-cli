use std::fmt;
use std::error;

use crate::render::Render;
use crate::math::Complex;

type Result = std::result::Result<(), RenderError>;

#[derive(Debug, Clone)]
pub struct RenderError(String);

type FFIReal = f64;

#[repr(C)]
struct FFIComplex {
    real: FFIReal,
    imag: FFIReal,
}

#[repr(C)]
struct FFIPixel {
    c: FFIComplex,
    z: FFIComplex,
    i: u32,
    d: bool,
}

#[repr(C)]
pub struct FFIRenderData {
    iterations: u32,
    num: u32,
    pixels: *mut FFIPixel,
}

extern "C" {
    fn cuda_compute(data: FFIRenderData, iterations: u32) -> u32;
}

pub fn compute(data: FFIRenderData, iterations: u32) -> Result {
    // Call C code
    let result_code = unsafe {
        cuda_compute(data, iterations)
    };

    println!("Result from C: {}", result_code);

    Ok(())
}

impl From<Render> for FFIRenderData {
    fn from(render: Render) -> FFIRenderData {
        // Convert to FFI-safe array
        let mut pixels_vec: Vec<FFIPixel> = render.pixels.iter().map(|p| FFIPixel { c: FFIComplex::from(p.1), z: FFIComplex::from(p.2), i: p.0, d: p.3 } ).collect();
        FFIRenderData {
            iterations: render.iterations,
            pixels: pixels_vec.as_mut_ptr(),
            num: render.pixels.len() as u32,
        }
    }
}

impl From<Complex> for FFIComplex {
    fn from(complex: Complex) -> FFIComplex {
        FFIComplex {
            real: complex.0,
            imag: complex.1
        }
    }
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RenderError: {}", self.0)
    }
}

impl error::Error for RenderError {
    fn description(&self) -> &str {
        &self.0[..]
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
