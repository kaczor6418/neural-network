use crate::neural_network::layer::Layer;
use crate::neural_network::network::network_config::NetworkConfig;

pub mod network_config;

pub struct Network {
    layers: Vec<Layer>,
    learning_rate: f64,
    loss_function: fn(expected: &f64, predicted: &f64) -> f64,
}

impl Network {
    pub fn new(config: NetworkConfig) -> Network {
        return Network {
            layers: Network::generate_layers(&config),
            learning_rate: config.learning_rate,
            loss_function: config
                .loose_function
                .unwrap_or(|expected: &f64, predicted: &f64| (predicted - expected).powi(2)),
        };
    }

    pub fn train(&mut self, inputs: Vec<Vec<f64>>, outputs: Vec<Vec<f64>>, iterations: usize) {
        let mut i = 0;
        while i < iterations {
            let data_size = outputs.len();
            let mut j = 0;
            while j < data_size {
                self.forward_propagation(inputs[j].clone());
                self.back_propagation(outputs[j].clone());
                j += 1;
            }
            i += 1;
        }
    }

    fn generate_layers(config: &NetworkConfig) -> Vec<Layer> {
        let mut inputs_count = config.inputs_count;
        return config
            .layers
            .iter()
            .map(|layer_cfg| {
                let mut layer = Layer::new(
                    &layer_cfg.activation_callback,
                    &layer_cfg.activation_function_derivative,
                );
                layer.add_neurons(
                    &layer_cfg.neurons_count,
                    &inputs_count,
                    &config.weights_range.min_weight,
                    &config.weights_range.max_weight,
                );
                inputs_count = layer.size();
                return layer;
            })
            .collect();
    }

    fn forward_propagation(&mut self, mut inputs: Vec<f64>) {
        self.layers
            .iter_mut()
            .for_each(|layer| inputs = layer.calculate_outputs(&inputs).clone());
    }

    fn back_propagation(&mut self, mut expected_output: Vec<f64>) {
        let learning_rate = &self.learning_rate; // ugly workaround to borrow checker complaining when writing these inline
        let reversed_layers_iter = self.layers.iter_mut().rev();
        reversed_layers_iter.for_each(|layer| {
            expected_output = layer.calculate_weight_delta(&expected_output);
            layer.correct_neurons_weight(&expected_output, learning_rate);
        })
    }
}

#[cfg(test)]
mod network_test;
