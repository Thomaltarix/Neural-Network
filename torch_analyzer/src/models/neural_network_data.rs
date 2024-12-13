use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Neuron {
    pub weights: Vec<f64>,
    pub bias: f64,
    pub activation: String
}

impl Neuron {
    pub fn new(data : String) -> Self {
        let json_elem : Neuron = serde_json::from_str(data.as_str()).expect("Failed to parse JSON");
        Neuron {
            weights: json_elem.weights,
            bias: json_elem.bias,
            activation: json_elem.activation
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NeuralNetworkData {
    pub learning_rate: f64,
    pub layers : Vec<Vec<Neuron>>
}

impl NeuralNetworkData {
    pub fn new(data : String) -> Self {
        let json_elem : NeuralNetworkData = serde_json::from_str(data.as_str()).expect("Failed to parse JSON");
        NeuralNetworkData {
            learning_rate: json_elem.learning_rate,
            layers: json_elem.layers
        }
    }
}
