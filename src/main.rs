use data::compressor;

mod data;
mod config;
fn main() {
    compressor::read_file_and_get_repetitions();
}
