mod new {
    use crate::neural_network::neural_layer::NeuralLayer;
    use crate::neural_network::neuron::Neuron;

    #[test]
    fn should_create_empty_neural_layer() {
        let empty_layer = NeuralLayer::new();
        assert_eq!(empty_layer.neurons.len(), 0);
    }

    #[test]
    fn should_create_neural_layer_with_one_neuron() {
        let weights = vec![0.0, 1.0, 2.0];
        let neurons = vec![Neuron::new(weights)];
        let layer = NeuralLayer::new(Option(neurons));
        assert_eq!(layer.neurons.len(), 1);
    }

    #[test]
    fn should_create_neural_layer_with_two_neuron() {
        let weights1 = vec![0.0, 1.0, 2.0];
        let weights2 = vec![0.1, 1.2, 2.3];
        let neurons = vec![Neuron::new(weights1), Neuron::new(weights2)];
        let layer = NeuralLayer::new(Option(neurons));
        assert_eq!(layer.neurons.len(), 2);
    }
}

mod add_neuron {
    use crate::neural_network::neural_layer::NeuralLayer;
    use crate::neural_network::neuron::Neuron;

    #[test]
    fn should_add_neuron_to_empty_layer() {
        let weights = vec![0.0, 1.0, 2.0];
        let empty_layer = NeuralLayer::new();
        empty_layer.add_neuron(Neuron::new(weights));
        assert_eq!(layer.neurons.len(), 1);
    }
    #[test]
    fn should_add_neuron_to_layer_with_neurons() {
        let weights1 = vec![0.0, 1.0, 2.0];
        let weights2 = vec![0.1, 1.2, 2.3];
        let neurons = vec![Neuron::new(weights1)];
        let layer = NeuralLayer::new(Option(neurons));
        layer.add_neuron(Neuron::new(weights2));
        assert_eq!(layer.neurons.len(), 2);
    }
}
