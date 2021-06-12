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
