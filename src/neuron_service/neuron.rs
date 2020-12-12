pub struct Neuron {
    inputs: Vec<f64>,
    weights: Vec<f64>,
}

impl Neuron {
    pub fn new(inputs: Vec<f64>, weights: Vec<f64>) -> Result<Neuron, String> {
        if inputs.len() != weights.len() {
            return Err(format!("Inputs and weights has to be the same size.\n Inputs size: {}, Weights size: {}", inputs.len(), weights.len()));
        }
        return Ok(Neuron {
            weights,
            inputs,
        });
    }

    pub fn change_weight_value(&mut self, index: usize, value: f64) {
        self.weights[index] = value;
    }

    pub fn calculate_output_value(&self) -> f64 {
        let mut output = 0.0;
        let mut weights_iterator = self.weights.iter();
        self.inputs.iter().for_each(|input| {
            output += input * weights_iterator.next().unwrap()
        });
        return output;
    }

}

#[cfg(test)]
mod neuron_test;
