use std::{io::ErrorKind, collections::HashMap, fs::{self}};

use crate::config;

pub fn write_compressed_file(words: Vec<String>, words_and_markers: &HashMap<String, String>) -> Result<(), ErrorKind> {
    let mut compressed_text = String::new();
    for word in words {
        let marker = find_marker_for_word(word.as_str(), &words_and_markers);
        compressed_text.push_str(marker.as_str());

    }
    return write_in_file(compressed_text.as_str(), None);
}

fn find_marker_for_word(word: &str, words_and_markers: &HashMap<String, String>) -> String {
    let marker = words_and_markers.get(&String::from(word));
    return marker.unwrap().clone()
}


fn write_in_file(compressed_text: &str, file_name: Option<&str>) -> Result<(), ErrorKind> {
    let output_file = fs::write(file_name.unwrap_or(config::OUTPUT_COMPRESSED_FILE), compressed_text);

    if output_file.is_err() {
        eprintln!("Ocorreu um erro ao criar o arquivo de texto comprimido");
        return Err(ErrorKind::InvalidData);
    }    
    return Ok(());
}


pub fn write_key_file(words_and_markers: &HashMap<String, String>, file_path: Option<&str>) -> Result<(), ErrorKind> {
    let mut file_content = String::new();
    for (key, marker) in words_and_markers.clone().iter() {
        file_content.push_str(format!("[{}] => [{}] \n", key, marker).as_str());
    }
    let result = fs::write(file_path.unwrap_or(config::DECOMPRESSION_KEY), file_content);
    if result.is_err() {
        eprintln!("Ocorreu um erro ao escrever o arquivo de chave!");
        return Err(ErrorKind::InvalidData);
    }
    return Ok(());
}