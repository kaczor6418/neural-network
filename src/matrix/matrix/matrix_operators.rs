use crate::matrix::matrix::Matrix;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

impl Add<Matrix> for Matrix {
    type Output = Matrix;
    fn add(self, matrix: Matrix) -> Matrix {
        return self.add_matrix(matrix);
    }
}

impl Sub<Matrix> for Matrix {
    type Output = Matrix;
    fn sub(self, matrix: Matrix) -> Matrix {
        return self.subtract_matrix(matrix);
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, matrix: Matrix) -> Matrix {
        return self.multiply_by_matrix(matrix);
    }
}

impl Mul<f64> for Matrix {
    type Output = Matrix;
    fn mul(self, digit: f64) -> Matrix {
        return self.multiply_by_digit(digit);
    }
}

impl Mul<Matrix> for f64 {
    type Output = Matrix;
    fn mul(self, matrix: Matrix) -> Matrix {
        return matrix.multiply_by_digit(self);
    }
}

impl Index<usize> for Matrix {
    type Output = [f64];
    fn index(&self, row_index: usize) -> &[f64] {
        return self.get_matrix_row(row_index);
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, row_index: usize) -> &mut [f64] {
        return self.get_mutable_matrix_row(row_index);
    }
}
