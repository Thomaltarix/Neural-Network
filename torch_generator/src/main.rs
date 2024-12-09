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

fn main() {
    let tmp : Vec<(String, Configuration)> = parse_command_line_arguments();
    create_files(tmp)
}
