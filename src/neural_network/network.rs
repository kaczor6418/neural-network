use crate::neural_network::layer::Layer;
use crate::neural_network::network::network_config::{LayerConfig, NetworkConfig, WeightsRange};
use serde::private::ser::constrain;

pub mod network_config;

pub struct Network {
    hidden_layers: Vec<Layer>,
    learning_rate: f64,
    loss_function: fn(expected: &f64, predicted: &f64) -> f64,
    outputs: Vec<Vec<f64>>,
    output_layer: Layer,
}

impl Network {
    pub fn new(config: NetworkConfig) -> Network {
        return Network {
            hidden_layers: Network::generate_layers(&config),
            learning_rate: config.learning_rate,
            loss_function: config
                .loose_function
                .unwrap_or(|expected: &f64, predicted: &f64| (predicted - expected).powi(2)),
            outputs: vec![],
            output_layer: Network::generate_layer(
                &config.output_layer,
                &config.hidden_layers.last().unwrap().neurons_count,
                &config.weights_range,
            ),
        };
    }

    pub fn train(&mut self, inputs: Vec<Vec<f64>>, outputs: Vec<Vec<f64>>, iterations: usize) {
        self.outputs = vec![vec![]; outputs.len()];
        let mut i = 0;
        while i < iterations {
            let data_size = outputs.len();
            let mut j = 0;
            while j < data_size {
                self.forward_propagation(inputs[j].clone());
                self.back_propagation(outputs[j].clone());
                self.outputs[j] = self.get_output();
                j += 1;
            }
            i += 1;
        }
    }

    // pub fn fit(&mut self, inputs: Vec<Vec<f64>>, outputs: Vec<Vec<f64>>) {}

    fn back_propagation(&mut self, mut expected_output: Vec<f64>) {
        let learning_rate = &self.learning_rate; // ugly workaround to borrow checker complaining when writing these inline
        let reversed_layers_iter = self.hidden_layers.iter_mut().rev();
        reversed_layers_iter.for_each(|layer| {
            expected_output = layer.calculate_layer_delta(&expected_output);
            layer.correct_neurons_weight(&expected_output, learning_rate);
        })
    }

    fn forward_propagation(&mut self, mut inputs: Vec<f64>) {
        self.hidden_layers.iter_mut().for_each(|layer| {
            layer.calculate_output(&inputs);
            inputs = layer.get_output().clone();
        });
    }

    fn generate_layer(
        layer_cfg: &LayerConfig,
        inputs_count: &usize,
        weights_range: &WeightsRange,
    ) -> Layer {
        let mut layer = Layer::new(
            &layer_cfg.activation_callback,
            &layer_cfg.activation_function_derivative,
        );
        layer.add_neurons(
            &layer_cfg.neurons_count,
            inputs_count,
            &weights_range.min_weight,
            &weights_range.max_weight,
        );
        return layer;
    }

    fn generate_layers(config: &NetworkConfig) -> Vec<Layer> {
        let mut inputs_count = config.inputs_count;
        return config
            .hidden_layers
            .iter()
            .map(|layer_cfg| {
                let layer =
                    Network::generate_layer(layer_cfg, &inputs_count, &config.weights_range);
                inputs_count = layer.size();
                return layer;
            })
            .collect();
    }

    fn get_output(&self) -> Vec<f64> {
        return self.output_layer.get_output().clone();
    }
}

#[cfg(test)]
mod network_test;
