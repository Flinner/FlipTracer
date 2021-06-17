use std::ops::Mul;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    pub data: Vec<f64>,
}

pub mod identity {
    use super::Matrix;

    /// IDENTITY `Matrix` of size 2
    pub fn two() -> Matrix {
        Matrix {
            rows: 2,
            columns: 2,
            data: vec![1.0, 0.0, 0.0, 1.0],
        }
    }
    /// IDENTITY `Matrix` of size 3
    pub fn three() -> Matrix {
        Matrix {
            rows: 3,
            columns: 3,
            data: vec![
                1.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, //
                0.0, 0.0, 1.0,
            ],
        }
    }
    /// IDENTITY `Matrix` of size 4
    pub fn four() -> Matrix {
        Matrix {
            rows: 4,
            columns: 4,
            data: vec![
                1.0, 0.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, 0.0, //
                0.0, 0.0, 1.0, 0.0, //
                0.0, 0.0, 0.0, 1.0, //
            ],
        }
    }
}

impl Matrix {
    /// Create a new `zero Matrix`` of given size.
    pub fn new(rows: usize, columns: usize) -> Self {
        Self::new_fill_with(rows, columns, 0.0)
    }

    /// Create a new Matrix of given size, filled with `value`
    pub fn new_fill_with(rows: usize, columns: usize, value: f64) -> Self {
        Matrix::new_from_vec(rows, columns, vec![value; columns * rows])
    }

    /// Create a new Matrix of given size, filled with `vec`
    pub fn new_from_vec(rows: usize, columns: usize, vec: Vec<f64>) -> Self {
        if rows * columns != vec.len() {
            panic!("wrong number of matrix elements")
        }
        Matrix {
            rows,
            columns,
            data: vec,
        }
    }

    /// Creates Identity Square Matrix. use only when size greater than 4 is needed.
    /// other wise is `matrix::identity:TWO...`
    pub fn identity(size: usize) -> Self {
        let mut val = vec![0.0; size * size];
        for i in 0..size {
            val[size * i] = 1.0
        }
        Matrix::new_from_vec(size, size, val)
    }

    /// gets at point (x,y), zero indexed
    pub fn get(&self, row: usize, column: usize) -> f64 {
        self.data[column + row * self.columns]
    }

    /// writes at point (x,y), zero indexed
    pub fn write(&mut self, row: usize, column: usize, value: f64) {
        self.data[column + row * self.columns] = value;
    }

    /// Transposes the `Matrix`. rows are converted to columns and vice versa.
    pub fn transpose(self) -> Self {
        let mut matrix = Matrix::new(self.rows, self.columns);
        for column in 0..self.columns {
            for row in 0..self.rows {
                matrix.write(column, row, self.get(row, column));
            }
        }
        matrix
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Self;

    /// Multiply two Matrices together
    fn mul(self, rhs: Self) -> Self::Output {
        let mut output = Matrix::new(self.rows, rhs.columns);

        if self.columns == rhs.rows {
            for row in 0..self.rows {
                for column in 0..rhs.columns {
                    let mut val = 0.0;
                    for i in 0..rhs.rows {
                        val += self.get(row, i) * rhs.get(i, column)
                    }
                    output.write(row, column, val);
                }
            }
        } else {
            panic!(
                "Can't multiply Matrix with {} rows to Matrix with {} columns! ",
                self.rows, rhs.columns
            )
        }

        output
    }
}

/// Determinant of 2x2 matrix
fn determinant_2x2(matrix: Matrix) -> f64 {
    matrix.get(0, 0) * matrix.get(1, 1) - matrix.get(1, 0) * matrix.get(0, 1)
}

/// Removes `row_to_remove` and `column_to_remove`. returns a matrix that
/// is 1 row and 1 column smaller
fn submatrix(matrix: Matrix, row_to_remove: usize, column_to_remove: usize) -> Matrix {
    Matrix {
        rows: matrix.rows - 1,
        columns: matrix.columns - 1,
        data: matrix
            .data
            .iter()
            .enumerate()
            .filter_map(|(i, x)| -> Option<f64> {
                let (row, column) = (i / matrix.columns, i % matrix.columns);
                if row == row_to_remove || column == column_to_remove {
                    None
                } else {
                    Some(*x)
                }
            })
            .collect(),
    }
}

#[test]
fn submatrix_of_3x3_is_2x2() {
    let vec3 = vec![
        1.0, 2.0, 3.0, //
        5.5, 6.5, 7.5, //
        9.0, 10.0, 11.0, //
    ];
    let vec2 = vec![
        1.0, 3.0, //
        9.0, 11.0, //
    ];

    let matrix3 = Matrix::new_from_vec(3, 3, vec3);
    let matrix2 = Matrix::new_from_vec(2, 2, vec2);

    assert_eq!(submatrix(matrix3, 1, 1), matrix2)
}

#[test]
fn submatrix_of_4x4_is_3x3() {
    let vec4 = vec![
        9.0, 8.0, 6.0, 7.0, //
        1.0, 2.0, 3.0, 9.0, //
        5.5, 6.5, 7.5, 1.8, //
        9.0, 10.0, 11.0, 99.0, //
    ];
    let vec3 = vec![
        1.0, 2.0, 3.0, //
        5.5, 6.5, 7.5, //
        9.0, 10.0, 11.0, //
    ];

    let matrix4 = Matrix::new_from_vec(4, 4, vec4);
    let matrix3 = Matrix::new_from_vec(3, 3, vec3);

    assert_eq!(submatrix(matrix4, 0, 3), matrix3)
}

#[test]
fn determinant_of_2x2_matrix() {
    let vec = vec![
        -3.0, 5.0, //
        1.0, -2.0,
    ];
    let matrix = Matrix::new_from_vec(2, 2, vec);
    assert_eq!(determinant_2x2(matrix), 1.0);
}
