use crate::util::Config;
use crate::render::*;
use crate::math::*;

pub fn begin(config: Config) {
    // Create a render object
    let mut render = Render::new(Parameters {
        image_size: (1000, 1000),
        supersampling: 1,
        center: Complex(0.0, 0.0),
        radius: 2.0
    });

    render.run(1000);
}
