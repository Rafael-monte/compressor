use std::{path::{PathBuf, Path}, fs};

fn get_file_from_args(fp: String) -> PathBuf {
    let current_dir = std::env::current_dir();
    if current_dir.is_err() {
        eprintln!("Ocorreu um erro ao buscar diretório atual");
        panic!();
    }
    let path = Path::new(&fp);
    let r_path = format!("{}/{}", current_dir.unwrap().to_str().unwrap(), path.to_str().unwrap());
    let r_as_path = Path::new(r_path.as_str()).to_path_buf();
    if fs::read(&r_as_path).is_err() {
        eprintln!("Ocorreu um erro ao verificar existência de arquivo: {}", &r_as_path.display());
        panic!();
    }
    return r_as_path;
}

pub fn handle_args() -> Option<PathBuf> {
    let args = get_args();
    let first_arg: String = args[0].clone();
    match first_arg.as_str() {
        "-c" => {
            if args.len() < 2 {
                eprintln!("Era esperado um arquivo de entrada como segundo argumento");
                panic!();
            }
            let path_arg: String = args[1].clone();
            return Some(get_file_from_args(path_arg))
        },
        "-i" => {
            show_info();
            return None
        }
        _ => {return None}
    }
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
    println!("Compressor é um aplicativo feito em Rust que permite comprimir arquivos de texto gerando uma chave que pode reverter a compressão");
    println!("Como usar 🏷️");
    println!("-------------------------------------------------------------------------");
    println!("");
    println!("\tcompressor [OPTION]... [FILE]...");
    println!("\tDescrição:");
    println!("\t\t-c");
    println!("\t\t\tCompress the file");
    println!("\t\t-i");
    println!("\t\t\tshow info about project");
    println!("\t\tfile: arquivo");
    println!("");
    println!("-------------------------------------------------------------------------");
}
