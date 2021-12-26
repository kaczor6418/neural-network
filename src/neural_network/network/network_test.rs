// fn valid_config_with_one_layer() -> NetworkConfig {
//     return NetworkConfig {
//         inputs_count: 3,
//         hidden_layers: vec![LayerConfig {
//             neurons_count: 2,
//             activation_function: None,
//             activation_function_derivative: None,

//         }],
//         learning_rate: 0.01,
//         loose_function: None,
//         weights_range: WeightsRange {
//             min_weight: 0.0,

//             max_weight: 1.0,
//         },
//         output_layer: LayerConfig {
//             neurons_count: 2,

//             activation_function: None,
//             activation_function_derivative: None,
//         },
//     };
// }
//
// fn valid_config_with_three_layers() -> NetworkConfig {
//     return NetworkConfig {
//         inputs_count: 3,
//         hidden_layers: vec![
//             LayerConfig {
//                 neurons_count: 4,
//                 activation_function: None,
//                 activation_function_derivative: None,
//             },
//             LayerConfig {
//                 neurons_count: 3,
//                 activation_function: None,
//                 activation_function_derivative: None,
//             },
//             LayerConfig {
//                 neurons_count: 5,
//                 activation_function: None,
//                 activation_function_derivative: None,
//             },
//         ],
//         learning_rate: 0.01,
//         loose_function: None,
//         weights_range: WeightsRange {
//             min_weight: 0.0,
//             max_weight: 1.0,
//         },
//         output_layer: LayerConfig {
//             neurons_count: 2,
//             activation_function: None,
//             activation_function_derivative: None,
//         },
//     };
// }
//
// mod new {
//     use crate::neural_network::network::network_test::{
//         valid_config_with_one_layer, valid_config_with_three_layers,
//     };
//     use crate::neural_network::network::Network;
//
//     #[test]
//     fn should_create_network_with_one_hidden_layer() {
//         let network = Network::new(valid_config_with_one_layer());
//         assert_eq!(network.hidden_layers.len(), 1)
//     }
//
//     #[test]
//     fn should_create_network_with_three_hidden_layers() {
//         let network = Network::new(valid_config_with_three_layers());
//         assert_eq!(network.hidden_layers.len(), 3)
//     }
// }
