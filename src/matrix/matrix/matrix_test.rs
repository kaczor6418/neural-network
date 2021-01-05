mod new {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_create_matrix_with_three_columns_and_two_rows() {
        let columns_count = 3;
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let matrix = Matrix::new(columns_count, values.clone());
        assert_eq!(matrix.columns_count, columns_count);
        assert_eq!(matrix.rows_count, values.len() / columns_count);
    }
}

mod multiply_matrix {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_multiply_matrix_with_many_rows_and_columns_if_matrix_rows_count_equal_target_matrix_columns_count(
    ) {
        let matrix_a_values = vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0];
        let matrix_b_values = vec![2.0, 2.0, 2.0, 3.0, 3.0, 3.0];
        let expected_result = Matrix::new(2, vec![7.0, 8.0, 14.0, 16.0]);
        let matrix_a = Matrix::new(3, matrix_a_values);
        let matrix_b = Matrix::new(2, matrix_b_values);
        let result_matrix = matrix_a.multiply_by_matrix(matrix_b);
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }

    #[test]
    fn should_multiply_matrix_with_one_row_if_target_matrix_has_one_column() {
        let matrix_a_values = vec![1.0, 2.0, 3.0];
        let matrix_b_values = vec![1.0, 2.0, 3.0];
        let expected_result = Matrix::new(1, vec![14.0]);
        let matrix_a = Matrix::new(3, matrix_a_values);
        let matrix_b = Matrix::new(1, matrix_b_values);
        let result_matrix = matrix_a.multiply_by_matrix(matrix_b);
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }
}

mod multiply_by_digit {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_multiply_all_matrix_values_by_given_digit() {
        let digit = 2.0;
        let matrix_values = vec![1.0, 2.0, 3.0];
        let expected_result =
            Matrix::new(3, matrix_values.iter().map(|value| value * digit).collect());
        let matrix = Matrix::new(3, matrix_values);
        let result_matrix = matrix.multiply_by_digit(digit);
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }
}
