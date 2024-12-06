use core::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub input_by_input_neuron: usize,
    pub bias: f64,
    pub learning_rate: f64,
    pub layers: Vec<u8>,
}

impl Configuration {
    pub fn new(data: String) -> Self {
        let json_elem : Configuration = serde_json::from_str(data.as_str()).expect("Failed to parse JSON");
        Configuration {
            input_by_input_neuron: json_elem.input_by_input_neuron,
            bias: json_elem.bias,
            learning_rate: json_elem.learning_rate,
            layers: json_elem.layers,
        }
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "input_by_input_neuron: {}\nbias: {}\nlearning_rate: {}\nlayers: {:?}", self.input_by_input_neuron, self.bias, self.learning_rate, self.layers)
    }
}
