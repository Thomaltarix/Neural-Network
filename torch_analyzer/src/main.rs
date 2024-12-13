use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use my_torch_analyzer::models::NeuralNetworkData;
use my_torch_analyzer::neural_network::NeuralNetwork;

#[derive (PartialEq, Debug)]
enum Mode {
    Train,
    Predict,
}

enum Save {
    Save(String),
    NoSave,
}

impl Display for Save {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Save::Save(s) => write!(f, "{}", s),
            Save::NoSave => write!(f, "NoSave"),
        }
    }
}

fn display_help() {
    println!("USAGE\n\
    \t./my_torch_analyzer [--predict || --train [--save SAVEFILE]] LOADFILE FILE\n\n\
    DESCRIPTION\n\
    \t--train Launch the neural network in training mode. Each chessboard in FILE must contain inputs to send to the neural network in FEN notation and the expected output separated by space. If specified, the newly trained neural network will be saved in SAVEFILE. Otherwise, it will be saved in the original LOADFILE.\n\
    \t--predict Launch the neural network in prediction mode. Each chessboard in FILE must contain inputs to send to the neural network in FEN notation, and optionally an expected output.\n\
    \t--save Save the neural network in SAVEFILE. Only works in train mode\n\n\
    \tLOADFILE File containing an artificial neural network\n\
    \tFILE File containing chessboards\n");
}

fn parse_options() -> (Mode, Save, String, String) {
    let args: Vec<String> = std::env::args().collect();
    let mode : Mode;

    if args.len() < 2 {
        println!("Invalid number of arguments");
        display_help();
        std::process::exit(84);
    }
    if args[1] == "--help" ||args[1] == "-h" {
        display_help();
        std::process::exit(0);
    }
    if args[1] == "--train" {
        mode = Mode::Train;
    } else if args[1] == "--predict" {
        mode = Mode::Predict;
    } else {
        println!("Invalid option");
        display_help();
        std::process::exit(84);
    }
    if mode == Mode::Train {
        if args.len() < 4 {
            println!("Invalid number of arguments");
            display_help();
            std::process::exit(84);
        } else {
            if args[2] == "--save" {
                if args.len() < 6 {
                    println!("Invalid number of arguments");
                    display_help();
                    std::process::exit(84);
                }
                return (mode, Save::Save(args[3].clone()), args[4].clone(), args[5].clone());
            }
            return (mode, Save::NoSave, args[2].clone(), args[3].clone());
        }
    }
    if args.len() < 4 {
        println!("Invalid number of arguments");
        display_help();
        std::process::exit(84);
    }
    (mode, Save::NoSave, args[2].clone(), args[3].clone())
}

fn verify_files_exist(load_file: &str, file: &str) -> bool {
    if !std::path::Path::new(load_file).exists() {
        println!("LOADFILE does not exist");
        return true;
    }
    if !std::path::Path::new(file).exists() {
        println!("FILE does not exist");
        return true;
    }
    false
}

fn get_full_path_file(path : String) -> String {
    format!("{}/{}", std::env::current_dir().unwrap().to_str().unwrap(), path)
}

fn get_file_content(path : String) -> String {
    let mut file = File::open(get_full_path_file(path)).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read file");
    data
}

fn main() {
    let (_mode, _save, load_file, file) = parse_options();
    if verify_files_exist(&load_file, &file) {
        std::process::exit(84);
    }
    let _ = NeuralNetwork::new_from_data(NeuralNetworkData::new(get_file_content(load_file)));
}
