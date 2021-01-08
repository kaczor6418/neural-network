pub struct LayerConfig {
    pub activation_function: Option<fn(value: &f64) -> f64>,
    pub activation_function_derivative: Option<fn(value: &f64) -> f64>,
    pub neurons_count: usize,
    pub weights_range: WeightsRange,
}

pub struct WeightsRange {
    pub min_weight: f64,
    pub max_weight: f64,
}
