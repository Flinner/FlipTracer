use std::ops::Mul;

#[derive(Debug, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    pub data: Vec<f64>,
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

    /// gets at point (x,y), zero indexed
    pub fn get(&self, row: usize, column: usize) -> f64 {
        self.data[column + row * self.columns]
    }

    /// writes at point (x,y), zero indexed
    pub fn write(&mut self, row: usize, column: usize, value: f64) {
        self.data[column + row * self.columns] = value;
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Self;

    /// Only for 4x4 Matrices!, `panics` if can't multply
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
