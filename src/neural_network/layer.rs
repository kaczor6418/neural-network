use crate::neural_network::neuron::Neuron;

pub struct Layer {
    neurons: Vec<Neuron>,
    activation_function: fn(value: f64) -> f64,
    activation_function_derivative: fn(value: f64) -> f64,
}

impl Layer {
    pub fn new(activation_callback: &Option<fn(value: f64) -> f64>) -> Layer {
        return Layer {
            neurons: vec![],
            activation_function: activation_callback.unwrap_or(|value: f64| value),
            activation_function_derivative: activation_callback.unwrap_or(|value: f64| 1.0),
        };
    }

    pub fn size(&self) -> usize {
        return self.neurons.len();
    }

    pub fn add_neurons(&mut self, neurons_count: &usize, inputs_count: &usize, min_weight: &f64, max_weight: &f64) {
        let mut i = 0;
        while &i < neurons_count {
            self.neurons.push(Neuron::new(inputs_count, min_weight, max_weight));
            i += 1;
        }
    }

    pub fn calculate_outputs(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        let activation_callback = &mut self.activation_function; // ugly workaround to borrow checker complaining when writing these inline
        return self.neurons.iter_mut().map(|neuron| (activation_callback)(neuron.calculate_output_value(&inputs))).collect();
    }
}

#[cfg(test)]
mod layer_test;

