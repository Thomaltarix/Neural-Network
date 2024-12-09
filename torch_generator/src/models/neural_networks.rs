use core::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use crate::models::Configuration;
use rand::Rng;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Neuron {
    pub weights: Vec<f64>,
    pub bias: f64,
    pub activation: String
}

impl Neuron {
    pub fn new(weights: Vec<f64>, bias: f64, activation: String) -> Self {
        Neuron {
            weights,
            bias,
            activation
        }
    }
}

impl fmt::Display for Neuron {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "weights: {:?}\nbias: {}\nactivation: {}", self.weights, self.bias, self.activation)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NeuralNetwork {
    pub learning_rate: f64,
    pub layers : Vec<Vec<Neuron>>
}

impl NeuralNetwork {
    pub fn new(config: Configuration) -> Self {
        let mut layers: Vec<Vec<Neuron>> = Vec::new();
        for i in 0..config.layers.len() {
            let mut neurons: Vec<Neuron> = Vec::new();
            for _ in 0..config.layers[i] {
                let mut weights: Vec<f64> = Vec::new();
                for _ in 0..config.input_by_input_neuron {
                    let mut rng = rand::thread_rng();
                    let value : f64 = rng.gen();
                    weights.push(value);
                }
                neurons.push(Neuron::new(weights, config.bias, config.activation.clone()));
            }
            layers.push(neurons);
        }
        NeuralNetwork {
            learning_rate: config.learning_rate,
            layers
        }
    }

    pub fn get_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

impl fmt::Display for NeuralNetwork {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "learning_rate: {}\nlayers: {:?}", self.learning_rate, self.layers)
    }
}
