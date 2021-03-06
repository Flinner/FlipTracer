use std::ops::Mul;

use super::{matrix::Matrix, point::Point, vector::Vector};

/// Wrapper around `Matrix`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transformation {
    pub matrix: Matrix,
}

impl Transformation {
    /// Identity Matrix `4x4`
    pub fn identity() -> Transformation {
        let matrix = Matrix::identity();
        Transformation { matrix }
    }

    /// Creates a translation, that only works with `Points` and not `Vectors`
    pub fn translation(x: f64, y: f64, z: f64) -> Transformation {
        let mut matrix = Matrix::identity();
        matrix.write(0, 3, x);
        matrix.write(1, 3, y);
        matrix.write(2, 3, z);
        Transformation { matrix }
    }

    /// Inverses the Transformation
    pub fn inverse(self) -> Option<Transformation> {
        self.matrix
            .inverse()
            .map(|matrix| Transformation { matrix })
    }
    /// Transpose the Transformation
    pub fn transpose(self) -> Transformation {
        Transformation {
            matrix: self.matrix.transpose(),
        }
    }

    /// Return a Scaling Transformation. works for both `Point` and `Vectors`
    pub fn scaling(x: f64, y: f64, z: f64) -> Transformation {
        let vec = [
            x, 0.0, 0.0, 0.0, //
            0.0, y, 0.0, 0.0, //
            0.0, 0.0, z, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ];
        let matrix = Matrix::new_from_vec(vec);
        Transformation { matrix }
    }

    /// Return an X Rotation Transformation. works for both `Point` Only!
    /// takes `rad` angle in radians only!
    pub fn rotate_x(rad: f64) -> Transformation {
        let cos = rad.cos();
        let sin = rad.sin();

        let vec = [
            1.0, 0.0, 0.0, 0.0, //
            0.0, cos, -sin, 0.0, //
            0.0, sin, cos, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ];
        let matrix = Matrix::new_from_vec(vec);
        Transformation { matrix }
    }

    /// Return an Y Rotation Transformation. works for both `Point` Only!
    /// takes `rad` angle in radians only!
    pub fn rotate_y(rad: f64) -> Transformation {
        let cos = rad.cos();
        let sin = rad.sin();

        let vec = [
            cos, 0.0, sin, 0.0, //
            0.0, 1.0, 0.0, 0.0, //
            -sin, 0.0, cos, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ];
        let matrix = Matrix::new_from_vec(vec);
        Transformation { matrix }
    }

    /// Return an Z Rotation Transformation. works for both `Point` Only!
    /// takes `rad` angle in radians only!
    pub fn rotate_z(rad: f64) -> Transformation {
        let cos = rad.cos();
        let sin = rad.sin();

        let vec = [
            cos, -sin, 0.0, 0.0, //
            sin, cos, 0.0, 0.0, //
            0.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ];
        let matrix = Matrix::new_from_vec(vec);
        Transformation { matrix }
    }

    /// Return a Shear(skew) `Transformation`. each component is affected by other two components
    /// there is a total of 6 params
    /// - x in proportion to y
    /// - x in proportion to z
    /// - y in proportion to x
    /// - y in proportion to z
    /// - z in proportion to x
    /// - z in proportion to y
    pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Transformation {
        let vec = [
            1.0, x_y, x_z, 0.0, //
            y_x, 1.0, y_z, 0.0, //
            z_x, z_y, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ];
        let matrix = Matrix::new_from_vec(vec);
        Transformation { matrix }
    }

    /// Moves/Transforms the `World`, aka moving the eye/camera
    pub fn view(from: Point, to: Point, up: Vector) -> Transformation {
        let forward = (to - from).normalize();
        let left = forward.cross_product(&up.normalize());
        // true up, allows `up` to be an approximation
        let true_up = left.cross_product(&forward);

        let vec = [
            left.x, left.y, left.z, 0.0, //
            true_up.x, true_up.y, true_up.z, 0.0, //
            -forward.x, -forward.y, -forward.z, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ];
        let orientation = Matrix::new_from_vec(vec);

        Transformation {
            matrix: orientation,
        } * // moveing scene to place
	    Transformation::translation(-from.x, -from.y, -from.z)
    }
}

impl Mul<Point> for Transformation {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        rhs * self.matrix
    }
}

impl Mul<Vector> for Transformation {
    type Output = Vector;

    // Transformation doesn't affect vectors
    fn mul(self, rhs: Vector) -> Vector {
        self.matrix * rhs
    }
}

impl Mul<Transformation> for Transformation {
    type Output = Transformation;

    // Transformation doesn't affect vectors
    fn mul(self, rhs: Self) -> Self {
        Transformation {
            matrix: self.matrix * rhs.matrix,
        }
    }
}
