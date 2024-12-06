use std::io::Read;
use crate::models::Configuration;

pub fn get_full_path_file(path : String) -> String {
    format!("{}/{}", std::env::current_dir().unwrap().to_str().unwrap(), path)
}

fn get_content_from_file(path : String) -> String {
    let mut file = std::fs::File::open(get_full_path_file(path)).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read file");
    data
}

pub fn parse_command_line_arguments() -> Vec<(String, Configuration)> {
    let args: Vec<String> = std::env::args().collect();
    let mut pair : bool = false;
    let mut first : bool = true;
    let mut tmp : (String, Option<Configuration>) = (String::new(), None);
    let mut config: Vec<(String, Configuration)> = Vec::new();

    if args.len() == 1 {
        println!("No arguments given");
        return config;
    }
    for arg in args.iter() {
        if first {
            first = false;
            continue;
        }
        if arg.contains(".conf") {
            tmp = (arg.clone(), Option::from(Configuration::new(get_content_from_file(arg.to_string()))));
            pair = true;
        } else {
            if tmp.0.is_empty() {
                continue;
            }
            let number = arg.parse::<usize>().unwrap();
            for _ in 0..number {
                config.push((tmp.0.clone(), tmp.1.clone().unwrap()));
            }
            pair = false;
        }
    }
    if pair == true {
        config.clear();
    }
    if config.is_empty() {
        panic!("No configuration found");
    }
    config
}
