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
        for column in 0..4 {
            for row in 0..4 {
                matrix.write(column, row, self.get(row, column));
            }
        }
        matrix
    }

    /// Find `Matrix` determinant. Might not work with greater than 4x4.
    pub fn determinant(&self) -> f64 {
        let m11 = matrix_cofactor(self, 0, 0);
        let m21 = matrix_cofactor(self, 1, 0);
        let m31 = matrix_cofactor(self, 2, 0);
        let m41 = matrix_cofactor(self, 3, 0);

        let e11 = self.get(0, 0);
        let e21 = self.get(1, 0);
        let e31 = self.get(2, 0);
        let e41 = self.get(3, 0);

        e11 * m11 //.
	    + e21 * m21//.
	    + e31 * m31//.
	    + e41 * m41
    }

    /// Inverses `Matrix`, returns None if can't inverse (`Matrix.determinant ==0 `)
    pub fn inverse(&self) -> Option<Self> {
        let mut cofactors: [f64; 16] = [0.0; 16];
        let determinant = self.determinant();

        if determinant == 0.0 {
            None
        } else {
            for row in 0..4 {
                for column in 0..4 {
                    cofactors[column + row * 4] = matrix_cofactor(self, row, column);
                }
            }
            Some(Matrix::new_from_vec(cofactors).transpose() / determinant)
        }
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Self;

    /// Multiply two Matrices together
    fn mul(self, rhs: Self) -> Self::Output {
        let mut output = Matrix::new();

        // if self.columns == rhs.rows {
        for row in 0..4 {
            for column in 0..4 {
                let mut val = 0.0;
                for i in 0..4 {
                    val += self.get(row, i) * rhs.get(i, column)
                }
                output.write(row, column, val);
            }
        }
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
        for (i, e) in self.data.iter().enumerate() {
            f[i] = e / rhs;
        }
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

/// Removes `row` and `column`. and finds determinant of the matrix.
fn matrix_minor(matrix: &Matrix, row: usize, column: usize) -> f64 {
    let sub = submatrix(matrix, row, column);
    let e11 = sub[0];
    let e12 = sub[1];
    let e13 = sub[2];
    let e21 = sub[3];
    let e22 = sub[4];
    let e23 = sub[5];
    let e31 = sub[6];
    let e32 = sub[7];
    let e33 = sub[8];

    0.0 //.
	+ e11 * e22 * e33 //.
	+ e12 * e23 * e31 //.
	+ e13 * e21 * e32 //.
	- e11 * e23 * e32 //.
	- e12 * e21 * e33 //.
	- e13 * e22 * e31 //.
}

/// Removes `row` and `column`. and finds determinant of the matrix. with the correct sign
#[inline]
fn matrix_cofactor(matrix: &Matrix, row: usize, column: usize) -> f64 {
    matrix_minor(matrix, row, column) * (if (row + column) % 2 != 0 { -1.0 } else { 1.0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn submatrix_of_4x4_is_3x3() {
        let vec4 = [
            9.0, 8.0, 6.0, 7.0, //
            1.0, 2.0, 3.0, 9.0, //
            5.5, 6.5, 7.5, 1.8, //
            9.0, 10.0, 11.0, 99.0, //
        ];
        let vec3 = [
            1.0, 2.0, 3.0, //
            5.5, 6.5, 7.5, //
            9.0, 10.0, 11.0, //
        ];

        let matrix4 = Matrix::new_from_vec(vec4);

        assert_eq!(submatrix(&matrix4, 0, 3), vec3);
    }

    #[test]
    fn determinant_of_3x3_matrix() {
        let vec3 = [
            1.0, 2.0, 6.0, 0.0, //
            -5.0, 8.0, -4.0, 0.0, //
            2.0, 6.0, 4.0, 0.0, //
            0.0, 0.0, 0.0, 0.0,
        ];
        let matrix3 = Matrix::new_from_vec(vec3);

        assert_eq!(matrix_minor(&matrix3, 3, 3), -196.0);
    }
}
