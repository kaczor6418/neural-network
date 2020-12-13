use crate::neural_network::neuron::Neuron;

struct NeuralLayer {
    neurons: Vec<Neuron>
}

impl NeuralLayer {
    pub fn new(neurons: Option<Vec<Neuron>>) -> NeuralLayer {
        return NeuralLayer {
            neurons: neurons.unwrap_or(vec![])
        };
    }

    pub fn add_neuron(&mut self, neuron: Neuron) {
        self.neurons.push(neuron);
    }
}

#[cfg(test)]
mod neural_layer_test;

