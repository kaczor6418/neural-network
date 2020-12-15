use crate::neural_network::layer::Layer;
use crate::neural_network::network::network_config::NetworkConfig;

pub mod network_config;

pub struct Network {
    expected_output: Vec<f64>,
    inputs: Vec<f64>,
    layers: Vec<Layer>,
    learning_rate: f64,
    loss_function: fn(expected: f64, predicted: f64) -> f64,
}

impl Network {
    pub fn new(config: NetworkConfig) -> Network {
        return Network {
            layers: Network::generate_layers(&config),
            expected_output: config.expected_output,
            inputs: config.inputs,
            learning_rate: config.learning_rate,
            loss_function: config.loose_function.unwrap_or(|expected: f64, predicted: f64| (predicted - expected).powi(2)),
        };
    }

    fn generate_layers(config: &NetworkConfig) -> Vec<Layer> {
        let mut inputs_count = config.inputs.len();
        return config.layers.iter().map(|layer_cfg| {
            let mut layer = Layer::new(&layer_cfg.activation_callback);
            layer.add_neurons(&layer_cfg.neurons_count, &inputs_count, &config.weights_range.min_weight, &config.weights_range.max_weight);
            inputs_count = layer.size();
            return layer;
        }).collect();
    }

    fn forward_propagation(&self) -> Vec<f64> {
        return self.layers.iter().fold(self.inputs.clone(), |output, layer| layer.calculate_outputs(output));
    }
}


#[cfg(test)]
mod network_test;
