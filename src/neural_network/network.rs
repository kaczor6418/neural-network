// use crate::matrix::matrix::Matrix;
// use crate::neural_network::layer::Layer;
// use crate::neural_network::network::network_config::NetworkConfig;
//
// pub mod network_config;
//
// pub struct Network {
//     hidden_layers: Vec<Layer>,
//     learning_rate: f64,
//     loss_function: fn(expected: &f64, predicted: &f64) -> f64,
//     outputs: Matrix,
//     output_layer: Layer,
// }
//
// impl Network {
//     pub fn new(config: NetworkConfig) -> Network {
//         return Network {
//             outputs: Matrix::new_zeros_matrix(0, 0),
//             output_layer: Layer::new(
//                 config.output_layer,
//                 &config.hidden_layers.last().unwrap().neurons_count,
//             ),
//             hidden_layers: Network::generate_layers(&config),
//             learning_rate: config.learning_rate,
//             loss_function: config
//                 .loose_function
//                 .unwrap_or(|expected: &f64, predicted: &f64| (predicted - expected).powi(2)),
//         };
//     }
//
//     pub fn train(&mut self, inputs: Vec<Vec<f64>>, outputs: Vec<Vec<f64>>, iterations: usize) {
//         self.outputs = Matrix::new_zeros_matrix(outputs.len(), outputs[0].len());
//         let mut t_outputs: Vec<Vec<f64>> = vec![];
//         let mut i = 0;
//         while i < iterations {
//             t_outputs = vec![];
//             let data_size = outputs.len();
//             let mut j = 0;
//             while j < data_size {
//                 self.forward_propagation(&inputs[j]);
//                 self.back_propagation(&outputs[j]);
//                 // self.outputs[j] = self.output_layer.get_values().get_values()[..];
//                 t_outputs.push(self.output_layer.get_values().get_values().to_vec());
//                 j += 1;
//             }
//             i += 1;
//         }
//         println!("{:?}", outputs);
//     }
//
//     // pub fn fit(&mut self, inputs: Vec<Vec<f64>>) {}
//
//     fn back_propagation(&mut self, expected_output: &Vec<f64>) {
//         let mut next_layer_delta = self.output_layer.get_values()
//             - &Matrix::new(expected_output.len(), expected_output.clone());
//         self.output_layer.correct_neurons_weight(
//             &self.learning_rate,
//             self.hidden_layers.last().unwrap().get_values(),
//             &next_layer_delta,
//             &self.output_layer.get_neurons_weights(),
//         );
//         let mut next_layer_weights = self.output_layer.get_neurons_weights();
//         let mut index = self.hidden_layers.len() - 1;
//         while index >= 0 {
//             self.hidden_layers[index].correct_neurons_weight(
//                 &self.learning_rate,
//                 self.hidden_layers[index - 1].get_values(),
//                 &next_layer_delta,
//                 &next_layer_weights,
//             );
//             next_layer_delta = self.hidden_layers[index]
//                 .calculate_layer_delta(&next_layer_delta, &next_layer_weights);
//             next_layer_weights = self.hidden_layers[index].get_neurons_weights();
//             index -= 1;
//         }
//         // let learning_rate = &self.learning_rate; // ugly workaround to borrow checker complaining when writing these inline
//         // let reversed_layers_iter = self.hidden_layers.iter_mut().rev();
//         // reversed_layers_iter.for_each(|layer| {
//         //     next_layer_delta = layer.calculate_layer_delta(&output);
//         //     layer.correct_neurons_weight(&output, learning_rate);
//         // })
//     }
//
//     fn forward_propagation(&mut self, inputs: &Vec<f64>) {
//         let mut values = Matrix::new(inputs.len(), inputs.clone());
//         self.hidden_layers.iter_mut().for_each(|layer| {
//             layer.calculate_values(&values);
//             values = layer.get_values().clone();
//         });
//         self.output_layer.calculate_values(&values);
//     }
//
//     fn generate_layers(config: &NetworkConfig) -> Vec<Layer> {
//         let mut inputs_count = config.inputs_count;
//         return config
//             .hidden_layers
//             .into_iter()
//             .map(|layer_cfg| {
//                 let layer = Layer::new(layer_cfg, &inputs_count);
//                 inputs_count = layer.size();
//                 return layer;
//             })
//             .collect();
//     }
// }
//
// #[cfg(test)]
// mod network_test;
