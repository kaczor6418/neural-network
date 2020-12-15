use rand::Rng;

pub struct Neuron {
    weights: Vec<f64>,
    value: f64,
}

impl Neuron {
    pub fn new(weights_count: &usize, min_weight: &f64, max_weight: &f64) -> Neuron {
        return Neuron {
            weights: Neuron::generate_random_weights(&weights_count, &min_weight, &max_weight),
            value: 0.0,
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

    pub fn update_weight_value(&mut self, index: usize, value: f64) {
        self.weights[index] = value;
    }

    pub fn set_weights_values(&mut self, weights: Vec<f64>) {
        self.weights = weights;
    }

    pub fn calculate_output_value(&mut self, inputs: &Vec<f64>) -> f64 {
        let mut weights_iterator = self.weights.iter();
        self.value = inputs.iter().map(|input| input * weights_iterator.next().unwrap()).sum();
        return self.value;
    }
}

#[cfg(test)]
mod neuron_test;
