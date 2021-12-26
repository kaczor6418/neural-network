use std::sync::{Arc, Mutex};
use std::thread;

pub struct Matrix {
    values: Vec<f64>,
    columns_count: usize,
    rows_count: usize,
}

impl Matrix {
    pub fn new(columns_count: usize, values: Vec<f64>) -> Matrix {
        return Matrix {
            rows_count: values.len() / columns_count,
            values,
            columns_count,
        };
    }

    pub fn new_zeros_matrix(rows_count: usize, columns_count: usize) -> Matrix {
        return Matrix {
            rows_count,
            columns_count,
            values: vec![0.0; rows_count * columns_count],
        };
    }

    pub fn new_identity_matrix(rows_count: usize, columns_count: usize) -> Matrix {
        let mut matrix = Matrix::new_zeros_matrix(rows_count, columns_count);
        let mut column_index = 0;
        for row_index in 0..rows_count {
            matrix[row_index][column_index] = 1.0;
            column_index += 1;
        }
        return matrix;
    }

    pub fn add_matrix(&self, matrix: &Matrix) -> Matrix {
        let mut matrix_iter = matrix.values.iter();
        return Matrix::new(
            matrix.columns_count,
            self.values
                .iter()
                .map(|value| value + matrix_iter.next().unwrap())
                .collect(),
        );
    }

    pub fn columns_count(&self) -> usize {
        return self.columns_count;
    }

    pub fn get_value(&self, row_index: usize, column_index: usize) -> f64 {
        return self.values[row_index * self.columns_count + column_index];
    }

    pub fn get_values(&self) -> &Vec<f64> {
        return &self.values;
    }

    pub fn kronecker_product(&self, matrix: &Matrix) -> Matrix {
        let product_rows = self.rows_count * matrix.rows_count;
        let product_columns_count = self.columns_count * matrix.columns_count;
        let mut product = Matrix::new_zeros_matrix(product_rows, product_columns_count);
        for m1_row_index in 0..self.rows_count {
            for m1_column_index in 0..self.columns_count {
                for m2_row_index in 0..matrix.rows_count {
                    for m2_column_index in 0..matrix.columns_count {
                        let product_row_index = m1_row_index * matrix.rows_count + m2_row_index;
                        let product_column_index =
                            m1_column_index * matrix.columns_count + m2_column_index;
                        product[product_row_index][product_column_index] = self[m1_row_index]
                            [m1_column_index]
                            * matrix[m2_row_index][m2_column_index];
                    }
                }
            }
        }
        return product;
    }

    pub fn outer_product(&self, matrix: &Matrix) -> Matrix {
        let product_rows = self.rows_count * matrix.rows_count;
        let product_columns_count = self.columns_count * matrix.columns_count;
        let mut product = Matrix::new_zeros_matrix(product_rows, product_columns_count);
        for m2_row_index in 0..matrix.rows_count {
            for m2_column_index in 0..matrix.columns_count {
                for m1_row_index in 0..self.rows_count {
                    for m1_column_index in 0..self.columns_count {
                        let product_row_index = m2_row_index * self.rows_count + m1_row_index;
                        let product_column_index =
                            m2_column_index * self.columns_count + m1_column_index;
                        product[product_row_index][product_column_index] = self[m1_row_index]
                            [m1_column_index]
                            * matrix[m2_row_index][m2_column_index];
                    }
                }
            }
        }
        return product;
    }

    pub fn parallel_kronecker_product(&self, matrix: &Matrix) -> Matrix {
        let product_rows = self.rows_count * matrix.rows_count;
        let product_columns_count = self.columns_count * matrix.columns_count;
        let product = Arc::new(Mutex::new(Matrix::new_zeros_matrix(
            product_rows,
            product_columns_count,
        )));
        let mut handles = vec![];
        for m2_row_index in 0..matrix.rows_count {
            let product = Arc::clone(&product);
            let matrix_a = self.to_owned();
            let matrix_b = matrix.to_owned();
            let handle = thread::spawn(move || {
                let mut prod = product.lock().unwrap();
                for m2_column_index in 0..matrix_b.columns_count {
                    for m1_row_index in 0..matrix_a.rows_count {
                        for m1_column_index in 0..matrix_a.columns_count {
                            let product_row_index =
                                m2_row_index * matrix_a.rows_count + m1_row_index;
                            let product_column_index =
                                m2_column_index * matrix_a.columns_count + m1_column_index;
                            (*prod)[product_row_index][product_column_index] = matrix_a
                                [m1_row_index][m1_column_index]
                                * matrix_b[m2_row_index][m2_column_index];
                        }
                    }
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        return product.lock().unwrap().clone();
    }

    pub fn join_horizontal(&self, matrix: &Matrix) -> Matrix {
        let total_columns_count = self.columns_count + matrix.columns_count;
        let mut output_matrix = Matrix::new_zeros_matrix(self.rows_count, total_columns_count);
        for row_index in 0..self.rows_count {
            for column_index in 0..self.columns_count {
                output_matrix[row_index][column_index] = self[row_index][column_index];
            }
            for column_index in self.columns_count..total_columns_count {
                output_matrix[row_index][column_index] =
                    matrix[row_index][column_index - self.columns_count]
            }
        }
        return output_matrix;
    }

    pub fn join_vertical(&self, matrix: &Matrix) -> Matrix {
        let mut values = self.values.clone();
        values.extend(&matrix.values);
        return Matrix::new(self.columns_count, values);
    }

    pub fn multiply_by_matrix(&self, matrix: &Matrix) -> Matrix {
        let mut result: Vec<f64> = vec![0.0; self.rows_count * matrix.columns_count];
        for row_index in 0..self.rows_count {
            for column_index in 0..matrix.columns_count {
                result[row_index * matrix.columns_count + column_index] = self
                    .multiply_row_by_column(
                        self.get_matrix_row(row_index),
                        matrix.get_matrix_column(column_index),
                    );
            }
        }
        return Matrix::new(matrix.columns_count, result);
    }

    pub fn multiply_by_vector(&self, values: &Vec<f64>) -> Matrix {
        let mut new_values: Vec<f64> = vec![];
        for index in 0..values.len() {
            new_values.push(self.values[index] * values[index])
        }
        return Matrix::new(self.columns_count, new_values);
    }

    pub fn multiply_by_digit(&self, digit: f64) -> Matrix {
        return Matrix::new(
            self.columns_count,
            self.values.iter().map(|value| value * digit).collect(),
        );
    }

    pub fn set_value(&mut self, row_index: usize, column_index: usize, value: f64) {
        self.values[row_index * self.columns_count + column_index] = value;
    }

    pub fn set_values(&mut self, values: Vec<f64>) {
        self.values = values;
    }

    pub fn subtract_matrix(&self, matrix: &Matrix) -> Matrix {
        let mut matrix_iter = matrix.values.iter();
        return Matrix::new(
            matrix.columns_count,
            self.values
                .iter()
                .map(|value| value - matrix_iter.next().unwrap())
                .collect(),
        );
    }

    pub fn rows_count(&self) -> usize {
        return self.rows_count;
    }

    pub fn transpose(&self) -> Matrix {
        let mut values: Vec<f64> = vec![];
        for column_index in 0..self.columns_count {
            values = values
                .into_iter()
                .chain(self.get_matrix_column(column_index).into_iter())
                .collect();
        }
        return Matrix::new(self.rows_count, values);
    }

    fn get_matrix_row(&self, row_index: usize) -> &[f64] {
        return &self.values
            [row_index * self.columns_count..row_index * self.columns_count + self.columns_count];
    }

    fn get_mutable_matrix_row(&mut self, row_index: usize) -> &mut [f64] {
        return &mut self.values
            [row_index * self.columns_count..row_index * self.columns_count + self.columns_count];
    }

    fn get_matrix_column(&self, column_index: usize) -> Vec<f64> {
        return self
            .values
            .iter()
            .skip(column_index)
            .step_by(self.columns_count)
            .copied()
            .collect();
    }

    fn multiply_row_by_column(&self, row: &[f64], column: Vec<f64>) -> f64 {
        let mut sum = 0.0;
        for index in 0..row.len() {
            sum += row[index] * column[index];
        }
        return sum;
    }
}

mod matrix_clone;
mod matrix_operators;

#[cfg(test)]
mod matrix_test;
