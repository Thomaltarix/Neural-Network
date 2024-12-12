use std::fs::File;
use std::io::Write;
use std::vec::Vec;
use my_torch_generator::models::{Configuration, NeuralNetwork};
use my_torch_generator::parsing::get_full_path_file;
use my_torch_generator::parsing::parse_command_line_arguments;

fn create_files(configs: Vec<(String, Configuration)>) {
    for (i, config) in configs.iter().enumerate() {
        let mut complete_file_path = get_full_path_file(String::new());
        let file_name : Vec<&str> = config.0.split("/").collect();
        let tmp : Vec<&str> = file_name[file_name.len() - 1].split(".").collect();
        for j in 0..file_name.len() - 1 {
            complete_file_path.push_str(file_name[j]);
            complete_file_path.push_str("/");
        }
        complete_file_path.push_str(format!("{}{}{}", tmp[0], i.to_string(), ".nn").as_str());
        let mut file = File::create(complete_file_path).expect("Unable to create file");
        let nn = NeuralNetwork::new(config.1.clone());
        file.write_all(nn.get_json().as_bytes()).expect("Unable to write data");
    }
}

fn display_help() {
    println!("USAGE\n\
    \t./my_torch_generator config_file_1 nb_1 [config_file_2 nb_2...]\n\n\
    DESCRIPTION\n\
    \tconfig_file_i\tConfiguration file containing description of the neural network we want to generate.\n\
    \tnb_i\t\tNumber of neural networks we want to generate from the configuration file.\n");
}

fn check_help_option() -> bool{
    let args: Vec<String> = std::env::args().collect();
    if (args.len() == 2 && args[1] == "-h") || (args.len() == 2 && args[1] == "--help") {
        display_help();
        return true;
    }
    false
}

fn main() {
    if check_help_option() {
        std::process::exit(0);
    }
    let tmp : Vec<(String, Configuration)> = parse_command_line_arguments();
    if tmp.is_empty() {
        display_help();
        std::process::exit(84);
    }
    create_files(tmp);
}
