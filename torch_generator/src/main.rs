use std::fs::File;
use std::io::Write;
use std::vec::Vec;
use my_torch_generator::models::Configuration;
use my_torch_generator::parsing::command_line_parsing::get_full_path_file;
use my_torch_generator::parsing::parse_command_line_arguments;

fn create_files(configs: Vec<(String, Configuration)>) -> std::io::Result<()>{
    for (i, config) in configs.iter().enumerate() {
        let file_path = format!("{}{}", format!("{}{}", get_full_path_file(String::new()), config.0), i);
        println!("{}", file_path);
        let mut file = File::create(file_path)?;
        file.write_all(b"Hello, world!")?;
    }
    Ok(())
}

fn main() {
    let tmp : Vec<(String, Configuration)> = parse_command_line_arguments();
    create_files(tmp).expect("Pannnnnniiccc");
}
