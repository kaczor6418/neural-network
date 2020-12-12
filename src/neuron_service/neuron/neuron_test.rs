mod new {
    use crate::neuron_service::neuron::Neuron;

    #[test]
    fn should_create_neuron_if_inputs_and_weights_have_same_size() {
        let inputs = vec![1.0];
        let weights = vec![2.0];
        let neuron = Neuron::new(inputs, weights);
        assert!(neuron.is_ok());
    }

    #[test]
    fn should_not_create_neuron_if_inputs_and_weights_have_different_size() {
        let inputs = vec![1.0];
        let weights = vec![1.0, 2.0];
        let neuron = Neuron::new(inputs, weights);
        assert!(neuron.is_err());
    }
}

mod change_weight_value {
    use crate::neuron_service::neuron::Neuron;

    #[test]
    fn should_change_weight_value_if_weight_exists() {
        let inputs = vec![1.0];
        let weights = vec![2.0];
        let mut neuron = Neuron::new(inputs, weights).unwrap();
        let index = 0;
        let value = 1.1;
        neuron.change_weight_value(index, value);
        assert_eq!(neuron.weights[index], value);
    }

    #[test]
    #[should_panic]
    fn should_not_change_weight_value_if_weight_dose_not_exists() {
        let inputs = vec![1.0];
        let weights = vec![1.0, 2.0];
        let weights_size = weights.len();
        let mut neuron = Neuron::new(inputs, weights).unwrap();
        neuron.change_weight_value(weights_size, 2.1);
    }
}

mod calculate_output_value {
    use crate::neuron_service::neuron::Neuron;

    #[test]
    fn should_calculate_value() {
        let inputs = vec![1.0, 2.0];
        let weights = vec![1.0, 2.0];
        let expected_output_value = inputs[0] * weights[0] + inputs[1] * weights[1];
        let neuron = Neuron::new(inputs, weights).unwrap();
        let output_value = neuron.calculate_output_value();
        assert_eq!(output_value, expected_output_value);
    }

    #[test]
    fn should_calculate_value_for_changed_value() {
        let inputs = vec![1.0, 2.0];
        let weights = vec![1.0, 2.0];
        let value = 2.0;
        let expected_output_value = inputs[0] * value + inputs[1] * weights[1];
        let mut neuron = Neuron::new(inputs, weights).unwrap();
        neuron.change_weight_value(0, value);
        let output_value = neuron.calculate_output_value();
        assert_eq!(output_value, expected_output_value);
    }
}
