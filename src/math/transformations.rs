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
            TransformationType::Scaling => rhs * self.matrix,
        }
    }
}
