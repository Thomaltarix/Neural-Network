use std::sync::{Arc, Mutex};
use crate::models::NeuralNetworkData;

type ActivationFunction = fn(Vec<f64>) -> f64;

#[derive(Clone, Debug)]
pub struct Node {
    weights: Vec<f64>,
    bias: f64,
    activation: ActivationFunction,
    learning_rate: f64,
    inputs: Vec<Option<Arc<Mutex<f64>>>>,
    output: Arc<Mutex<f64>>,
}

impl Node {
    pub fn new(nb_inputs: usize, activation: ActivationFunction) -> Node {
        Node {
            weights: vec![1.0; nb_inputs],
            bias: 0.0,
            activation,
            learning_rate: 0.0,
            inputs: vec![None; nb_inputs],
            output: Arc::new(Mutex::new(0.0)),
        }
    }

    pub fn new_with_weights(weights: Vec<f64>, bias: f64, activation: ActivationFunction, learning_rate: f64) -> Node {
        Node {
            weights : weights.clone(),
            bias,
            activation,
            learning_rate,
            inputs: vec![None; weights.len()],
            output: Arc::new(Mutex::new(0.0)),
        }
    }

    pub fn set_weight(&mut self, weight: f64, index: usize) {
        self.weights[index] = weight;
    }

    pub fn set_bias(&mut self, bias: f64) {
        self.bias = bias;
    }

    pub fn set_activation(&mut self, activation: ActivationFunction) {
        self.activation = activation;
    }

    pub fn set_learning_rate(&mut self, learning_rate: f64) {
        self.learning_rate = learning_rate;
    }

    pub fn set_input(&mut self, input: &Arc<Mutex<f64>>, index: usize) {
        self.inputs[index] = Some(Arc::clone(input));
    }

    pub fn get_weight(&self) -> Vec<f64> {
        self.weights.clone()
    }

    pub fn get_bias(&self) -> f64 {
        self.bias
    }

    pub fn get_activation(&self) -> ActivationFunction {
        self.activation
    }

    pub fn get_learning_rate(&self) -> f64 {
        self.learning_rate
    }

    pub fn get_input(&self) -> Vec<Option<Arc<Mutex<f64>>>> {
        self.inputs.clone()
    }

    pub fn get_output(&self) -> &Arc<Mutex<f64>> {
        &self.output
    }

    pub fn calculate_output(&mut self) {
        let mut sum = self.bias;
        for i in 0..self.inputs.len() {
            if self.inputs[i].as_ref().is_none() {
                continue;
            }
            sum += self.weights[i] * self.inputs[i].as_ref().unwrap().lock().unwrap().clone();
        }
        let mut output_lock = self.output.lock().unwrap();
        *output_lock = (self.activation)(vec![sum]);
    }
}

#[derive(Debug)]
pub struct NodeLayer {
    nodes: Vec<Node>,
}

impl NodeLayer {
    pub fn new() -> NodeLayer {
        NodeLayer {
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn get_node(&self, index: usize) -> &Node {
        &self.nodes[index]
    }

    pub fn get_node_mut(&mut self, index: usize) -> &mut Node {
        &mut self.nodes[index]
    }

    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    pub fn get_nodes_mut(&mut self) -> &mut Vec<Node> {
        &mut self.nodes
    }

    pub fn calculate_output(&mut self) {
        for node in self.nodes.iter_mut() {
            node.calculate_output();
        }
    }

    pub fn  connect_layers(&mut self, next_layer : &mut NodeLayer) {
        for i in 0..self.nodes.len() {
            for j in 0..next_layer.nodes.len() {
                next_layer.nodes[j].set_input(self.nodes[i].get_output(), i);
            }
        }
    }

    pub fn connect_output(&mut self, next_layer: &mut NodeLayer) {
        for node in self.nodes.iter_mut() {
            let mut i : usize = 0;
            for next_node in next_layer.get_nodes_mut().iter_mut() {
                next_node.set_input(node.get_output(), i);
                i += 1;
            }
        }
    }

    pub fn connect_input(&mut self, previous_layer: &mut NodeLayer) {
        for node in self.nodes.iter_mut() {
            let mut i: usize = 0;
            for previous_node in previous_layer.get_nodes_mut().iter_mut() {
                if node.inputs.len() <= i {
                    node.inputs.resize(i + 1, None); // Ensure there's space in the inputs
                }
                let output = previous_node.get_output();
                node.set_input(output, i); // Clone the Arc reference
                i += 1;
            }
        }
    }
}

#[derive(Debug)]
pub struct NeuralNetwork {
    layers: Vec<NodeLayer>,
}

fn tmp(x: Vec<f64>) -> f64 {
    x[0]
}

fn get_activation_from_string(activation: &str) -> ActivationFunction {
    match activation {
        _ => tmp
    }
}

impl NeuralNetwork {
    pub fn new() -> NeuralNetwork {
        NeuralNetwork {
            layers: Vec::new(),
        }
    }

    pub fn new_with_layers(layers: Vec<usize>, bias: f64, learning_rate: f64, activation: ActivationFunction) -> NeuralNetwork {
        let mut perceptron = NeuralNetwork::new();
        for i in 0..layers.len() {
            let mut layer = NodeLayer::new();
            for _ in 0..layers[i] {
                let mut node = Node::new(1, activation);
                node.set_bias(bias);
                node.set_learning_rate(learning_rate);
                node.set_activation(activation);
                layer.add_node(node);
            }
            perceptron.add_layer(layer);
        }
        perceptron.connect_layers();
        perceptron
    }

    pub fn new_from_data(data : NeuralNetworkData) -> NeuralNetwork {
        let mut nn = NeuralNetwork::new();
        for layer in data.layers.iter() {
            let mut node_layer = NodeLayer::new();
            for neuron in layer.iter() {
                node_layer.add_node(Node::new_with_weights(neuron.weights.clone(), neuron.bias, ActivationFunction::from(get_activation_from_string(neuron.activation.as_str())), data.learning_rate));
            }
            nn.add_layer(node_layer);
        }
        nn.connect_layers();
        nn
    }

    pub fn add_layer(&mut self, layer: NodeLayer) {
        self.layers.push(layer);
    }

    pub fn add_node_to_layer(&mut self, node: Node, index: usize) {
        self.layers[index].add_node(node);
    }

    pub fn get_layer(&self, index: usize) -> &NodeLayer {
        &self.layers[index]
    }

    pub fn get_layer_mut(&mut self, index: usize) -> &mut NodeLayer {
        &mut self.layers[index]
    }

    pub fn get_layers(&self) -> &Vec<NodeLayer> {
        &self.layers
    }

    pub fn get_layers_mut(&mut self) -> &mut Vec<NodeLayer> {
        &mut self.layers
    }

    pub fn calculate_output(&mut self) {
        for layer in self.layers.iter_mut() {
            layer.calculate_output();
        }
    }

    pub fn connect_layers(&mut self) {
        if self.layers.len() < 2 {
            return;
        }
        for i in 0..self.layers.len() - 1 {
            let (first, second) = self.layers.split_at_mut(i + 1);
            first[i].connect_layers(&mut second[0]);
        }
    }

    pub fn get_output(&self) -> Vec<f64> {
        let mut output = Vec::new();
        for node in self.layers.last().unwrap().get_nodes().iter() {
            output.push(node.get_output().lock().unwrap().clone());
        }
        output
    }

    pub fn set_input(&mut self, input: Vec<f64>, index: usize) {
        if self.layers[0].get_nodes().len() <= index {
            panic!("There is no node at index {}", index);
        }
        if self.layers[0].get_node(index).inputs.len() < input.len() {
            panic!("The node at index {} does not have enough inputs", index);
        }
        for i in 0..input.len() {
            self.layers[0].get_node_mut(index).set_input(&Arc::new(Mutex::new(input[i])), i);
        }
    }

    pub fn set_inputs(&mut self, inputs: Vec<Vec<f64>>) {
        for i in 0..inputs.len() {
            self.set_input(inputs[i].clone(), i);
        }
    }
}
