use crate::neural_network::network::config::Config;

fn valid_config_with_three_layers() -> Config {
    return Config {
        inputs: vec![1.0, 2.0, 3.0],
        layers_neurons_weights: vec![
            vec![
                vec![1.0, 2.0, 3.0],
                vec![1.0, 2.0, 3.0],
                vec![1.0, 2.0, 3.0]
            ],
            vec![
                vec![2.0, 3.0, 4.0],
                vec![2.0, 3.0, 4.0]
            ],
            vec![
                vec![3.0, 4.0, 5.0],
            ]
        ],
    };
}

fn invalid_config() -> Config {
    return Config {
        inputs: vec![1.0, 2.0, 3.0],
        layers_neurons_weights: vec![
            vec![
                vec![1.0, 2.0, 3.0],
                vec![1.0, 2.0],
                vec![1.0]
            ],
            vec![
                vec![2.0, 3.0, 4.0],
                vec![2.0, 3.0]
            ],
            vec![
                vec![3.0, 4.0, 5.0],
            ]
        ],
    };
}

mod new {
    use crate::neural_network::network::Network;
    use crate::neural_network::layer::Layer;

    #[test]
    fn should_create_empty_network() {
        let network = Network::new(None);
        assert!(network.layers.is_empty());
    }

    #[test]
    fn should_create_network_with_two_layers() {
        let layers = vec![Layer::new(None, None)];
        let network = Network::new(Some(layers));
        assert!(!network.layers.is_empty())
    }
}

mod configure_network {
    use crate::neural_network::network::Network;
    use crate::neural_network::network::network_test::{valid_config_with_three_layers, invalid_config};

    #[test]
    fn should_configure_network_for_valid_config() {
        let network = Network::new(None);
        network.configure_network(valid_config_with_three_layers());
        assert_eq!(network.layers, 3)
    }

    #[test]
    #[should_panic]
    fn should_not_configure_network_for_invalid_config() {
        let network = Network::new(None);
        network.configure_network(invalid_config());
    }
}
