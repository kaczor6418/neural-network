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

mod matrix_operators;

#[cfg(test)]
mod matrix_test;
