use data::compressor;
use handler::args_handler;

mod data;
mod config;
mod handler;
fn main() {
    let res = args_handler::handle_args();
    if let Some(path) = res {
        compressor::compress_text(path);
    }
}
