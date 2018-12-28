use std::fs::File;
use std::rc::Rc;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use crate::math::*;

#[derive(Copy, Clone)]
pub enum Color {
    RGB(f32, f32, f32),
    HSV(f32, f32, f32),
}

// TODO add converstion between RGB and HSV

type Func = Rc<dyn Fn(u32, u32, Complex) -> Color>;

#[derive(Clone)]
pub struct ColorFunction {
    pub func: Func,
}

impl FromStr for ColorFunction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_string().trim().to_string();

        if s == "greyscale" {
            Ok(ColorFunction::greyscale())
        } else if s == "rgreyscale" {
            Ok(ColorFunction::rgreyscale())
        } else if s.starts_with("color(") && s.ends_with(")") {
            let end = s.len() - 1;
            let param_str = &s[6..end].to_string();

            let params: Vec<_> = param_str.split(",").collect();
            if params.len() != 2 {
                Err("Incorrect syntax: color has 2 parameters (shift and scale).".to_string())
            } else {
                let shift = match params[0].trim().parse::<u32>() {
                    Ok(value) => value,
                    Err(e) => return Err(format!("Couldn't parse shift {}: {:?}.", params[0], e))
                };
                let scale = match params[1].trim().parse::<f64>() {
                    Ok(value) => value,
                    Err(e) => return Err(format!("Couldn't parse scale {}: {:?}.", params[1], e))
                };
                Ok(ColorFunction::color(shift, scale))
            }
        } else {
            Err("No such color function.".to_string())
        }
    }
}

impl ColorFunction {
    pub fn new(func: Func) -> ColorFunction {
        ColorFunction {
            func
        }
    }

    pub fn info(&self) -> String {
        String::from("")
    }

    ///////////////////////////////////////////////
    ////////// PREDEFINED COLORFUNCTIONS //////////
    ///////////////////////////////////////////////
    
    pub fn color(shift: u32, scale: f64) -> ColorFunction {
        // Read colors from file
        let file = File::open("colors.csv").unwrap();
        let colors: Vec<_> = BufReader::new(file).lines().map(|line| {
            let rgb: Vec<_> = line.unwrap().split(",").map(|s| s.parse::<u8>().unwrap() as f32 / 255.0).collect();
            Color::RGB(rgb[0], rgb[1], rgb[2])
        }).collect();
        ColorFunction::new(Rc::new(move |i: u32, m: u32, z: Complex| -> Color {
            if i == m {
                Color::RGB(0.0, 0.0, 0.0)
            } else {
                let size = z.abs();
                let smoothed = size.log(2.0).log(2.0);
                let idx = ((i as f64 + 1.0 - smoothed) * scale + shift as f64) as i32 % 2048;
                colors[idx as usize]
            }
        }))
    }

    pub fn greyscale() -> ColorFunction {
        ColorFunction::new(Rc::new(|i: u32, m: u32, _: Complex| -> Color {
            if i == m {
                Color::RGB(0.0, 0.0, 0.0)
            } else {
                let p = 1.0 - i as f32 / m as f32;
                Color::RGB(p, p, p)
            }
        }))
    }

    pub fn rgreyscale() -> ColorFunction {
        ColorFunction::new(Rc::new(|i: u32, m: u32, _: Complex| -> Color {
            if i == m {
                Color::RGB(1.0, 1.0, 1.0)
            } else {
                let p = i as f32 / m as f32;
                Color::RGB(p, p, p)
            }
        }))
    }

}
