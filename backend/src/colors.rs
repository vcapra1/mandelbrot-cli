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
