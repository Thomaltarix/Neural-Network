use my_torch_generator::models::Configuration;
use my_torch_generator::models::NeuralNetwork;

#[test]
fn test_configuration_new() {
    let config = Configuration::new("{\
    \"input_by_input_neuron\": 2,\
    \"bias\": 0.5,\
    \"learning_rate\": 0.1,\
    \"layers\": [2, 3, 1],\
    \"activation\": \"test\"\
}".to_string());
    assert_eq!(config.input_by_input_neuron, 2);
    assert_eq!(config.bias, 0.5);
    assert_eq!(config.learning_rate, 0.1);
    assert_eq!(config.layers, vec![2, 3, 1]);
    assert_eq!(config.activation, "test");
}

#[test]
fn test_neural_network_new() {
    let nn = NeuralNetwork::new(Configuration::new("{\
    \"input_by_input_neuron\": 2,\
    \"bias\": 0.5,\
    \"learning_rate\": 0.1,\
    \"layers\": [2, 3, 1],\
    \"activation\": \"test\"\
}".to_string()));

    assert_eq!(nn.learning_rate, 0.1);
    for layer in nn.layers {
        for neuron in layer {
            assert_eq!(neuron.weights.len(), 2);
            assert_eq!(neuron.bias, 0.5);
            assert_eq!(neuron.activation, "test");

        }
    }
}
