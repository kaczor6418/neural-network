use crate::neural_network::layer::Layer;

pub mod config;

pub struct Network {
    layers: Vec<Layer>
}

impl Network {
    pub fn new(layers: Option<Vec<Layer>>) -> Network {
        return Network {
            layers: layers.unwrap_or(vec![])
        };
    }
}


#[cfg(test)]
mod network_test;
