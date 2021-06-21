use std::ops::Mul;

use super::{
    matrix::{self, Matrix},
    point::Point,
    vector::Vector,
};

/// Wrapper around `Matrix`
#[derive(Debug, Clone)]
pub struct Transformation {
    pub matrix: Matrix,
    pub trans_type: TransformationType,
}

#[derive(Debug, Clone)]
pub enum TransformationType {
    Translation,
    Scaling,
    Rotation,
}

impl Transformation {
    /// Creates a translation, that only works with `Points` and not `Vectors`
    pub fn translation(x: f64, y: f64, z: f64) -> Transformation {
        let mut matrix = matrix::identity::four();
        matrix.write(0, 3, x);
        matrix.write(1, 3, y);
        matrix.write(2, 3, z);
        Transformation {
            matrix,
            trans_type: TransformationType::Translation,
        }
    }

    /// Inverses the Transformation
    pub fn inverse(self) -> Option<Transformation> {
        match self.matrix.inverse() {
            Some(matrix) => Some(Transformation {
                matrix,
                trans_type: self.trans_type,
            }),
            None => None,
        }
    }

    /// Return a Scaling Transformation. works for both `Point` and `Vectors`
    pub fn scaling(x: f64, y: f64, z: f64) -> Transformation {
        let vec = vec![
            x, 0.0, 0.0, 0.0, //
            0.0, y, 0.0, 0.0, //
            0.0, 0.0, z, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ];
        let matrix = Matrix::new_from_vec(4, 4, vec);
        Transformation {
            trans_type: TransformationType::Scaling,
            matrix,
        }
    }

    /// Return an X Rotation Transformation. works for both `Point` Only!
    /// takes `rad` angle in radians only!
    pub fn rotate_x(rad: f64) -> Transformation {
        let cos = rad.cos();
        let sin = rad.sin();

        let vec = vec![
            1.0, 0.0, 0.0, 0.0, //
            0.0, cos, -sin, 0.0, //
            0.0, sin, cos, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ];
        let matrix = Matrix::new_from_vec(4, 4, vec);
        Transformation {
            trans_type: TransformationType::Rotation,
            matrix,
        }
    }

    /// Return an Y Rotation Transformation. works for both `Point` Only!
    /// takes `rad` angle in radians only!
    pub fn rotate_y(rad: f64) -> Transformation {
        let cos = rad.cos();
        let sin = rad.sin();

        let vec = vec![
            cos, 0.0, sin, 0.0, //
            0.0, 1.0, 0.0, 0.0, //
            -sin, 0.0, cos, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ];
        let matrix = Matrix::new_from_vec(4, 4, vec);
        Transformation {
            trans_type: TransformationType::Rotation,
            matrix,
        }
    }

    /// Return an Z Rotation Transformation. works for both `Point` Only!
    /// takes `rad` angle in radians only!
    pub fn rotate_z(rad: f64) -> Transformation {
        let cos = rad.cos();
        let sin = rad.sin();

        let vec = vec![
            cos, -sin, 0.0, 0.0, //
            sin, cos, 0.0, 0.0, //
            0.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ];
        let matrix = Matrix::new_from_vec(4, 4, vec);
        Transformation {
            trans_type: TransformationType::Rotation,
            matrix,
        }
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
        let vec = vec![
            1.0, x_y, x_z, 0.0, //
            y_x, 1.0, y_z, 0.0, //
            z_x, z_y, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ];
        let matrix = Matrix::new_from_vec(4, 4, vec);
        Transformation {
            trans_type: TransformationType::Rotation,
            matrix,
        }
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
        match self.trans_type {
            TransformationType::Translation => rhs,
            //TransformationType::Scaling => self.matrix * rhs,
            //TransformationType::Rotation => self.matrix * rhs,
            _ => self.matrix * rhs,
        }
    }
}
