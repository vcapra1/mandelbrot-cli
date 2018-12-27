use std::ops::{Add, Sub, Mul, Div};

pub type Real = f64;

#[derive(Clone, Copy)]
pub struct Complex(pub Real, pub Real);

impl Complex {
    pub fn abs_squared(&self) -> Real {
        (self.0 * self.0 + self.1 * self.1)
    }

    pub fn abs(&self) -> Real {
        self.abs_squared().sqrt()
    }
}

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
        Complex(self.0 * other.0 - self.1 * other.1, self.0 * other.1 + self.1 * other.0)
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
