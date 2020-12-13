use crate::neural_network::neuron::Neuron;

pub struct Layer {
    neurons: Vec<Neuron>,
    activation_callback: fn(value: f64) -> f64,
}

impl Layer {
    pub fn new(neurons: Option<Vec<Neuron>>, activation_callback: Option<fn(value: f64) -> f64>) -> Layer {
        return Layer {
            neurons: neurons.unwrap_or(vec![]),
            activation_callback: activation_callback.unwrap_or(|value: f64| value),
        };
    }

    pub fn add_neuron(&mut self, neuron: Neuron) {
        self.neurons.push(neuron);
    }

    pub fn get_outputs(&self) -> Vec<f64> {
        return self.neurons.iter().map(|neuron| (self.activation_callback)(neuron.calculate_output_value())).collect();
    }
}

#[cfg(test)]
mod layer_test;

