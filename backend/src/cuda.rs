use std::fmt;
use std::error;

use crate::render::Render;
use crate::math::Complex;

type Result = std::result::Result<Render, RenderError>;

#[derive(Debug, Clone)]
pub struct RenderError(pub String);

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

#[derive(Clone)]
#[repr(C)]
pub struct FFIRenderData {
    pixels: *mut FFIPixel,
    iterations: u32,
    num: u32,
}

extern "C" {
    fn cuda_compute(iterations: u32, data: FFIRenderData) -> u32;
}

pub fn compute(render: Render, iterations: u32) -> Result {
    // Convert to FFI-safe array
    let mut pixels_vec: Vec<FFIPixel> = render.pixels.iter().map(|p| FFIPixel { 
        c: FFIComplex::from(p.1), z: FFIComplex::from(p.2), i: p.0, d: p.3 
    }).collect();

    let data = FFIRenderData {
        pixels: pixels_vec.as_mut_ptr(),
        iterations: render.iterations,
        num: render.pixels.len() as u32,
    };

    std::mem::forget(pixels_vec);

    // Call C code
    let result_code = unsafe {
        cuda_compute(iterations, data.clone())
    };

    match result_code {
        0 => {
            // Get pixels vec back
            let pixels_vec = unsafe {
                Vec::from_raw_parts(data.pixels, data.num as usize, data.num as usize)
            };

            let render = Render {
                iterations: render.iterations + iterations,
                pixels: pixels_vec.iter().map(|p| (
                    p.i, p.c.to_complex(), p.z.to_complex(), p.d
                )).collect(),
                ..render
            };
            Ok(render)
        },
        c => Err(RenderError(format!("CUDA Error [{}].", c)))
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

impl FFIComplex {
    fn to_complex(&self) -> Complex {
        Complex(self.real, self.imag)
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
