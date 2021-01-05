pub struct Matrix {
    values: Vec<f64>,
    columns_count: usize,
    rows_count: usize,
}

impl Matrix {
    pub fn new(columns_count: usize, mut values: Option<Vec<f64>>) -> Matrix {
        return match values {
            Some(initial_values) => Matrix {
                rows_count: initial_values.len() / columns_count,
                values: initial_values,
                columns_count,
            },
            None => Matrix {
                rows_count: 1,
                values: vec![0.0; columns_count],
                columns_count,
            },
        };
    }

    pub fn add(&self, matrix: Matrix) -> Matrix {
        let mut matrix_iter = matrix.values.iter();
        return Matrix::new(
            matrix.columns_count,
            Some(
                self.values
                    .iter()
                    .map(|value| value + matrix_iter.next().unwrap())
                    .collect(),
            ),
        );
    }

    pub fn get_value(&self, row_index: usize, column_index: usize) -> f64 {
        return self.values[row_index * self.columns_count + column_index];
    }

    pub fn multiply_by_matrix(&self, matrix_b: Matrix) -> Matrix {
        if matrix_b.columns_count == 1 {
            let mut sum = 0.0;
            for index in 0..matrix_b.rows_count {
                sum += self.values[index] * matrix_b.values[index];
            }
            return Matrix::new(1, Some(vec![sum]));
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
        return Matrix::new(matrix_b.columns_count, Some(result));
    }

    pub fn multiply_by_digit(&self, digit: f64) -> Matrix {
        return Matrix::new(
            self.columns_count,
            Some(self.values.iter().map(|value| value * digit).collect()),
        );
    }

    pub fn set_value(&mut self, row_index: usize, column_index: usize, value: f64) {
        self.values[row_index * self.columns_count + column_index] = value;
    }

    pub fn set_values(&mut self, values: Vec<f64>) {
        self.values = values;
    }

    pub fn subtract(&self, matrix: Matrix) -> Matrix {
        let mut matrix_iter = matrix.values.iter();
        return Matrix::new(
            matrix.columns_count,
            Some(
                self.values
                    .iter()
                    .map(|value| value - matrix_iter.next().unwrap())
                    .collect(),
            ),
        );
    }

    pub fn transpose(&self) -> Matrix {
        let mut values: Vec<f64> = vec![];
        for column_index in 0..self.columns_count {
            values = values
                .into_iter()
                .chain(self.get_matrix_column(column_index).values.into_iter())
                .collect();
        }
        return Matrix::new(self.rows_count, Some(values));
    }

    fn get_matrix_row(&self, row_index: usize) -> Matrix {
        let row = self.values
            [row_index * self.columns_count..row_index * self.columns_count + self.columns_count]
            .to_vec();
        return Matrix::new(row.len(), Some(row));
    }

    fn get_matrix_column(&self, column_index: usize) -> Matrix {
        let column = Some(
            self.values
                .iter()
                .skip(column_index)
                .step_by(self.columns_count)
                .copied()
                .collect(),
        );
        return Matrix::new(1, column);
    }
}

#[cfg(test)]
mod matrix_test;
