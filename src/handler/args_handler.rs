use std::{path::{PathBuf, Path}, fs};

use crate::{data::{compressor, decompressor}, config};



pub fn handle_args() {
    let args = get_args();
    let first_arg: String = args[0].clone();
    match first_arg.as_str() {
        "-c" => {
            if args.len() < 2 {
                eprintln!("An input file as 2nd argument was expected.");
                panic!();
            }
            let path_arg: String = args[1].clone();
            let path: PathBuf = get_full_path(path_arg.as_str());
            check_file_extension(&path, config::TEXT_FILE_EXTENSION);
            compressor::compress_text(path);
        },
        "-d" => {
            if args.len() < 3 {
                eprintln!("An input file as 2nd argumented and an key file as 3rd argument was expected.");
                panic!();
            }
            let path_target_file: String = args[1].clone();
            let path_key_file = args[2].clone();
            let full_paths: (PathBuf, PathBuf) = (get_full_path(path_target_file.as_str()), get_full_path(path_key_file.as_str()));
            check_file_extension(&full_paths.0, config::COMPRESSED_FILE_EXTENSION);
            check_file_extension(&full_paths.1, config::COMPRESSED_KEY_FILE_EXTENSION);
            decompressor::decompress_file(full_paths.0, full_paths.1);
        },
        "-i" => {
            show_info();
        }
        _ => {}
    }
}


fn get_full_path(argument_file_path: &str) -> PathBuf {
    let path = Path::new(argument_file_path);
    let current_dir = get_current_directory();
    let r_path = format!("{}/{}", current_dir.to_str().unwrap(), path.to_str().unwrap());
    let r_as_path = Path::new(r_path.as_str()).to_path_buf();
    if fs::read(&r_as_path).is_err() {
        eprintln!("An error occoured when verify file existence: {}", &r_as_path.display());
        panic!();
    }
    return r_as_path;
}

fn get_current_directory() -> PathBuf {
    let current_dir = std::env::current_dir();
    if current_dir.is_err() {
        eprintln!("An error occoured while search the current directory.");
        panic!();
    }
    return current_dir.unwrap();
}

fn check_file_extension(file_path: &PathBuf, extension: &str) {
    let f_p_binding = file_path.clone();
    let path_as_str: &str = f_p_binding.to_str().unwrap();
    let separator_index: usize = path_as_str.rfind(config::FILE_EXTENSION_SEPARATOR).unwrap();
    let path_extension: &str = &path_as_str[separator_index..];
    if path_extension != extension {
        eprintln!("Cannot do operation with file type.\nExpected: \"{}\", found: \"{}\"", extension, path_extension);
        panic!();
    }
}

fn get_args() -> Vec<String> {
    let mut args: Vec<String> = std::env::args().collect();
    args = Vec::from(&args.clone()[1..]);
    if args.len() < 1 {
        eprintln!("Not enough arguments");
        panic!();
    }
    return args;
}

fn show_info() {
    println!("");
    println!("Infos 📦");
    println!("");
    println!("Compressor is an app Rust-made app that allows compress text files generating a key that can revert the compression");
    println!("");
    println!("How to use it? 🏷️");
    println!("-------------------------------------------------------------------------");
    println!("");
    println!("\tcompressor [OPTION]... [INPUT_FILE]... [KEY]...");
    println!("\t\x1B[1m{}\x1B[0m", "DESCRIPTION:");
    println!("\t\t\x1B[1m{}\x1B[0m", "-c");
    println!("\t\t\tCompress the file");
    println!("\t\t\x1B[1m{}\x1B[0m","-d");
    println!("\t\t\tDecompress the file, giving a key as 3rd parameter");
    println!("\t\t\x1B[1m{}\x1B[0m", "-i");
    println!("\t\t\tshow info about project");
    println!("");
    println!("-------------------------------------------------------------------------");
    println!("");
    println!("The output file always has the \".rco\" (Rust Compressed Output) extension, while the key file has the \".rcok\" (Rust Compressed Output Key) extension.");
    println!("");
}
