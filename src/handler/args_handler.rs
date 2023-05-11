use std::{path::{PathBuf, Path}, fs};

use crate::data::{compressor, decompressor};



pub fn handle_args() {
    let args = get_args();
    let first_arg: String = args[0].clone();
    match first_arg.as_str() {
        "-c" => {
            if args.len() < 2 {
                eprintln!("Era esperado um arquivo de entrada como segundo argumento");
                panic!();
            }
            let path_arg: String = args[1].clone();
            compressor::compress_text(get_full_path(path_arg.as_str()));
        },
        "-d" => {
            if args.len() < 3 {
                eprintln!("Era esperado um arquivo de entrada como segundo argumento e um arquivo key como terceiro argumento.");
                panic!();
            }
            let path_target_file: String = args[1].clone();
            let path_key_file = args[2].clone();
            let full_paths: (PathBuf, PathBuf) = (get_full_path(path_target_file.as_str()), get_full_path(path_key_file.as_str()));
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
        eprintln!("Ocorreu um erro ao verificar existência de arquivo: {}", &r_as_path.display());
        panic!();
    }
    return r_as_path;
}

fn get_current_directory() -> PathBuf {
    let current_dir = std::env::current_dir();
    if current_dir.is_err() {
        eprintln!("Ocorreu um erro ao buscar diretório atual");
        panic!();
    }
    return current_dir.unwrap();
}

fn get_args() -> Vec<String> {
    let mut args: Vec<String> = std::env::args().collect();
    args = Vec::from(&args.clone()[1..]);
    if args.len() < 1 {
        eprintln!("Argumentos insuficientes");
        panic!();
    }
    return args;
}

fn show_info() {
    println!("");
    println!("Informações 📦");
    println!("");
    println!("Compressor é um aplicativo feito em Rust que permite comprimir arquivos de texto gerando uma chave que pode reverter a compressão.");
    println!("");
    println!("Como usar 🏷️");
    println!("-------------------------------------------------------------------------");
    println!("");
    println!("\tcompressor [OPTION]... [FILE]...");
    println!("\tDescrição:");
    println!("\t\t-c");
    println!("\t\t\tCompress the file");
    println!("\t\t-i");
    println!("\t\t\tshow info about project");
    println!("");
    println!("-------------------------------------------------------------------------");
    println!("");
    println!("O arquivo de saída sempre irão possuir a extensão \".rco\" (Rust Compressed Output) enquanto a chave terá a extensão \".rcok\" (Rust Compressed Output Key)");
    println!("");
}
