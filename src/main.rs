use data::compressor;
use handler::args_handler;

mod data;
mod config;
mod handler;
fn main() {
    let args = args_handler::get_file_from_args();
    if args.is_ok() { 
        compressor::compress_text(args.unwrap());
    } else {
        eprintln!("Erro: {}", args.unwrap_err());
    }
}
