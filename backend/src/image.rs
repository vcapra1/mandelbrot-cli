extern crate image;

use crate::colors::*;
use crate::render::*;

// Image structure, created from a render, that stores the pixels as colors
pub struct Image {
    pub size: (u32, u32),
    pub pixels: Vec<Color>,
    pub scale: u32,
}

impl Image {
    pub fn new(render: &Render, color_func: ColorFunction) -> Image {
        // Use the provided color functino to map each pixel from the render to a color
        let pixels: Vec<_> = render
            .pixels
            .iter()
            .map(|(i, _, z, _)| (*color_func.func)(*i, render.iterations, *z))
            .collect();

        Image {
            pixels,
            size: render.params.image_size,
            scale: render.params.supersampling,
        }
    }

    // Export the image to specified file
    pub fn export(&self, path: String) -> std::io::Result<()> {
        // Create a new RGB image
        let mut img = image::RgbImage::new(self.size.0, self.size.1);

        // Set the colors of each pixel according to the stored data
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            // Compute index
            let idx: usize = (x + y * self.size.0) as usize;

            // Put color into image
            if let Color::RGB(r, g, b) = self.pixels[idx] {
                *pixel = image::Rgb([(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8]);
            } else {
                panic!("HSV Colors not implemented!");
            }
        }

        // Resize image down by supersampling factor (linear filtering)
        let resized = image::DynamicImage::ImageRgb8(img).resize(
            self.size.0 / self.scale,
            self.size.1 / self.scale,
            image::FilterType::Triangle,
        );

        // Save the image to filesystem
        resized.save(&path)
    }
}
