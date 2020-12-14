use crate::neural_network::layer::Layer;

pub mod config;

pub struct Network {
    layers: Vec<Layer>,
    inputs: Vec<f64>
}

impl Network {
    pub fn new(layers: Option<Vec<Layer>>) -> Network {
        return Network {
            layers: layers.unwrap_or(vec![]),
            inputs: vec![]
        };
    }

    // pub fn configure_network(&mut self, config: Config) {
    //     self.inputs = config.inputs;
    //     config.layers.iter().for_each(|layer| {
    //         let neural_layer = Layer::new(None)
    //     })
    // }

}


#[cfg(test)]
mod network_test;
