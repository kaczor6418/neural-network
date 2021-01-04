pub struct NetworkConfig {
    pub layers: Vec<LayerConfig>,
    pub inputs_count: usize,
    pub learning_rate: f64,
    pub loose_function: Option<fn(expected: &f64, predicted: &f64) -> f64>,
    pub weights_range: WeightsRange,
}

pub struct LayerConfig {
    pub activation_callback: Option<fn(value: f64) -> f64>,
    pub activation_function_derivative: Option<fn(value: &f64) -> f64>,
    pub neurons_count: usize,
}

pub struct WeightsRange {
    pub min_weight: f64,
    pub max_weight: f64,
}
