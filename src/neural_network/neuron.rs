use rand::Rng;

pub struct Neuron {
    weights: Vec<f64>
}

impl Neuron {
    pub fn new(weights_count: &usize, min_weight: &f64, max_weight: &f64) -> Result<Neuron, String> {
        if Neuron::is_min_weight_less_then_max_weight(&min_weight, &max_weight) == false {
            return Err(format!("Min weight has to be smaller then max weight.\n Min weight: {}, Max weight: {}", min_weight, max_weight));
        }
        let weights = Neuron::generate_random_weights(&weights_count, &min_weight, &max_weight);
        return Ok(Neuron {
            weights
        });
    }

    fn is_min_weight_less_then_max_weight(min_weight: &f64, max_weight: &f64) -> bool {
        return min_weight < max_weight;
    }

    fn generate_random_weights(weights_count: &usize, min_weight: &f64, max_weight: &f64) -> Vec<f64> {
        let mut i = 0;
        let mut weights: Vec<f64> =  vec![];
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

    pub fn calculate_output_value(&self, inputs: &Vec<f64>) -> f64 {
        let mut weights_iterator = self.weights.iter();
        return inputs.iter().map(|input| input * weights_iterator.next().unwrap()).sum();
    }
}

#[cfg(test)]
mod neuron_test;
