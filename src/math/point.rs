use super::{matrix::Matrix, vector::Vector};
use std::ops::{Mul, Neg};

pub const ORIGIN: Point = Point {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

// ========== POINT =============///
#[derive(Debug, Clone, Copy)]
/// Represents a 3D point in a Cartesian Space
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    /// Creates a new `Point`
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    /// Displaces the `Point` by adding it to a `Vector`
    pub fn displacment(&self, vector: &Vector) -> Point {
        Point {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        }
    }

    /// Displaces the `Point` by subtracting it from a `Vector`
    pub fn negative_displacment(&self, vector: &Vector) -> Point {
        Point {
            x: self.x - vector.x,
            y: self.y - vector.y,
            z: self.z - vector.z,
        }
    }

    /// Subtracts `self - point2` giving a `Vector`
    pub fn subtract(&self, point2: &Point) -> Vector {
        Vector {
            x: self.x - point2.x,
            y: self.y - point2.y,
            z: self.z - point2.z,
        }
    }
}

impl Neg for Point {
    type Output = Self;
    /// Negates The Point, using urnary `-`
    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Matrix> for Point {
    /// using `Mul` (`*`) for multiplying `Points` with `Matrix` (Tansformation): `Point` * `Matrix`
    type Output = Self;

    fn mul(self, m: Matrix) -> Self::Output {
        // 1.0 is the 'magic' number, used to distinguish between vectors and points
        // the point is converted to a matrix to allow multiplication
        let self_matrix = Matrix::new_from_vec(4, 1, vec![self.x, self.y, self.z, 1.0]);
        let product = m * self_matrix;
        Self {
            x: product.data[0],
            y: product.data[1],
            z: product.data[2],
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        use std::f64::EPSILON;
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
    }
}
