use rand::Rng;

pub struct Neuron {
    weights: Vec<f64>,
}

impl Neuron {
    pub fn new(weights_count: &usize, min_weight: &f64, max_weight: &f64) -> Neuron {
        return Neuron {
            weights: Neuron::generate_random_weights(&weights_count, &min_weight, &max_weight)
        };
    }

    fn generate_random_weights(weights_count: &usize, min_weight: &f64, max_weight: &f64) -> Vec<f64> {
        let mut i = 0;
        let mut weights: Vec<f64> = vec![];
        while &i < weights_count {
            weights.push(rand::thread_rng().gen_range(min_weight, max_weight));
            i += 1;
        }
        return weights;
    }

    pub fn update_weight(&mut self, index: usize, learning_rate: &f64, loss_value: &f64) {
        self.set_weight(index, self.calculate_new_weight(index, learning_rate, loss_value));
    }

    pub fn calculate_output_value(&self, inputs: &Vec<f64>) -> f64 {
        let mut weights_iterator = self.weights.iter();
        return inputs.iter().map(|input| input * weights_iterator.next().unwrap()).sum();
    }

    fn set_weight(&mut self, index: usize, value: f64) {
        self.weights[index] = value;
    }

    fn calculate_new_weight(&self, index: usize, learning_rate: &f64, loss_value: &f64) -> f64 {
        return self.weights[index] - learning_rate * loss_value;
    }
}

#[cfg(test)]
mod neuron_test;
