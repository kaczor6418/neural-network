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

    pub fn multiply_by_matrix(&self, matrix_b: Matrix) -> Matrix {
        if matrix_b.columns_count == 1 {
            let mut sum = 0.0;
            for index in 0..matrix_b.rows_count {
                sum += self.values[index] * matrix_b.values[index];
            }
            return Matrix::new(1, vec![sum]);
        }
        let mut result: Vec<f64> = vec![0.0; self.rows_count * matrix_b.columns_count];
        for row_index in 0..self.rows_count {
            for column_index in 0..matrix_b.columns_count {
                result[row_index * matrix_b.columns_count + column_index] = self
                    .get_matrix_row(row_index)
                    .multiply_by_matrix(matrix_b.get_matrix_column(column_index))
                    .values[0]
            }
        }
        return Matrix::new(matrix_b.columns_count, result);
    }

    pub fn multiply_by_digit(&self, digit: f64) -> Matrix {
        return Matrix::new(
            self.columns_count,
            self.values.iter().map(|value| value * digit).collect(),
        );
    }

    fn get_matrix_row(&self, row_index: usize) -> Matrix {
        let row = self.values
            [row_index * self.columns_count..row_index * self.columns_count + self.columns_count]
            .to_vec();
        return Matrix::new(row.len(), row);
    }

    fn get_matrix_column(&self, column_index: usize) -> Matrix {
        let column = self
            .values
            .iter()
            .skip(column_index)
            .step_by(self.columns_count)
            .copied()
            .collect();
        return Matrix::new(1, column);
    }
}

#[cfg(test)]
mod matrix_test;
