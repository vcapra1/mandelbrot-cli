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
    width: u32,
    height: u32
}

extern "C" {
    fn cuda_compute(iterations: u32, data: FFIRenderData, progress: *mut u32) -> u32;
}

pub fn compute(render: Render) -> Result {
    // Convert to FFI-safe array
    let mut pixels_vec: Vec<FFIPixel> = render.pixels.iter().map(|p| FFIPixel { 
        c: FFIComplex::from(p.1), z: FFIComplex::from(p.2), i: p.0, d: p.3 
    }).collect();

    let data = FFIRenderData {
        pixels: pixels_vec.as_mut_ptr(),
        iterations: render.iterations,
        num: render.pixels.len() as u32,
        width: render.params.image_size.0,
        height: render.params.image_size.1,
    };

    std::mem::forget(pixels_vec);

    // Progress counter
    let mut progress: u32 = 0;

    //TODO let progress_ptr: u64 = (&mut progress as *mut u32) as u64;

    /* TODO TODO TODO let progress_thread = std::thread::spawn(move || {
        unsafe {
            loop {
                // FIXME This really seems sketchy
                let progress = *(progress_ptr as *mut u32);
                if progress == 999 {
                    break;
                }
                println!("Progress: {}%", progress);
                if progress >= 100 {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    });*/

    // Call C code
    let result_code = unsafe {
        cuda_compute(render.params.max_iter, data.clone(), &mut progress as *mut u32)
    };

    match result_code {
        0 => {
            // Stop the progress thread
            //TODO progress = 100;
            // TODO progress_thread.join();

            // Get pixels vec back
            let pixels_vec = unsafe {
                Vec::from_raw_parts(data.pixels, data.num as usize, data.num as usize)
            };

            let render = Render {
                iterations: render.params.max_iter,
                pixels: pixels_vec.iter().map(|p| (
                    p.i, p.c.to_complex(), p.z.to_complex(), p.d
                )).collect(),
                ..render
            };
            Ok(render)
        },
        c => {
            // Stop the progress thread
            //TODO progress = 999;
            // TODO progress_thread.join();

            Err(RenderError(format!("CUDA Error [{}].", c)))
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
