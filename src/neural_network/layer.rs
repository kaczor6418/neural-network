use crate::matrix::matrix::Matrix;
use crate::neural_network::layer::layer_config::{LayerConfig, WeightsRange};
use crate::neural_network::neuron::Neuron;

pub mod layer_config;

pub struct Layer {
    activation_function: fn(value: &f64) -> f64,
    activation_function_derivative: fn(value: &f64) -> f64,
    neurons: Vec<Neuron>,
    values: Matrix,
}

impl Layer {
    pub fn new(config: LayerConfig, inputs_count: usize) -> Layer {
        let mut layer = Layer {
            activation_function: config
                .activation_function
                .unwrap_or(|value: &f64| value.clone()),
            activation_function_derivative: config
                .activation_function_derivative
                .unwrap_or(|_value: &f64| 1.0),
            neurons: vec![],
            values: Matrix::new_zeros_matrix(1, config.neurons_count),
        };
        layer.add_neurons(config.neurons_count, inputs_count, config.weights_range);
        return layer;
    }

    pub fn calculate_values(&mut self, inputs: &Matrix) {
        self.values.set_values(
            self.neurons
                .iter()
                .map(|neuron| (self.activation_function)(&neuron.calculate_output_value(&inputs)))
                .collect(),
        );
    }

    pub fn correct_neurons_weight(
        &mut self,
        learning_rate: &f64,
        prev_layer_values: &Matrix,
        next_layer_delta: &Matrix,
        next_layer_weights: &Matrix,
    ) {
        let weights_delta = learning_rate.clone()
            * &self.calculate_wights_delta(prev_layer_values, next_layer_delta, next_layer_weights);
        let new_weights = &self.get_neurons_weights() - &weights_delta;
        for row_index in 0..new_weights.rows_count() {
            self.neurons[row_index]
                .get_mutable_weights()
                .set_values(new_weights[row_index].to_vec())
        }
    }

    pub fn get_values(&self) -> &Matrix {
        return &self.values;
    }

    pub fn size(&self) -> usize {
        return self.neurons.len();
    }

    fn add_neurons(
        &mut self,
        neurons_count: usize,
        inputs_count: usize,
        weights_range: WeightsRange,
    ) {
        let mut i = 0;
        while i < neurons_count {
            self.neurons.push(Neuron::new(
                &inputs_count,
                &weights_range.min_weight,
                &weights_range.max_weight,
            ));
            i += 1;
        }
    }

    fn calculate_derivative_values(&self) -> Matrix {
        return Matrix::new(
            self.values.columns_count(),
            self.values
                .get_values()
                .iter()
                .map(|value| (self.activation_function_derivative)(value))
                .collect(),
        );
    }

    fn calculate_layer_delta(
        &self,
        next_layer_delta: &Matrix,
        next_layer_weights: &Matrix,
    ) -> Matrix {
        return next_layer_delta
            * next_layer_weights
            * self.calculate_derivative_values().get_values();
    }

    fn calculate_wights_delta(
        &self,
        prev_layer_values: &Matrix,
        next_layer_delta: &Matrix,
        next_layer_weights: &Matrix,
    ) -> Matrix {
        return self
            .calculate_layer_delta(next_layer_delta, next_layer_weights)
            .transpose()
            .kronecker_product(prev_layer_values);
    }

    fn get_neurons_weights(&self) -> Matrix {
        let mut values: Vec<f64> = vec![];
        self.neurons
            .iter()
            .for_each(|neuron| values.extend(neuron.get_weights().get_values()));
        return Matrix::new(self.neurons[0].get_weights().get_values().len(), values);
    }
}

#[cfg(test)]
mod layer_test;
