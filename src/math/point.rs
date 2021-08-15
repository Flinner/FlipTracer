use super::{matrix::Matrix, transformations::Transformation, vector::Vector};
use std::ops::{Add, Mul, Neg, Sub};

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

/**
Transformations, Fluent API, allows chaining
```
use raytracer::math::point::Point;
use std::f64::consts::PI;

let p = Point::new(1.0,0.0,1.0)
            .rotate_x(PI/2.0)
            .scaling(5.0,5.0,5.0)
            .translation(10.0,5.0,7.0);

assert_eq!(p, Point::new(15.0, 0.0, 7.0));
```
*/
impl Point {
    /// Equivelent of `Transformation::rotate_x(rad) * self`
    pub fn rotate_x(self, rad: f64) -> Point {
        Transformation::rotate_x(rad) * self
    }
    /// Equivelent of `Transformation::rotate_y(rad) * self`
    pub fn rotate_y(self, rad: f64) -> Point {
        Transformation::rotate_y(rad) * self
    }
    /// Equivelent of `Transformation::rotate_z(rad) * self`
    pub fn rotate_z(self, rad: f64) -> Point {
        Transformation::rotate_z(rad) * self
    }
    /// Equivelent of `Transformation::scaling(x, y, z) * self`
    pub fn scaling(self, x: f64, y: f64, z: f64) -> Point {
        Transformation::scaling(x, y, z) * self
    }
    /// Equivelent of `Transformation::shearing(x_y, x_z, y_x, y_z, z_x, z_y) * self`
    pub fn shearing(self, x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Point {
        Transformation::shearing(x_y, x_z, y_x, y_z, z_x, z_y) * self
    }
    /// Equivelent of `Transformation::translation(x, y, z) * self`
    pub fn translation(self, x: f64, y: f64, z: f64) -> Point {
        Transformation::translation(x, y, z) * self
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
    type Output = Self;

    /// using `Mul` (`*`) for multiplying `Points` with `Matrix` (Tansformation): `Point` * `Matrix`
    ///```text
    /// [ 1.0,   2.0,  3.0,  4.0,        [x,
    ///   5.0,   6.0,  7.0,  8.0,   Mul   y
    ///   9.0,  10.0, 11.0, 12.0,   Mul   z
    ///   13.0, 14.0, 15.0, 15.0,]        0.0]
    ///```
    fn mul(self, m: Matrix) -> Self::Output {
        // 0.0 is the 'magic' number, used to distinguish between vectors and points
        let m = m.data;
        let p = self;
        Self {
            x: m[0] * p.x + m[1] * p.y + m[2] * p.z + m[3],
            y: m[4] * p.x + m[5] * p.y + m[6] * p.z + m[7],
            z: m[8] * p.x + m[9] * p.y + m[10] * p.z + m[11],
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

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        self.displacment(&rhs)
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        self.negative_displacment(&rhs)
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Vector {
        self.subtract(&rhs)
    }
}
