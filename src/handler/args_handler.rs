use std::{path::{PathBuf, Path}, io::ErrorKind, fs};
pub fn get_file_from_args() -> Result<PathBuf, ErrorKind> {
    let mut args: Vec<String> = std::env::args().collect();
    args = Vec::from(&args.clone()[1..]);
    if args.len() == 0 {
        eprintln!("Argumentos insuficientes");
        return Err(ErrorKind::InvalidInput);
    }
    let current_dir = std::env::current_dir();
    if current_dir.is_err() {
        eprintln!("Ocorreu um erro ao buscar diretório atual");
        return Err(ErrorKind::InvalidData);
    }
    let path = Path::new(&args[0]);
    let r_path = format!("{}/{}", current_dir.unwrap().to_str().unwrap(), path.to_str().unwrap());
    let r_as_path = Path::new(r_path.as_str()).to_path_buf();
    if fs::read(&r_as_path).is_err() {
        eprintln!("Ocorreu um erro ao verificar existência de arquivo: {}", &r_as_path.display());
        return Err(ErrorKind::InvalidInput);
    }
    return Ok(r_as_path);
}


