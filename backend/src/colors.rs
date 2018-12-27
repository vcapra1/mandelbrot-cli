use crate::math::*;

pub enum Color {
    RGB(f32, f32, f32),
    HSV(f32, f32, f32),
}

// TODO add converstion between RGB and HSV

pub struct ColorFunction<F: Fn(u32, Complex) -> Color> {
    func: F,
}
