mod new {
    use crate::neural_network::neural_layer::NeuralLayer;
    use crate::neural_network::neuron::Neuron;

    #[test]
    fn should_create_empty_neural_layer() {
        let empty_layer = NeuralLayer::new(None);
        assert_eq!(empty_layer.neurons.len(), 0);
    }

    #[test]
    fn should_create_neural_layer_with_one_neuron() {
        let weights = vec![0.0, 1.0, 2.0];
        let neurons = vec![Neuron::new(weights).unwrap()];
        let layer = NeuralLayer::new(Some(neurons));
        assert_eq!(layer.neurons.len(), 1);
    }

    #[test]
    fn should_create_neural_layer_with_two_neuron() {
        let weights1 = vec![0.0, 1.0, 2.0];
        let weights2 = vec![0.1, 1.2, 2.3];
        let neurons = vec![Neuron::new(weights1).unwrap(), Neuron::new(weights2).unwrap()];
        let layer = NeuralLayer::new(Some(neurons));
        assert_eq!(layer.neurons.len(), 2);
    }
}

mod add_neuron {
    use crate::neural_network::neural_layer::NeuralLayer;
    use crate::neural_network::neuron::Neuron;

    #[test]
    fn should_add_neuron_to_empty_layer() {
        let weights = vec![0.0, 1.0, 2.0];
        let mut empty_layer = NeuralLayer::new(None);
        empty_layer.add_neuron(Neuron::new(weights).unwrap());
        assert_eq!(empty_layer.neurons.len(), 1);
    }
    #[test]
    fn should_add_neuron_to_layer_with_neurons() {
        let weights1 = vec![0.0, 1.0, 2.0];
        let weights2 = vec![0.1, 1.2, 2.3];
        let neurons = vec![Neuron::new(weights1).unwrap()];
        let mut layer = NeuralLayer::new(Some(neurons));
        layer.add_neuron(Neuron::new(weights2).unwrap());
        assert_eq!(layer.neurons.len(), 2);
    }
}
