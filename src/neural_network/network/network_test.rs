use crate::neural_network::network::network_config::{NetworkConfig, WeightsRange, LayerConfig};

fn valid_config_with_one_layer() -> NetworkConfig {
    return NetworkConfig {
        weights_range: WeightsRange { min_weight: 0.0, max_weight: 1.0},
        inputs: vec![1.0, 2.0, 3.0],
        layers: vec![
            LayerConfig {
                neurons_count: 2,
                activation_callback: None
            }
        ]
    };
}

fn valid_config_with_three_layers() -> NetworkConfig {
    return NetworkConfig {
        weights_range: WeightsRange { min_weight: 0.0, max_weight: 1.0},
        inputs: vec![1.0, 2.0, 3.0],
        layers: vec![
            LayerConfig {
                neurons_count: 4,
                activation_callback: None
            },
            LayerConfig {
                neurons_count: 3,
                activation_callback: None
            },
            LayerConfig {
                neurons_count: 5,
                activation_callback: None
            },
        ]
    };
}

mod new {
    use crate::neural_network::network::Network;
    use crate::neural_network::network::network_test::{valid_config_with_three_layers, valid_config_with_one_layer};

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

mod forward_propagation {
    use crate::neural_network::network::Network;
    use crate::neural_network::network::network_test::{valid_config_with_three_layers, valid_config_with_one_layer};

    #[test]
    fn should_contains_output_with_two_values() {
        let network = Network::new(valid_config_with_one_layer());
        assert_eq!(network.forward_propagation().len(), 2);
    }

    #[test]
    fn should_contains_output_with_five_values() {
        let network = Network::new(valid_config_with_three_layers());
        assert_eq!(network.forward_propagation().len(), 5);
    }
}
