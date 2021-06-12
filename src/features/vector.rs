use std::ops::{Div, Mul, Neg};

#[derive(PartialEq, Debug)]
/// Represents a `Vector` in a Cartesian Space
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    /// Creates a new `Vector`
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    /// Adds Two `Vector`s together Producing a `Vector`
    pub fn add(self: &Self, vector2: &Vector) -> Vector {
        Vector {
            x: self.x + vector2.x,
            y: self.y + vector2.y,
            z: self.z + vector2.z,
        }
    }
    /// Returns the magnitude of the Vector
    pub fn magnitude(self: &Self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }
}

impl Neg for Vector {
    type Output = Self;
    /// Negates The Vector, using urnary `-`
    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vector {
    /// using `Mul` (`*`) for multiplying `Vectors`: `Vector` * `f64`
    type Output = Self;

    fn mul(self, m: f64) -> Self::Output {
        Self {
            x: self.x * m,
            y: self.y * m,
            z: self.z * m,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;
    fn div(self, d: f64) -> Self {
        Self {
            x: self.x / d,
            y: self.y / d,
            z: self.z / d,
        }
    }
}
