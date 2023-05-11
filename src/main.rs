use handler::args_handler;

mod data;
mod config;
mod handler;
fn main() {
    args_handler::handle_args();
}
