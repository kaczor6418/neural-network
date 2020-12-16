use crate::neural_network::network::network_config::{LayerConfig, NetworkConfig, WeightsRange};

fn valid_config_with_one_layer() -> NetworkConfig {
    return NetworkConfig {
        expected_output: vec![1.0, 2.0],
        inputs: vec![1.0, 2.0, 3.0],
        layers: vec![LayerConfig {
            neurons_count: 2,
            activation_callback: None,
            activation_function_derivative: None,
        }],
        learning_rate: 0.01,
        loose_function: None,
        weights_range: WeightsRange {
            min_weight: 0.0,
            max_weight: 1.0,
        },
    };
}

fn valid_config_with_three_layers() -> NetworkConfig {
    return NetworkConfig {
        expected_output: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        inputs: vec![1.0, 2.0, 3.0],
        layers: vec![
            LayerConfig {
                neurons_count: 4,
                activation_callback: None,
                activation_function_derivative: None,
            },
            LayerConfig {
                neurons_count: 3,
                activation_callback: None,
                activation_function_derivative: None,
            },
            LayerConfig {
                neurons_count: 5,
                activation_callback: None,
                activation_function_derivative: None,
            },
        ],
        learning_rate: 0.01,
        loose_function: None,
        weights_range: WeightsRange {
            min_weight: 0.0,
            max_weight: 1.0,
        },
    };
}

mod new {
    use crate::neural_network::network::network_test::{
        valid_config_with_one_layer, valid_config_with_three_layers,
    };
    use crate::neural_network::network::Network;

    #[test]
    fn should_create_network_with_one_layers() {
        let network = Network::new(valid_config_with_one_layer());
        assert_eq!(network.layers.len(), 1)
    }

    #[test]
    fn should_create_network_with_three_layers() {
        let network = Network::new(valid_config_with_three_layers());
        assert_eq!(network.layers.len(), 3)
    }
}
