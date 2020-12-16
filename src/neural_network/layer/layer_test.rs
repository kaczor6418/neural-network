mod new {
    use crate::neural_network::layer::Layer;

    #[test]
    fn should_create_empty_neural_layer() {
        let empty_layer = Layer::new(&None, &None);
        assert_eq!(empty_layer.neurons.len(), 0);
    }
}

mod size {
    use crate::neural_network::layer::Layer;

    #[test]
    fn should_have_size_one_for_layer_with_one_neuron() {
        let mut layer = Layer::new(&None, &None);
        layer.add_neurons(&1, &3, &0.0, &1.0);
        assert_eq!(layer.size(), 1);
    }

    #[test]
    fn should_have_size_ten_for_layer_with_ten_neurons() {
        let mut layer = Layer::new(&None, &None);
        layer.add_neurons(&10, &3, &0.0, &1.0);
        assert_eq!(layer.size(), 10);
    }
}

mod add_neurons {
    use crate::neural_network::layer::Layer;

    #[test]
    fn should_add_one_neuron_to_empty_layer() {
        let mut layer = Layer::new(&None, &None);
        layer.add_neurons(&1, &3, &0.0, &1.0);
        assert_eq!(layer.neurons.len(), 1);
    }

    #[test]
    fn should_add_ten_neurons_to_layer_with_neurons() {
        let mut layer = Layer::new(&None, &None);
        layer.add_neurons(&10, &3, &0.0, &1.0);
        assert_eq!(layer.neurons.len(), 10);
    }
}

mod calculate_outputs {
    use crate::neural_network::layer::Layer;

    #[test]
    fn should_return_one_output_for_one_neuron() {
        let mut layer = Layer::new(&None, &None);
        layer.add_neurons(&1, &3, &0.0, &1.0);
        layer.calculate_outputs(&vec![1.0, 2.0]);
        assert_eq!(layer.values.len(), 1)
    }

    #[test]
    fn should_return_ten_outputs_for_ten_neurons() {
        let mut layer = Layer::new(&None, &None);
        layer.add_neurons(&10, &3, &0.0, &1.0);
        layer.calculate_outputs(&vec![1.0, 2.0]);
        assert_eq!(layer.values.len(), 10)
    }
}

mod calculate_weight_delta {
    use crate::neural_network::layer::Layer;

    #[test]
    fn should_return_positive_delta() {
        let expected_values = vec![100.0, 200.0];
        let inputs = vec![1.0, 2.0];
        let mut layer = Layer::new(&None, &None);
        layer.add_neurons(&2, &2, &0.0, &1.0);
        layer.calculate_outputs(&inputs);
        let expected_delta = vec![
            expected_values[0] - layer.values[0],
            expected_values[1] - layer.values[1],
        ];
        assert_eq!(
            layer.calculate_weight_delta(&expected_values),
            expected_delta
        )
    }

    #[test]
    fn should_return_negative_delta() {
        let expected_values = vec![1.0, 2.0];
        let inputs = vec![-1.0, -2.0];
        let mut layer = Layer::new(&None, &None);
        layer.add_neurons(&2, &2, &0.0, &1.0);
        layer.calculate_outputs(&inputs);
        let expected_delta = vec![
            expected_values[0] - layer.values[0],
            expected_values[1] - layer.values[1],
        ];
        assert_eq!(
            layer.calculate_weight_delta(&expected_values),
            expected_delta
        )
    }
}
