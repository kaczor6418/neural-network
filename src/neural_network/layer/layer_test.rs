mod new {
    use crate::neural_network::layer::Layer;
    use crate::neural_network::neuron::Neuron;

    #[test]
    fn should_create_empty_neural_layer() {
        let empty_layer = Layer::new(None);
        assert_eq!(empty_layer.neurons.len(), 0);
    }
}
//
// mod add_neuron {
//     use crate::neural_network::layer::Layer;
//     use crate::neural_network::neuron::Neuron;
//
//     #[test]
//     fn should_add_neuron_to_empty_layer() {
//         let weights = vec![0.0, 1.0, 2.0];
//         let mut empty_layer = Layer::new(None, None);
//         empty_layer.add_neuron(Neuron::new(weights).unwrap());
//         assert_eq!(empty_layer.neurons.len(), 1);
//     }
//
//     #[test]
//     fn should_add_neuron_to_layer_with_neurons() {
//         let weights1 = vec![0.0, 1.0, 2.0];
//         let weights2 = vec![0.1, 1.2, 2.3];
//         let neurons = vec![Neuron::new(weights1).unwrap()];
//         let mut layer = Layer::new(Some(neurons), None);
//         layer.add_neuron(Neuron::new(weights2).unwrap());
//         assert_eq!(layer.neurons.len(), 2);
//     }
// }
//
// mod get_outputs {
//     use crate::neural_network::layer::Layer;
//     use crate::neural_network::neuron::Neuron;
//
//     #[test]
//     fn should_return_one_outputs_for_one_neurons() {
//         let weights = vec![0.0, 1.0, 2.0];
//         let neuron = Neuron::new(weights).unwrap();
//         let expected_output =  vec![neuron.calculate_output_value()];
//         let layer = Layer::new(Some(vec![neuron]), None);
//         assert_eq!(layer.get_outputs(), expected_output);
//     }
//
//     #[test]
//     fn should_return_three_outputs_for_three_neurons() {
//         let weights = vec![vec![0.0, 1.0, 2.0], vec![3.0, 4.0, 5.0], vec![6.0, 7.0, 8.0]];
//         let neurons: Vec<Neuron> = weights.iter().map(|weight| Neuron::new(weight.clone()).unwrap()).collect();
//         let expected_output: Vec<f64> =  neurons.iter().map(|neuron| neuron.calculate_output_value()).collect();
//         let layer = Layer::new(Some(neurons), None);
//         assert_eq!(layer.get_outputs(), expected_output);
//     }
// }
