mod new {
    use crate::neural_network::neuron::Neuron;

    #[test]
    fn should_create_neuron_with_three_weights() {
        let weights_count: usize = 3;
        let min_weight = 0.0;
        let max_weight = 1.0;
        let neuron = Neuron::new(&weights_count, &min_weight, &max_weight);
        assert_eq!(neuron.weights.len(), weights_count);
    }
}

mod set_weight {
    use crate::neural_network::neuron::Neuron;

    #[test]
    fn should_update_weight_value_if_weight_exists() {
        let mut neuron = Neuron::new(&1, &0.0, &1.0);
        let index = 0;
        let value = 1.1;
        neuron.set_weight(index, value);
        assert_eq!(neuron.weights[index], value);
    }

    #[test]
    #[should_panic]
    fn should_not_update_weight_value_if_weight_dose_not_exists() {
        let mut neuron = Neuron::new(&1, &0.0, &1.0);
        neuron.set_weight(1, 2.1);
    }
}

mod calculate_output_value {
    use crate::neural_network::neuron::Neuron;

    #[test]
    fn should_calculate_value() {
        let inputs = vec![1.0, 2.0];
        let mut neuron = Neuron::new(&2, &0.0, &1.0);
        let expected_output_value = inputs[0] * neuron.weights[0] + inputs[1] * neuron.weights[1];
        let output_value = neuron.calculate_output_value(&inputs);
        assert_eq!(output_value, expected_output_value);
    }

    #[test]
    fn should_calculate_value_for_changed_weight() {
        let inputs = vec![1.0, 2.0];
        let value = 2.0;
        let mut neuron = Neuron::new(&2, &0.0, &1.0);
        neuron.set_weight(0, value);
        let expected_output_value = inputs[0] * neuron.weights[0] + inputs[1] * neuron.weights[1];
        let output_value = neuron.calculate_output_value(&inputs);
        assert_eq!(output_value, expected_output_value);
    }
}

mod calculate_new_weight {
    use crate::neural_network::neuron::Neuron;

    #[test]
    fn should_calculate_new_weight_if_weight_exists() {
        let mut neuron = Neuron::new(&2, &0.0, &1.0);
        let index = 0;
        let learning_rate = 0.01;
        let loss_value = 12.0;
        let expected_new_value = neuron.weights[index] - learning_rate * loss_value;
        assert_eq!(neuron.calculate_new_weight(index, &learning_rate, &loss_value), expected_new_value);
    }

    #[test]
    #[should_panic]
    fn should_not_calculate_new_weight_if_weight_does_not_exists() {
        let mut neuron = Neuron::new(&2, &0.0, &1.0);
        let index = 2;
        let learning_rate = 0.01;
        let loss_value = 12.0;
        neuron.calculate_new_weight(index, &learning_rate, &loss_value);
    }
}

mod update_weight {
    use crate::neural_network::neuron::Neuron;

    #[test]
    fn should_update_weight_if_weight_exists() {
        let mut neuron = Neuron::new(&2, &0.0, &1.0);
        let index = 0;
        let learning_rate = 0.01;
        let loss_value = 12.0;
        let expected_new_value = neuron.weights[index] - learning_rate * loss_value;
        neuron.update_weight(index, &learning_rate, &loss_value);
        assert_eq!(neuron.weights[index], expected_new_value);
    }

    #[test]
    #[should_panic]
    fn should_not_update_weight_if_weight_does_not_exists() {
        let mut neuron = Neuron::new(&2, &0.0, &1.0);
        let index = 2;
        let learning_rate = 0.01;
        let loss_value = 12.0;
        neuron.update_weight(index, &learning_rate, &loss_value);
    }
}
