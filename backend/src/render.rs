use crate::math::*;

pub struct Parameters {
    max_iter: u32,
    image_size: (u32, u32),
    supersampling: u32,
    center: Complex,
    radius: Real,
}

pub struct Render {
    params: Parameters,
    iterations: u32,
    pixels: Vec<(u32, Complex, Complex, bool)>,
}

// TODO impl Render
