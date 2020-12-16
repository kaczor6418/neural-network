use crate::neural_network::neuron::Neuron;

pub struct Layer {
    activation_function: fn(value: f64) -> f64,
    activation_function_derivative: fn(value: &f64) -> f64,
    neurons: Vec<Neuron>,
    values: Vec<f64>,
}

impl Layer {
    pub fn new(
        activation_function: &Option<fn(value: f64) -> f64>,
        activation_function_derivative: &Option<fn(value: &f64) -> f64>,
    ) -> Layer {
        return Layer {
            activation_function: activation_function.unwrap_or(|value: f64| value),
            activation_function_derivative: activation_function_derivative
                .unwrap_or(|_value: &f64| 1.0),
            neurons: vec![],
            values: vec![],
        };
    }

    pub fn size(&self) -> usize {
        return self.neurons.len();
    }

    pub fn add_neurons(
        &mut self,
        neurons_count: &usize,
        inputs_count: &usize,
        min_weight: &f64,
        max_weight: &f64,
    ) {
        let mut i = 0;
        while &i < neurons_count {
            self.neurons
                .push(Neuron::new(inputs_count, min_weight, max_weight));
            i += 1;
        }
    }

    pub fn calculate_outputs(&mut self, inputs: &Vec<f64>) -> &Vec<f64> {
        self.values = self
            .neurons
            .iter()
            .map(|neuron| (self.activation_function)(neuron.calculate_output_value(inputs)))
            .collect();
        return &self.values;
    }

    pub fn calculate_weight_delta(&self, expected_values: &Vec<f64>) -> Vec<f64> {
        let mut expected_values_iter = expected_values.iter();
        return self
            .values
            .iter()
            .map(|value| {
                (expected_values_iter.next().unwrap() - value)
                    * (self.activation_function_derivative)(value)
            })
            .collect();
    }

    pub fn correct_neurons_weight(&mut self, loss_values: &Vec<f64>, learning_rate: &f64) {
        let mut loss_values_iter = loss_values.iter();
        for (index, neuron) in self.neurons.iter_mut().enumerate() {
            neuron.update_weight(index, learning_rate, loss_values_iter.next().unwrap());
        }
    }
}

#[cfg(test)]
mod layer_test;
