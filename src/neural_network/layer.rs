use crate::neural_network::neuron::Neuron;

pub struct Layer {
    neurons: Vec<Neuron>,
    activation_callback: fn(value: f64) -> f64,
}

impl Layer {
    pub fn new(activation_callback: Option<fn(value: f64) -> f64>) -> Layer {
        return Layer {
            neurons: vec![],
            activation_callback: activation_callback.unwrap_or(|value: f64| value),
        };
    }

    pub fn add_neurons(&mut self, neurons_count: &usize, inputs_count: &usize, min_weight: &f64, max_weight: &f64) {
        let mut i = 0;
        while &i < neurons_count {
            self.neurons.push(Neuron::new(inputs_count, min_weight, max_weight).unwrap());
            i += 1;
        }
    }

    pub fn get_outputs(&self, inputs: Vec<f64>) -> Vec<f64> {
        return self.neurons.iter().map(|neuron| (self.activation_callback)(neuron.calculate_output_value(&inputs))).collect();
    }
}

#[cfg(test)]
mod layer_test;

