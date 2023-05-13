#[macro_export]
macro_rules! show_info {
    () => {
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
    };
}
