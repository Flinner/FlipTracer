use super::vector::Vector;
use std::ops::{Div, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix {
    // pub rows: usize, // always 4
    // pub columns: usize,
    pub data: [f64; 16],
}

impl Matrix {
    /// Create a new `zero Matrix`` of given size.
    pub fn new() -> Self {
        Self::new_fill_with(0.0)
    }

    /// Create a new Matrix of given size, filled with `value`
    pub fn new_fill_with(value: f64) -> Self {
        Matrix::new_from_vec([value; 16])
    }

    /// Create a new Matrix of given size, filled with `vec`
    pub fn new_from_vec(vec: [f64; 16]) -> Self {
        Matrix { data: vec }
    }

    /// Creates Identity Square Matrix. use only when size greater than 4 is needed.
    /// other wise is `matrix::identity:TWO...`
    pub fn identity() -> Self {
        Matrix {
            data: [
                1.0, 0.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, 0.0, //
                0.0, 0.0, 1.0, 0.0, //
                0.0, 0.0, 0.0, 1.0, //
            ],
        }
    }

    /// gets at point (x,y), zero indexed
    pub fn get(&self, row: usize, column: usize) -> f64 {
        self.data[column + row * 4]
    }

    /// writes at point (x,y), zero indexed
    pub fn write(&mut self, row: usize, column: usize, value: f64) {
        self.data[column + row * 4] = value;
    }

    /// Transposes the `Matrix`. rows are converted to columns and vice versa.
    pub fn transpose(self) -> Self {
        let mut matrix = Matrix::new();
        (0..4).for_each(|column| {
            (0..4).for_each(|row| {
                matrix.write(column, row, self.get(row, column));
            });
        });
        matrix
    }

    /// Find `Matrix` determinant. Might not work with greater than 4x4.
    pub fn determinant(&self) -> f64 {
        let m = self.data;

        let inv0 = m[5] * m[10] * m[15] - m[5] * m[11] * m[14] - m[9] * m[6] * m[15]
            + m[9] * m[7] * m[14]
            + m[13] * m[6] * m[11]
            - m[13] * m[7] * m[10];

        let inv1 = -m[4] * m[10] * m[15] + m[4] * m[11] * m[14] + m[8] * m[6] * m[15]
            - m[8] * m[7] * m[14]
            - m[12] * m[6] * m[11]
            + m[12] * m[7] * m[10];

        let inv2 = m[4] * m[9] * m[15] - m[4] * m[11] * m[13] - m[8] * m[5] * m[15]
            + m[8] * m[7] * m[13]
            + m[12] * m[5] * m[11]
            - m[12] * m[7] * m[9];

        let inv3 = -m[4] * m[9] * m[14] + m[4] * m[10] * m[13] + m[8] * m[5] * m[14]
            - m[8] * m[6] * m[13]
            - m[12] * m[5] * m[10]
            + m[12] * m[6] * m[9];

        m[0] * inv0 + m[1] * inv1 + m[2] * inv2 + m[3] * inv3 // determinant
    }

    /// Inverses `Matrix`, returns None if can't inverse (`Matrix.determinant ==0 `)
    pub fn inverse(&self) -> Option<Self> {
        let m = self.data;
        let mut inv: [f64; 16] = [0.0; 16];

        inv[0] = m[5] * m[10] * m[15] - m[5] * m[11] * m[14] - m[9] * m[6] * m[15]
            + m[9] * m[7] * m[14]
            + m[13] * m[6] * m[11]
            - m[13] * m[7] * m[10];

        inv[4] = -m[4] * m[10] * m[15] + m[4] * m[11] * m[14] + m[8] * m[6] * m[15]
            - m[8] * m[7] * m[14]
            - m[12] * m[6] * m[11]
            + m[12] * m[7] * m[10];

        inv[8] = m[4] * m[9] * m[15] - m[4] * m[11] * m[13] - m[8] * m[5] * m[15]
            + m[8] * m[7] * m[13]
            + m[12] * m[5] * m[11]
            - m[12] * m[7] * m[9];

        inv[12] = -m[4] * m[9] * m[14] + m[4] * m[10] * m[13] + m[8] * m[5] * m[14]
            - m[8] * m[6] * m[13]
            - m[12] * m[5] * m[10]
            + m[12] * m[6] * m[9];

        inv[1] = -m[1] * m[10] * m[15] + m[1] * m[11] * m[14] + m[9] * m[2] * m[15]
            - m[9] * m[3] * m[14]
            - m[13] * m[2] * m[11]
            + m[13] * m[3] * m[10];

        inv[5] = m[0] * m[10] * m[15] - m[0] * m[11] * m[14] - m[8] * m[2] * m[15]
            + m[8] * m[3] * m[14]
            + m[12] * m[2] * m[11]
            - m[12] * m[3] * m[10];

        inv[9] = -m[0] * m[9] * m[15] + m[0] * m[11] * m[13] + m[8] * m[1] * m[15]
            - m[8] * m[3] * m[13]
            - m[12] * m[1] * m[11]
            + m[12] * m[3] * m[9];

        inv[13] = m[0] * m[9] * m[14] - m[0] * m[10] * m[13] - m[8] * m[1] * m[14]
            + m[8] * m[2] * m[13]
            + m[12] * m[1] * m[10]
            - m[12] * m[2] * m[9];

        inv[2] = m[1] * m[6] * m[15] - m[1] * m[7] * m[14] - m[5] * m[2] * m[15]
            + m[5] * m[3] * m[14]
            + m[13] * m[2] * m[7]
            - m[13] * m[3] * m[6];

        inv[6] = -m[0] * m[6] * m[15] + m[0] * m[7] * m[14] + m[4] * m[2] * m[15]
            - m[4] * m[3] * m[14]
            - m[12] * m[2] * m[7]
            + m[12] * m[3] * m[6];

        inv[10] = m[0] * m[5] * m[15] - m[0] * m[7] * m[13] - m[4] * m[1] * m[15]
            + m[4] * m[3] * m[13]
            + m[12] * m[1] * m[7]
            - m[12] * m[3] * m[5];

        inv[14] = -m[0] * m[5] * m[14] + m[0] * m[6] * m[13] + m[4] * m[1] * m[14]
            - m[4] * m[2] * m[13]
            - m[12] * m[1] * m[6]
            + m[12] * m[2] * m[5];

        inv[3] = -m[1] * m[6] * m[11] + m[1] * m[7] * m[10] + m[5] * m[2] * m[11]
            - m[5] * m[3] * m[10]
            - m[9] * m[2] * m[7]
            + m[9] * m[3] * m[6];

        inv[7] = m[0] * m[6] * m[11] - m[0] * m[7] * m[10] - m[4] * m[2] * m[11]
            + m[4] * m[3] * m[10]
            + m[8] * m[2] * m[7]
            - m[8] * m[3] * m[6];

        inv[11] = -m[0] * m[5] * m[11] + m[0] * m[7] * m[9] + m[4] * m[1] * m[11]
            - m[4] * m[3] * m[9]
            - m[8] * m[1] * m[7]
            + m[8] * m[3] * m[5];

        inv[15] = m[0] * m[5] * m[10] - m[0] * m[6] * m[9] - m[4] * m[1] * m[10]
            + m[4] * m[2] * m[9]
            + m[8] * m[1] * m[6]
            - m[8] * m[2] * m[5];

        let mut det = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];

        if det == 0.0 {
            None
        } else {
            det = 1.0 / det;
            (0..16).for_each(|i| {
                inv[i] *= det;
            });
            Some(Matrix { data: inv })
        }
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::new()
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Self;

    /// Multiply two Matrices together
    fn mul(self, rhs: Self) -> Self::Output {
        let mut output = Matrix::new();

        // if self.columns == rhs.rows {
        (0..4).for_each(|row| {
            (0..4).for_each(|column| {
                let mut val = 0.0;
                (0..4).for_each(|i| val += self.get(row, i) * rhs.get(i, column));
                output.write(row, column, val);
            });
        });
        output
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    /// using `Mul` (`*`) for multiplying `Matrix` * `Vector` for Translations
    ///```text
    /// [ 1.0,   2.0,  3.0,  4.0,        [x,
    ///   5.0,   6.0,  7.0,  8.0,   Mul   y
    ///   9.0,  10.0, 11.0, 12.0,   Mul   z
    ///   13.0, 14.0, 15.0, 15.0,]        1.0]
    ///```
    fn mul(self, rhs: Vector) -> Vector {
        let m = self.data;
        let v = rhs;

        Vector {
            x: m[0] * v.x + m[1] * v.y + m[2] * v.z,
            y: m[4] * v.x + m[5] * v.y + m[6] * v.z,
            z: m[8] * v.x + m[9] * v.y + m[10] * v.z,
        }
    }
}

impl Div<f64> for Matrix {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        let mut f = self.data;
        self.data.iter().enumerate().for_each(|(i, e)| {
            f[i] = e / rhs;
        });
        Matrix::new_from_vec(f)
    }
}

/// Removes `row_to_remove` and `column_to_remove`. returns an Array that
/// is 1 row and 1 column smaller, aka 3x3
pub fn submatrix(matrix: &Matrix, row_to_remove: usize, column_to_remove: usize) -> [f64; 9] {
    let mut a: [f64; 9] = [0.0; 9];
    let mut i_ = 0;
    for (i, _) in matrix.data.iter().enumerate() {
        let (row, column) = (i / 4, i % 4);
        if column == column_to_remove || row == row_to_remove {
            continue;
        }
        a[i_] = matrix.data[i];
        i_ += 1;
    }
    a
}
