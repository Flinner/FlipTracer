use std::ops::{Add, Div, Mul, Neg};

pub const UNIT_X: Vector = Vector {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};
pub const UNIT_Y: Vector = Vector {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};
pub const UNIT_Z: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

#[derive(PartialEq, Clone, Copy, Debug)]
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

    /// DEPRECIATED: Adds Two `Vector`s together Producing a `Vector`:
    pub fn add(self: &Self, other: &Vector) -> Self {
        *self + *other
    }

    /// Returns the magnitude of the Vector
    pub fn magnitude(self: &Self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }

    /// Normalize the `Vector`
    pub fn normalize(self: &Self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    /// Dot Product of two vectors, producing a scalar, order doesn't matter
    pub fn dot_product(self: &Self, vector2: &Vector) -> f64 {
        self.x * vector2.x + self.y * vector2.y + self.z * vector2.z
    }

    /// Vector Product of two vectors, producing a Vector, order IS important
    pub fn cross_product(self: &Self, vector2: &Vector) -> Self {
        Vector {
            x: self.y * vector2.z - self.z * vector2.y,
            y: self.z * vector2.x - self.x * vector2.z,
            z: self.x * vector2.y - self.y * vector2.x,
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

impl Mul<f64> for Vector {
    /// using `Mul` (`*`) for multiplying `Vectors` with `Scalars`: `Vector` * `f64`
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
    /// using `Div` (`/`) for dividing `Vectors` with `Scalars`: `Vector` / `f64`
    fn div(self, d: f64) -> Self {
        Self {
            x: self.x / d,
            y: self.y / d,
            z: self.z / d,
        }
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    /// using `Add` (`/`) for Adding `Vectors` and `Vectors`: `Vector` + `Vector`
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
