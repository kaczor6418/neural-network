use crate::matrix::matrix::Matrix;
use rand::Rng;

pub struct Neuron {
    weights: Matrix,
}

impl Neuron {
    pub fn new(weights_count: &usize, min_weight: &f64, max_weight: &f64) -> Neuron {
        return Neuron {
            weights: Matrix::new(
                1,
                Neuron::generate_random_weights(&weights_count, &min_weight, &max_weight),
            ),
        };
    }

    pub fn update_weight(&mut self, index: usize, learning_rate: &f64, delta_value: &f64) {
        self.set_weight(
            index,
            self.calculate_new_weight(index, learning_rate, delta_value),
        );
    }

    pub fn calculate_output_value(&self, inputs: &Matrix) -> f64 {
        return (inputs * &self.weights)[0][0];
    }

    pub fn get_weights(&self) -> &Matrix {
        return &self.weights;
    }

    pub fn get_mutable_weights(&mut self) -> &mut Matrix {
        return &mut self.weights;
    }

    fn generate_random_weights(
        weights_count: &usize,
        min_weight: &f64,
        max_weight: &f64,
    ) -> Vec<f64> {
        let mut i = 0;
        let mut weights: Vec<f64> = vec![];
        while &i < weights_count {
            weights.push(rand::thread_rng().gen_range(min_weight, max_weight));
            i += 1;
        }
        return weights;
    }

    fn set_weight(&mut self, index: usize, value: f64) {
        self.weights[index][0] = value;
    }

    fn calculate_new_weight(&self, index: usize, learning_rate: &f64, delta_value: &f64) -> f64 {
        return self.weights[index][0] - learning_rate * delta_value;
    }
}

#[cfg(test)]
mod neuron_test;
