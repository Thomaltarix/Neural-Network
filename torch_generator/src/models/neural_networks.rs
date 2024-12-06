use core::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Neuron {
    pub weights: Vec<f64>,
    pub bias: f64,
    pub activation: f64
}

impl Neuron {
    pub fn new(weights: Vec<f64>, bias: f64, activation: f64) -> Self {
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
    pub fn new(learning_rate: f64, layers: Vec<Vec<Neuron>>) -> Self {
        NeuralNetwork {
            learning_rate,
            layers
        }
    }
}

impl fmt::Display for NeuralNetwork {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "learning_rate: {}\nlayers: {:?}", self.learning_rate, self.layers)
    }
}
