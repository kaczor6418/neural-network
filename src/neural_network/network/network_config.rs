pub struct NetworkConfig {
    pub inputs: Vec<f64>,
    pub weights_range: WeightsRange,
    pub layers: Vec<LayerConfig>,
}

pub struct LayerConfig {
    pub activation_callback: Option<fn(value: f64) -> f64>,
    pub neurons_count: usize
}

pub struct WeightsRange {
    pub min_weight: f64,
    pub max_weight: f64
}
