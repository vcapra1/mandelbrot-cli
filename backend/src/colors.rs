use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::str::FromStr;

use crate::math::*;

#[derive(Copy, Clone)]
// A color can be either RGB or HSV, each represented by 3 floating-point values
pub enum Color {
    RGB(f32, f32, f32),
    HSV(f32, f32, f32),
}

// TODO add converstion between RGB and HSV

// The type of closure that maps a rendered pixel to a color
type Func = Rc<dyn Fn(u32, u32, Complex) -> Color>;

#[derive(Clone)]
// Wrapper struct for mapping function
pub struct ColorFunction {
    pub name: String,
    pub func: Func,
}

// Allow for parsing colorfunctions from user input
impl FromStr for ColorFunction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Trim whitespace off the string
        let s = s.to_string().trim().to_string();

        if s == "greyscale" {
            // Simple greyscale function
            Ok(ColorFunction::greyscale())
        } else if s == "rgreyscale" {
            // Reversed greyscale function
            Ok(ColorFunction::rgreyscale())
        } else if s.starts_with("color(") && s.ends_with(")") {
            // Color function, with two given parameters, shift and scale

            // Remove "color(" and ")", leaving just the parameters
            let end = s.len() - 1;
            let param_str = &s[6..end].to_string();

            // Isolate the parameters
            let params: Vec<_> = param_str.split(",").collect();

            if params.len() != 2 {
                Err("Incorrect syntax: color has 2 parameters (shift and scale).".to_string())
            } else {
                // Parse the parameters into numerical values
                let shift = match params[0].trim().parse::<u32>() {
                    Ok(value) => value,
                    Err(e) => return Err(format!("Couldn't parse shift {}: {:?}.", params[0], e)),
                };
                let scale = match params[1].trim().parse::<f64>() {
                    Ok(value) => value,
                    Err(e) => return Err(format!("Couldn't parse scale {}: {:?}.", params[1], e)),
                };

                Ok(ColorFunction::color(shift, scale))
            }
        } else if s.starts_with("red(") && s.ends_with(")") {
            // Color function, with two given parameters, shift and scale

            // Remove "color(" and ")", leaving just the parameters
            let end = s.len() - 1;
            let param_str = &s[4..end].to_string();

            // Isolate the parameters
            let params: Vec<_> = param_str.split(",").collect();

            if params.len() != 2 {
                Err("Incorrect syntax: color has 2 parameters (shift and scale).".to_string())
            } else {
                // Parse the parameters into numerical values
                let shift = match params[0].trim().parse::<u32>() {
                    Ok(value) => value,
                    Err(e) => return Err(format!("Couldn't parse shift {}: {:?}.", params[0], e)),
                };
                let scale = match params[1].trim().parse::<f64>() {
                    Ok(value) => value,
                    Err(e) => return Err(format!("Couldn't parse scale {}: {:?}.", params[1], e)),
                };

                Ok(ColorFunction::red(shift, scale))
            }
        } else {
            Err(format!("No such color function: {}.", s))
        }
    }
}

impl ColorFunction {
    pub fn new(func: Func, name: String) -> ColorFunction {
        ColorFunction { name, func }
    }

    // Get string representation of color function
    pub fn info(&self) -> String {
        self.name.clone()
    }

    ///////////////////////////////////////////////
    ////////// PREDEFINED COLORFUNCTIONS //////////
    ///////////////////////////////////////////////

    pub fn color(shift: u32, scale: f64) -> ColorFunction {
        // Read colors from file
        let file = File::open("colors.csv").unwrap();
        let colors: Vec<_> = BufReader::new(file)
            .lines()
            .map(|line| {
                let rgb: Vec<_> = line
                    .unwrap()
                    .split(",")
                    .map(|s| s.parse::<u8>().unwrap() as f32 / 255.0)
                    .collect();
                Color::RGB(rgb[0], rgb[1], rgb[2])
            })
            .collect();
        ColorFunction::new(
            Rc::new(move |i: u32, m: u32, z: Complex| -> Color {
                if i == m {
                    Color::RGB(0.0, 0.0, 0.0)
                } else {
                    let size = z.abs();
                    let smoothed = size.log(2.0).log(2.0);
                    let idx = ((i as f64 + 1.0 - smoothed) * scale + shift as f64) as i32 % 2048;
                    colors[idx as usize]
                }
            }),
            format!("color({}, {})", shift, scale),
        )
    }

    pub fn red(shift: u32, scale: f64) -> ColorFunction {
        // Read colors from file
        let file = File::open("red.csv").unwrap();
        let colors: Vec<_> = BufReader::new(file)
            .lines()
            .map(|line| {
                let rgb: Vec<_> = line
                    .unwrap()
                    .split(",")
                    .map(|s| s.parse::<u8>().unwrap() as f32 / 255.0)
                    .collect();
                Color::RGB(rgb[0], rgb[1], rgb[2])
            })
            .collect();
        ColorFunction::new(
            Rc::new(move |i: u32, m: u32, z: Complex| -> Color {
                if i == m {
                    Color::RGB(0.0, 0.0, 0.0)
                } else {
                    let size = z.abs();
                    let smoothed = size.log(2.0).log(2.0);
                    let idx = ((i as f64 + 1.0 - smoothed) * scale + shift as f64) as i32 % 2048;
                    colors[idx as usize]
                }
            }),
            format!("red({}, {})", shift, scale),
        )
    }

    pub fn greyscale() -> ColorFunction {
        ColorFunction::new(
            Rc::new(|i: u32, m: u32, _: Complex| -> Color {
                if i == m {
                    Color::RGB(0.0, 0.0, 0.0)
                } else {
                    let p = 1.0 - i as f32 / m as f32;
                    Color::RGB(p, p, p)
                }
            }),
            "greyscale".to_string(),
        )
    }

    pub fn rgreyscale() -> ColorFunction {
        ColorFunction::new(
            Rc::new(|i: u32, m: u32, _: Complex| -> Color {
                if i == m {
                    Color::RGB(1.0, 1.0, 1.0)
                } else {
                    let p = i as f32 / m as f32;
                    Color::RGB(p, p, p)
                }
            }),
            "rgreyscale".to_string(),
        )
    }
}
