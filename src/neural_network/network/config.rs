pub struct Config {
    pub inputs: Vec<f64>,
    pub layers: Vec<LayerConfig>,
}

pub union LayerConfig {
    pub layer_config_with_neurons_weights: LayerConfigWithNeuronsWeights,
    pub layer_config_with_neurons_count: LayerConfigWithNeuronsCount,
}

pub struct BaseLayerConfig {
    pub activation_callback: Option<fn(value: f64) -> f64>
}

pub struct LayerConfigWithNeuronsWeights {
    pub base_data: BaseLayerConfig,
    pub neurons_weights: Vec<Vec<f64>>,
}


pub struct  LayerConfigWithNeuronsCount {
    pub base_data: BaseLayerConfig,
    pub neurons_count: usize,
}
