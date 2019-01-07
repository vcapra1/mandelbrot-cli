use std::ops::{Add, Div, Mul, Sub};

// Scalar floating-point type to be used across the program
pub type Real = f64;

#[derive(Clone, Copy, PartialEq, Debug)]
// Complex floating-point type to be used across the program
pub struct Complex(pub Real, pub Real);

impl Complex {
    // Given the window and image size, get a closure that can be used to convert an image
    // coordinate to a complex coordinate.
    pub fn get_mapping(
        (w, h): (u32, u32),
        (center, radius): (Complex, Real),
    ) -> Box<dyn Fn(u32, u32) -> Complex> {
        // Compute the scale and shift in each dimension
        let (scale, shift) = if w >= h {
            // Radius maps to height
            (
                2.0 * radius / h as Real,
                Complex(center.0 - radius * w as Real / h as Real, center.1 + radius),
            )
        } else {
            // Radius maps to width
            (
                2.0 * radius / w as Real,
                Complex(center.0 - radius, center.1 + radius * h as Real / w as Real),
            )
        };

        // Return the mapping as a boxed closure
        Box::new(move |x: u32, y: u32| {
            let x = x as Real * scale;
            let y = y as Real * -scale;
            Complex(x, y) + shift
        })
    }

    // Compute the squared absolute value of the complex number, which is faster to compute than
    // the actual absolute value because no square root is needed
    pub fn abs_squared(&self) -> Real {
        (self.0 * self.0 + self.1 * self.1)
    }

    // Compute the absolute value of the complex number
    pub fn abs(&self) -> Real {
        self.abs_squared().sqrt()
    }
}

//////////////////////////////////////////////////
///////// Operations for Complex Numbers /////////
//////////////////////////////////////////////////

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex(
            self.0 * other.0 - self.1 * other.1,
            self.0 * other.1 + self.1 * other.0,
        )
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        let (Complex(a, b), Complex(c, d)) = (self, other);
        let denom = c * c + d * d;
        Complex(a * c + b * d, b * c - a * d) / denom
    }
}

impl Div<Real> for Complex {
    type Output = Complex;

    fn div(self, other: Real) -> Complex {
        Complex(self.0 / other, self.1 / other)
    }
}

impl From<Real> for Complex {
    fn from(real: Real) -> Complex {
        Complex(real, 0.0)
    }
}
