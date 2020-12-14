mod new {
    use crate::neural_network::neuron::Neuron;

    #[test]
    fn should_create_neuron_if_min_weight_is_smaller_than_max_weight() {
        let weights_count: usize = 1;
        let min_weight = 0.0;
        let max_weight = 1.0;
        let neuron = Neuron::new(&weights_count, &min_weight, &max_weight);
        assert!(neuron.is_ok());
    }

    #[test]
    fn should_not_create_neuron_if_min_weight_is_bigger_than_max_weight() {
        let weights_count: usize = 1;
        let min_weight = 1.0;
        let max_weight = 0.0;
        let neuron = Neuron::new(&weights_count, &min_weight, &max_weight);
        assert!(neuron.is_err());
    }

    #[test]
    fn should_create_neuron_with_three_weights() {
        let weights_count: usize = 3;
        let min_weight = 0.0;
        let max_weight = 1.0;
        let neuron = Neuron::new(&weights_count, &min_weight, &max_weight).unwrap();
        assert_eq!(neuron.weights.len(), weights_count);
    }
}

mod update_weight_value {
    use crate::neural_network::neuron::Neuron;

    #[test]
    fn should_update_weight_value_if_weight_exists() {
        let mut neuron = Neuron::new(&1, &0.0, &1.0).unwrap();
        let index = 0;
        let value = 1.1;
        neuron.update_weight_value(index, value);
        assert_eq!(neuron.weights[index], value);
    }

    #[test]
    #[should_panic]
    fn should_not_update_weight_value_if_weight_dose_not_exists() {
        let mut neuron = Neuron::new(&1, &0.0, &1.0).unwrap();
        neuron.update_weight_value(1, 2.1);
    }
}

mod set_weights_values {
    use crate::neural_network::neuron::Neuron;

    #[test]
    fn should_set_inputs_values() {
        let new_weights = vec![3.0];
        let mut neuron = Neuron::new(&1, &0.0, &1.0).unwrap();
        neuron.set_weights_values(new_weights.clone());
        assert_eq!(neuron.weights, new_weights);
    }
}

mod calculate_output_value {
    use crate::neural_network::neuron::Neuron;

    #[test]
    fn should_calculate_value() {
        let inputs = vec![1.0, 2.0];
        let neuron = Neuron::new(&2, &0.0, &1.0).unwrap();
        let expected_output_value = inputs[0] * neuron.weights[0] + inputs[1] * neuron.weights[1];
        let output_value = neuron.calculate_output_value(&inputs);
        assert_eq!(output_value, expected_output_value);
    }

    #[test]
    fn should_calculate_value_for_changed_weight() {
        let inputs = vec![1.0, 2.0];
        let value = 2.0;
        let mut neuron = Neuron::new(&2, &0.0, &1.0).unwrap();
        neuron.update_weight_value(0, value);
        let expected_output_value = inputs[0] * neuron.weights[0] + inputs[1] * neuron.weights[1];
        let output_value = neuron.calculate_output_value(&inputs);
        assert_eq!(output_value, expected_output_value);
    }
}
