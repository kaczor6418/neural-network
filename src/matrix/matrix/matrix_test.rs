mod new {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_create_matrix_with_three_columns_and_two_rows() {
        let columns_count = 3;
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let matrix = Matrix::new(columns_count, Some(values.clone()));
        assert_eq!(matrix.columns_count, columns_count);
        assert_eq!(matrix.rows_count, values.len() / columns_count);
    }

    #[test]
    fn should_create_matrix_of_zeros_if_values_is_none() {
        let columns_count = 3;
        let matrix = Matrix::new(columns_count, None);
        assert_eq!(matrix.columns_count, columns_count);
        assert_eq!(matrix.values, vec![0.0; columns_count]);
    }
}

mod multiply_matrix {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_multiply_matrix_with_many_rows_and_columns_if_matrix_rows_count_equal_target_matrix_columns_count(
    ) {
        let matrix_a_values = vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0];
        let matrix_b_values = vec![2.0, 2.0, 2.0, 3.0, 3.0, 3.0];
        let expected_result = Matrix::new(2, Some(vec![7.0, 8.0, 14.0, 16.0]));
        let matrix_a = Matrix::new(3, Some(matrix_a_values));
        let matrix_b = Matrix::new(2, Some(matrix_b_values));
        let result_matrix = matrix_a.multiply_by_matrix(matrix_b);
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }

    #[test]
    fn should_multiply_matrix_with_many_rows_and_columns_if_matrix_rows_count_equal_target_matrix_columns_count_with_overloaded_nul_operator(
    ) {
        let matrix_a_values = vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0];
        let matrix_b_values = vec![2.0, 2.0, 2.0, 3.0, 3.0, 3.0];
        let expected_result = Matrix::new(2, Some(vec![7.0, 8.0, 14.0, 16.0]));
        let matrix_a = Matrix::new(3, Some(matrix_a_values));
        let matrix_b = Matrix::new(2, Some(matrix_b_values));
        let result_matrix = matrix_a * matrix_b;
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }

    #[test]
    fn should_multiply_matrix_with_one_row_if_target_matrix_has_one_column() {
        let matrix_a_values = vec![1.0, 2.0, 3.0];
        let matrix_b_values = vec![1.0, 2.0, 3.0];
        let expected_result = Matrix::new(1, Some(vec![14.0]));
        let matrix_a = Matrix::new(3, Some(matrix_a_values));
        let matrix_b = Matrix::new(1, Some(matrix_b_values));
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
        let expected_result = Matrix::new(
            3,
            Some(matrix_values.iter().map(|value| value * digit).collect()),
        );
        let matrix = Matrix::new(3, Some(matrix_values));
        let result_matrix = matrix.multiply_by_digit(digit);
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }

    #[test]
    fn should_multiply_all_matrix_values_by_given_digit_with_overloaded_mul_right_operator() {
        let digit = 2.0;
        let matrix_values = vec![1.0, 2.0, 3.0];
        let expected_result = Matrix::new(
            3,
            Some(matrix_values.iter().map(|value| value * digit).collect()),
        );
        let matrix = Matrix::new(3, Some(matrix_values));
        let result_matrix = matrix * digit;
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }

    #[test]
    fn should_multiply_all_matrix_values_by_given_digit_with_overloaded_mul_left_operator() {
        let digit = 2.0;
        let matrix_values = vec![1.0, 2.0, 3.0];
        let expected_result = Matrix::new(
            3,
            Some(matrix_values.iter().map(|value| value * digit).collect()),
        );
        let matrix = Matrix::new(3, Some(matrix_values));
        let result_matrix = digit * matrix;
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }
}

mod subtract {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_subtract_each_element_of_matrix_a_and_matrix_b() {
        let matrix_a_values = vec![1.0, 2.0, 3.0];
        let matrix_b_values = vec![3.0, 2.0, 1.0];
        let expected_result = Matrix::new(3, Some(vec![-2.0, 0.0, 2.0]));
        let matrix_a = Matrix::new(3, Some(matrix_a_values));
        let matrix_b = Matrix::new(3, Some(matrix_b_values));
        let result_matrix = matrix_a.subtract_matrix(matrix_b);
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }

    #[test]
    fn should_subtract_each_element_of_matrix_a_and_matrix_b_with_overloaded_subtract_operator() {
        let matrix_a_values = vec![1.0, 2.0, 3.0];
        let matrix_b_values = vec![3.0, 2.0, 1.0];
        let expected_result = Matrix::new(3, Some(vec![-2.0, 0.0, 2.0]));
        let matrix_a = Matrix::new(3, Some(matrix_a_values));
        let matrix_b = Matrix::new(3, Some(matrix_b_values));
        let result_matrix = matrix_a - matrix_b;
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }
}

mod add {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_add_each_element_of_matrix_a_and_matrix_b() {
        let matrix_a_values = vec![1.0, 2.0, 3.0];
        let matrix_b_values = vec![3.0, 2.0, 1.0];
        let expected_result = Matrix::new(3, Some(vec![4.0, 4.0, 4.0]));
        let matrix_a = Matrix::new(3, Some(matrix_a_values));
        let matrix_b = Matrix::new(3, Some(matrix_b_values));
        let result_matrix = matrix_a.add_matrix(matrix_b);
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }

    #[test]
    fn should_add_each_element_of_matrix_a_and_matrix_b_with_overloaded_add_operator() {
        let matrix_a_values = vec![1.0, 2.0, 3.0];
        let matrix_b_values = vec![3.0, 2.0, 1.0];
        let expected_result = Matrix::new(3, Some(vec![4.0, 4.0, 4.0]));
        let matrix_a = Matrix::new(3, Some(matrix_a_values));
        let matrix_b = Matrix::new(3, Some(matrix_b_values));
        let result_matrix = matrix_a + matrix_b;
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }
}

mod transpose {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_transpose_2_x_3_into_3_x_2_matrix() {
        let matrix_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let expected_result = Matrix::new(2, Some(vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]));
        let matrix = Matrix::new(3, Some(matrix_values));
        let result_matrix = matrix.transpose();
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }
}

mod get_value {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_return_value_under_given_position() {
        let matrix_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let expected_result = 5.0;
        let matrix = Matrix::new(3, Some(matrix_values));
        let result = matrix.get_value(1, 1);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_value_under_given_position_by_overloaded_index_operator() {
        let matrix_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let expected_result = 5.0;
        let matrix = Matrix::new(3, Some(matrix_values));
        let result = matrix[1][1];
        assert_eq!(result, expected_result);
    }
}

mod set_value {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_set_value_under_given_position() {
        let matrix_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let row_index = 1;
        let column_index = 1;
        let new_value = 10.0;
        let mut matrix = Matrix::new(3, Some(matrix_values));
        matrix.set_value(row_index, column_index, new_value);
        assert_eq!(matrix.get_value(row_index, column_index), new_value);
    }

    #[test]
    fn should_set_value_under_given_position_with_overloaded_index_operator() {
        let matrix_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let row_index = 1;
        let column_index = 1;
        let new_value = 10.0;
        let mut matrix = Matrix::new(3, Some(matrix_values));
        matrix[row_index][column_index] = new_value;
        assert_eq!(matrix.get_value(row_index, column_index), new_value);
    }
}

mod columns_count {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_create_matrix_with_given_number_of_columns() {
        let columns_count = 6;
        let mut matrix = Matrix::new(columns_count, None);
        assert_eq!(matrix.columns_count(), columns_count);
    }
}

mod rows_count {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_create_matrix_with_calculated_number_of_rows() {
        let columns_count = 2;
        let matrix_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let rows_count = matrix_values.len() / columns_count;
        let mut matrix = Matrix::new(columns_count, Some(matrix_values));
        assert_eq!(matrix.rows_count(), rows_count);
    }
}

mod set_values {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_set_all_values() {
        let matrix_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let new_matrix_values = vec![5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let mut matrix = Matrix::new(3, Some(matrix_values));
        matrix.set_values(new_matrix_values.clone());
        assert_eq!(matrix.values, new_matrix_values);
    }
}
