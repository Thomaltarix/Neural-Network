use my_torch_generator::models::Configuration;

#[test]
fn test_configuration_new() {
    let config = Configuration::new("{\
    \"input_by_input_neuron\": 2,\
    \"bias\": 0.5,\
    \"learning_rate\": 0.1,\
    \"layers\": [2, 3, 1]}".to_string());
    assert_eq!(config.input_by_input_neuron, 2);
    assert_eq!(config.bias, 0.5);
    assert_eq!(config.learning_rate, 0.1);
    assert_eq!(config.layers, vec![2, 3, 1]);
}
