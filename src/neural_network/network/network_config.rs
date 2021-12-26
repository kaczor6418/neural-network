use crate::neural_network::layer::layer_config::LayerConfig;

pub struct NetworkConfig {
    pub hidden_layers: Vec<LayerConfig>,
    pub inputs_count: usize,
    pub learning_rate: f64,
    pub loose_function: Option<fn(expected: &f64, predicted: &f64) -> f64>,
    pub output_layer: LayerConfig,
}
