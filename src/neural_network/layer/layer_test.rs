use crate::neural_network::layer::layer_config::{LayerConfig, WeightsRange};

fn layer_config_with_3_neurons() -> LayerConfig {
    return LayerConfig {
        activation_function: None,
        activation_function_derivative: None,
        weights_range: WeightsRange {
            min_weight: -0.1,
            max_weight: 0.1,
        },
        neurons_count: 3,
    };
}

mod new {
    use crate::neural_network::layer::layer_test::layer_config_with_3_neurons;
    use crate::neural_network::layer::Layer;

    #[test]
    fn should_create_layer_with_3_neurons_with_2_weights() {
        let layer = Layer::new(layer_config_with_3_neurons(), 2);
        let expected_neurons_count = 3;
        assert_eq!(layer.neurons.len(), expected_neurons_count);
        layer
            .neurons
            .iter()
            .for_each(|neuron| assert_eq!(neuron.get_weights().rows_count(), 2));
    }
}

mod add_neurons {
    use crate::neural_network::layer::layer_config::WeightsRange;
    use crate::neural_network::layer::layer_test::layer_config_with_3_neurons;
    use crate::neural_network::layer::Layer;

    #[test]
    fn should_add_two_neurons_to_layer() {
        let mut layer = Layer::new(layer_config_with_3_neurons(), 2);
        let old_neurons_count = layer.neurons.len();
        layer.add_neurons(
            2,
            2,
            WeightsRange {
                min_weight: -0.1,
                max_weight: 0.1,
            },
        );
        assert_eq!(layer.neurons.len(), old_neurons_count + 2);
    }
}

mod size {
    use crate::neural_network::layer::layer_test::layer_config_with_3_neurons;
    use crate::neural_network::layer::Layer;

    #[test]
    fn should_have_size_three_for_layer_with_three_neurons() {
        let layer = Layer::new(layer_config_with_3_neurons(), 2);
        assert_eq!(layer.size(), 3);
    }
}

mod calculate_values_and_get_values {
    use crate::matrix::matrix::Matrix;
    use crate::neural_network::layer::layer_test::layer_config_with_3_neurons;
    use crate::neural_network::layer::Layer;

    #[test]
    fn should_calculate_and_save_layer_values() {
        let mut layer = Layer::new(layer_config_with_3_neurons(), 2);
        layer
            .neurons
            .iter_mut()
            .for_each(|neuron| neuron.get_mutable_weights().set_values(vec![2.0, 2.0]));
        let input = Matrix::new(2, vec![1.0, 2.0]);
        let expected_output = Matrix::new(3, vec![2.0 * 1.0 + 2.0 * 2.0; 3]);
        layer.calculate_values(&input);
        assert_eq!(
            layer.get_values().get_values(),
            expected_output.get_values()
        );
        assert_eq!(
            layer.get_values().rows_count(),
            expected_output.rows_count()
        );
        assert_eq!(
            layer.get_values().columns_count(),
            expected_output.columns_count()
        );
    }
}

mod calculate_derivative_values {
    use crate::matrix::matrix::Matrix;
    use crate::neural_network::layer::layer_test::layer_config_with_3_neurons;
    use crate::neural_network::layer::Layer;

    #[test]
    fn should_return_values_reduced_by_derivative_of_activation_function() {
        let mut layer = Layer::new(layer_config_with_3_neurons(), 2);
        let input = Matrix::new(2, vec![1.0, 2.0]);
        layer
            .neurons
            .iter_mut()
            .for_each(|neuron| neuron.get_mutable_weights().set_values(vec![2.0, 2.0]));
        layer.activation_function_derivative = |value| 0.5 * value;
        let expected_output = Matrix::new(3, vec![2.0 * 1.0 * 0.5 + 2.0 * 2.0 * 0.5; 3]);
        layer.calculate_values(&input);
        let derivative_values = layer.calculate_derivative_values();
        assert_eq!(derivative_values.get_values(), expected_output.get_values());
        assert_eq!(derivative_values.rows_count(), expected_output.rows_count());
        assert_eq!(
            derivative_values.columns_count(),
            expected_output.columns_count()
        );
    }
}

mod get_neurons_weights {
    use crate::neural_network::layer::layer_test::layer_config_with_3_neurons;
    use crate::neural_network::layer::Layer;

    #[test]
    fn should_return_all_weights_for_all_neurons() {
        let layer = Layer::new(layer_config_with_3_neurons(), 3);
        let expected_rows_count = 3;
        let expected_columns_count = 3;
        let all_weights = layer.get_neurons_weights();
        assert_eq!(all_weights.rows_count(), expected_rows_count);
        assert_eq!(all_weights.columns_count(), expected_columns_count);
    }
}

mod calculate_layer_delta {
    use crate::matrix::matrix::Matrix;
    use crate::neural_network::layer::layer_test::layer_config_with_3_neurons;
    use crate::neural_network::layer::Layer;

    #[test]
    fn should_hidden_layer_delta() {
        let mut layer = Layer::new(layer_config_with_3_neurons(), 2);
        let input = Matrix::new(2, vec![1.0, 2.0]);
        layer
            .neurons
            .iter_mut()
            .for_each(|neuron| neuron.get_mutable_weights().set_values(vec![2.0, 2.0]));
        layer.activation_function_derivative = |value| 0.5 * value;
        layer.calculate_values(&input);
        let derivative_values = Matrix::new(3, vec![3.0; 3]);
        let next_layer_delta = Matrix::new(3, vec![1.0, 2.0, 3.0]);
        let next_layer_weights = Matrix::new(3, vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0]);
        let expected_output =
            &next_layer_delta * &next_layer_weights * derivative_values.get_values();
        let result_matrix = layer.calculate_layer_delta(&next_layer_delta, &next_layer_weights);
        assert_eq!(result_matrix.get_values(), expected_output.get_values());
        assert_eq!(result_matrix.rows_count(), expected_output.rows_count());
        assert_eq!(
            result_matrix.columns_count(),
            expected_output.columns_count()
        );
    }
}

mod calculate_wights_delta {
    use crate::matrix::matrix::Matrix;
    use crate::neural_network::layer::layer_test::layer_config_with_3_neurons;
    use crate::neural_network::layer::Layer;

    #[test]
    fn should_hidden_layer_weights_delta() {
        let mut layer = Layer::new(layer_config_with_3_neurons(), 2);
        let input = Matrix::new(2, vec![1.0, 2.0]);
        let next_layer_delta = Matrix::new(3, vec![1.0, 2.0, 3.0]);
        let next_layer_weights = Matrix::new(3, vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0]);
        let expected_output = Matrix::new(2, vec![42.0, 84.0, 42.0, 84.0, 42.0, 84.0]);
        layer
            .neurons
            .iter_mut()
            .for_each(|neuron| neuron.get_mutable_weights().set_values(vec![2.0, 2.0]));
        layer.activation_function_derivative = |value| 0.5 * value;
        layer.calculate_values(&input);
        let result_matrix =
            layer.calculate_wights_delta(&input, &next_layer_delta, &next_layer_weights);
        assert_eq!(result_matrix.get_values(), expected_output.get_values());
        assert_eq!(result_matrix.rows_count(), expected_output.rows_count());
        assert_eq!(
            result_matrix.columns_count(),
            expected_output.columns_count()
        );
    }
}

mod correct_neurons_weight {
    use crate::matrix::matrix::Matrix;
    use crate::neural_network::layer::layer_test::layer_config_with_3_neurons;
    use crate::neural_network::layer::Layer;

    #[test]
    fn should_correct_neurons_weights_based_on_weights_delta() {
        let mut layer = Layer::new(layer_config_with_3_neurons(), 2);
        let learning_rate = 0.1;
        let input = Matrix::new(2, vec![1.0, 2.0]);
        let next_layer_delta = Matrix::new(3, vec![1.0, 2.0, 3.0]);
        let next_layer_weights = Matrix::new(3, vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0]);
        let expected_output = Matrix::new(2, vec![-2.2, -6.4, -2.2, -6.4, -2.2, -6.4]);
        layer
            .neurons
            .iter_mut()
            .for_each(|neuron| neuron.get_mutable_weights().set_values(vec![2.0, 2.0]));
        layer.activation_function_derivative = |value| 0.5 * value;
        layer.calculate_values(&input);
        layer.correct_neurons_weight(
            &learning_rate,
            &input,
            &next_layer_delta,
            &next_layer_weights,
        );
        assert_eq!(
            layer.get_neurons_weights().get_values(),
            expected_output.get_values()
        );
        assert_eq!(
            layer.get_neurons_weights().rows_count(),
            expected_output.rows_count()
        );
        assert_eq!(
            layer.get_neurons_weights().columns_count(),
            expected_output.columns_count()
        );
    }
}
