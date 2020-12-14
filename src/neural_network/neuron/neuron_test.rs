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
//
// mod update_input_value {
//     use crate::neural_network::neuron::Neuron;
//
//     #[test]
//     fn should_update_input_value_if_weight_exists() {
//         let weights = vec![1.0];
//         let mut neuron = Neuron::new(weights).unwrap();
//         let index = 0;
//         let value = 1.1;
//         neuron.update_input_value(index, value);
//         assert_eq!(neuron.inputs[index], value);
//     }
//
//     #[test]
//     #[should_panic]
//     fn should_not_update_input_value_if_weight_dose_not_exists() {
//         let weights = vec![1.0];
//         let neron_size = weights.len();
//         let mut neuron = Neuron::new(weights).unwrap();
//         neuron.update_input_value(neron_size, 2.1);
//     }
// }
//
// mod set_inputs_values {
//     use crate::neural_network::neuron::Neuron;
//
//     #[test]
//     fn should_set_inputs_values() {
//         let weights = vec![1.0, 2.0];
//         let inputs = vec![3.0, 4.0];
//         let mut neuron = Neuron::new(weights).unwrap();
//         neuron.set_inputs_values(inputs.clone());
//         assert_eq!(neuron.inputs, inputs);
//     }
// }
//
// mod set_weights_values {
//     use crate::neural_network::neuron::Neuron;
//
//     #[test]
//     fn should_set_inputs_values() {
//         let weights = vec![1.0, 2.0];
//         let new_weights = vec![3.0, 4.0];
//         let mut neuron = Neuron::new(weights).unwrap();
//         neuron.set_weights_values(new_weights.clone());
//         assert_eq!(neuron.weights, new_weights);
//     }
// }
//
// mod calculate_output_value {
//     use crate::neural_network::neuron::Neuron;
//
//     #[test]
//     fn should_calculate_value() {
//         let inputs = vec![1.0, 2.0];
//         let weights = vec![1.0, 2.0];
//         let expected_output_value = inputs[0] * weights[0] + inputs[1] * weights[1];
//         let mut neuron = Neuron::new(weights).unwrap();
//         neuron.set_inputs_values(inputs);
//         let output_value = neuron.calculate_output_value();
//         assert_eq!(output_value, expected_output_value);
//     }
//
//     #[test]
//     fn should_calculate_value_for_changed_weight() {
//         let inputs = vec![1.0, 2.0];
//         let weights = vec![1.0, 2.0];
//         let value = 2.0;
//         let expected_output_value = inputs[0] * value + inputs[1] * weights[1];
//         let mut neuron = Neuron::new(weights).unwrap();
//         neuron.set_inputs_values(inputs);
//         neuron.update_weight_value(0, value);
//         let output_value = neuron.calculate_output_value();
//         assert_eq!(output_value, expected_output_value);
//     }
//
//     #[test]
//     fn should_calculate_value_for_changed_input() {
//         let inputs = vec![1.0, 2.0];
//         let weights = vec![1.0, 2.0];
//         let value = 2.0;
//         let expected_output_value = inputs[0] * value + inputs[1] * weights[1];
//         let mut neuron = Neuron::new(weights).unwrap();
//         neuron.set_inputs_values(inputs);
//         neuron.update_input_value(0, value);
//         let output_value = neuron.calculate_output_value();
//         assert_eq!(output_value, expected_output_value);
//     }
// }
