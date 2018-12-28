use crate::util::Config;
use crate::render::*;
use crate::image::*;
use crate::math::*;
use crate::colors::*;

pub fn begin(_config: Config) {
    // Create a render object
    let mut render = Render::new(Parameters {
        image_size: (1000, 1000),
        supersampling: 1,
        center: Complex(0.0, 0.0),
        radius: 2.0
    });

    match render.run(10000) {
        Ok(_) => {
            println!("All good!");
            println!("i[500500] = {}", render.pixels[500500].0);
        },
        Err(s) => println!("Error: {}", s)
    };

    // export the image
    let cf = ColorFunction::new(Box::new(|i: u32, m: u32, z: Complex| -> Color { 
        Color::RGB(0.0, 0.0, 1.0)
    }));
    let image = Image::new(render, cf);

    image.export("/home/vinnie/Desktop/export.png");
}
