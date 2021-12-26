// use crate::neural_network::layer::layer_config::{LayerConfig, WeightsRange};
// use crate::neural_network::network::network_config::NetworkConfig;
// use crate::neural_network::network::Network;
// use csv::Writer;
// use rand::Rng;
// use serde::Serialize;

mod matrix;
mod neural_network;

fn main() {
    // creat_csv_colors();
    // let values = reade_csv_colors().unwrap();
    // let mut network = Network::new(NetworkConfig {
    //     inputs_count: 3,
    //     hidden_layers: vec![
    //         LayerConfig {
    //             neurons_count: 3,
    //             activation_function: None,
    //             activation_function_derivative: None,
    //             weights_range: WeightsRange {
    //                 min_weight: 0.0,
    //                 max_weight: 1.0,
    //             },
    //         },
    //         LayerConfig {
    //             neurons_count: 3,
    //             activation_function: None,
    //             activation_function_derivative: None,
    //             weights_range: WeightsRange {
    //                 min_weight: 0.0,
    //                 max_weight: 1.0,
    //             },
    //         },
    //     ],
    //     learning_rate: 0.01,
    //     loose_function: None,
    //     output_layer: LayerConfig {
    //         neurons_count: 3,
    //         activation_function: None,
    //         activation_function_derivative: None,
    //         weights_range: WeightsRange {
    //             min_weight: 0.0,
    //             max_weight: 1.0,
    //         },
    //     },
    // });
    // network.train(values.inputs, values.outputs, 2);
    // println!("Inputs: ------------------------------------------");
    // values.inputs.iter().for_each(|v| {
    //     println!("{:?}", v);
    // });
    // println!("Outputs: ------------------------------------------");
    // values.outputs.iter().for_each(|v| {
    //     println!("{:?}", v);
    // });
}

// struct ExternalConfig {
//     inputs: Vec<Vec<f64>>,
//     outputs: Vec<Vec<f64>>,
// }
//
// #[derive(Serialize)]
// struct Row<'a> {
//     red: u8,
//     green: u8,
//     blue: u8,
//     label: &'a str,
// }
//
// fn creat_csv_colors() -> Result<(), Box<dyn Error>> {
//     let mut wtr = Writer::from_path("./static/colors.csv")?;
//     let mut i: u32 = 0;
//     while i < 100 {
//         let label: String;
//         let rgb: Vec<u8> = vec![
//             rand::thread_rng().gen(),
//             rand::thread_rng().gen(),
//             rand::thread_rng().gen(),
//         ];
//         let mut rgb_iter = rgb.iter().enumerate();
//         let init = rgb_iter.next().ok_or("Need at least one input")?;
//         let result = rgb_iter
//             .try_fold(init, |acc, x| {
//                 let cmp = x.1.partial_cmp(acc.1)?;
//                 let max = if let std::cmp::Ordering::Greater = cmp {
//                     x
//                 } else {
//                     acc
//                 };
//                 Some(max)
//             })
//             .unwrap_or_else(|| panic!("Sth went wrong"));
//         if result.0 == 0 {
//             label = String::from("red")
//         } else if result.0 == 1 {
//             label = String::from("green")
//         } else {
//             label = String::from("blue")
//         }
//         wtr.serialize(Row {
//             red: rgb[0],
//             green: rgb[1],
//             blue: rgb[2],
//             label: label.as_str(),
//         })?;
//         i += 1;
//     }
//     wtr.flush()?;
//     Ok(())
// }
//
// fn reade_csv_colors() -> Result<ExternalConfig, Box<dyn Error>> {
//     let mut rdr = csv::Reader::from_path("./static/colors.csv")?;
//     let mut inputs: Vec<Vec<f64>> = vec![];
//     let mut outputs: Vec<Vec<f64>> = vec![];
//     for row in rdr.records() {
//         let mut row_inputs: Vec<f64> = vec![];
//         let mut row_outputs: Vec<f64> = vec![];
//         for (index, record) in row.unwrap().iter().enumerate() {
//             if index == 3 {
//                 row_outputs = output_value_from_label(record)
//             } else {
//                 row_inputs.push(record.parse::<f64>().unwrap())
//             }
//         }
//         inputs.push(row_inputs);
//         outputs.push(row_outputs);
//     }
//     Ok(ExternalConfig { inputs, outputs })
// }
//
// fn output_value_from_label(label: &str) -> Vec<f64> {
//     match label {
//         "red" => vec![1.0, 0.0, 0.0],
//         "green" => vec![0.0, 1.0, 0.0],
//         "blue" => vec![0.0, 0.0, 1.0],
//         _ => vec![],
//     }
// }
