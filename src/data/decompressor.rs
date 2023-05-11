use std::{path::PathBuf, collections::HashMap, fs};

use super::writer;
struct KeyFile {
    size: i32,
    pub hash_values: HashMap<String, String>,
}
struct Decompressor {
    pub content: String,
    pub key_file: KeyFile
}
impl KeyFile {
    pub fn feed(&mut self, content: &str) {
        let k_file_content = self.normalize_file(content);
        for line in k_file_content.lines() {
            let extracted_elements = self.extract_content_from_line(line);
            self.hash_values.insert(extracted_elements.0, extracted_elements.1);
        }
    }


    fn normalize_file(&mut self, mut content: &str) -> String {
        let json_start: usize = content.find("{").unwrap();
        let file_size_as_str: &str = &content[..json_start];
        self.size = file_size_as_str.parse::<i32>().unwrap();
        content = &content[json_start..];
        let replaced: String = content.replace("{", "").replace("}", "").replace(", ", "\n");
        return String::from(replaced);
    }


    fn extract_content_from_line(&mut self, line: &str) -> (String, String) {
        let l_value_idxs: (usize, usize) = (self.char_at_position(0, &'\"', line), self.char_at_position(1, &'\"', line));
        let r_value_idxs: (usize, usize) = (self.char_at_position(2, &'\"', line), self.char_at_position(3, &'\"', line));
        let l_value: String = String::from(&line[l_value_idxs.0 .. (l_value_idxs.1 + 1)]).replace("\"", "");
        let r_value: String = String::from(&line[r_value_idxs.0 .. (r_value_idxs.1 + 1)]).replace("\"", "");
        return (l_value, r_value);
    }

    fn char_at_position(&self, position: u8, target: &char, line: &str) -> usize {
        let mut count = 0;
        for (i, c) in line.char_indices() {
            if *target == c {
                if count == position {
                    return i;
                }
                count += 1;
            }
        }
        return 0;
    }


    pub fn new() -> Self {
        return Self {
            size: 0,
            hash_values: HashMap::<String, String>::new()
        }
    }
}

impl Decompressor {
    pub fn new() -> Self {
        return Self {
            content: String::new(),
            key_file: KeyFile::new(),
        }
    }

    pub fn read_key(&mut self, decompression_key_path: PathBuf) {
        let mut key_handler = KeyFile::new();
        let content = fs::read_to_string(decompression_key_path);
        key_handler.feed(content.unwrap().as_str());
        self.key_file = key_handler;
    }

    pub fn read_compressed_file(&mut self, compressed_file_path: PathBuf) {
        let content = fs::read_to_string(compressed_file_path);
        self.content = content.unwrap();
    }
}

pub fn decompress_file(compressed_file_path: PathBuf, decompression_key_path: PathBuf) {
    let mut decompressor = Decompressor::new();
    decompressor.read_key(decompression_key_path);
    decompressor.read_compressed_file(compressed_file_path);

    let res = writer::rewrite_file(decompressor.content.as_str(), decompressor.key_file.hash_values);
    if res.is_err() {
        eprintln!("An error occoured when write the decompressed file: {}", res.unwrap_err());
        panic!();
    }
}