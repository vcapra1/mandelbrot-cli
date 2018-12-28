use crate::math::*;

pub enum Color {
    RGB(f32, f32, f32),
    HSV(f32, f32, f32),
}

// TODO add converstion between RGB and HSV

pub struct ColorFunction {
    pub func: Box<dyn Fn(u32, u32, Complex) -> Color>,
}

impl ColorFunction {
    pub fn new(func: Box<dyn Fn(u32, u32, Complex) -> Color>) -> ColorFunction {
        ColorFunction {
            func
        }
    }
}

pub fn cf_greyscale(i: u32, m: u32, _: Complex) -> Color {
    if i == m {
        Color::RGB(1.0, 1.0, 1.0)
    } else {
        let p = i as f32 / m as f32;
        Color::RGB(p, p, p)
    }
}
