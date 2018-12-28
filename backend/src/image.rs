extern crate image;

use crate::render::*;
use crate::colors::*;

pub struct Image {
    pub size: (u32, u32),
    pub pixels: Vec<Color>,
}

impl Image {
    pub fn new(render: Render, color_func: ColorFunction) -> Image {
        let pixels: Vec<_> = render.pixels.iter().map(|(i, _, z, _)| {
            (*color_func.func)(*i, render.iterations, *z)
        }).collect();
        Image { pixels, size: render.params.image_size }
    }

    pub fn export(&self, path: &str) { 
        let mut img = image::RgbImage::new(self.size.0, self.size.1);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let idx: usize = (x + y * self.size.0) as usize;

            if let Color::RGB(r, g, b) = self.pixels[idx] {
                *pixel = image::Rgb([(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8]);
            } else {
                panic!("HSV Colors not implemented!");
            }
        }

        img.save(path).unwrap();
    }
}
