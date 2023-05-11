use std::{collections::HashMap, io::{ErrorKind}, fs::{self}, path::PathBuf, iter::Enumerate};

use crate::config;

use super::writer;

struct RepetitionHandler {
    repetitions: HashMap<String, i32>,
    words: Vec<String>,
}


impl RepetitionHandler {
    pub fn new() -> Self {
        let map: HashMap<String, i32> = HashMap::new();
        return RepetitionHandler { repetitions: map, words: Vec::new() };
    }

    pub fn read_text(&mut self, text: &str) {
        let file_text = String::from(text);
        let mut words: Vec<String> = self.normalize_text(file_text.as_str());
        words.retain(|el|{el != ""});
        for word in &words {
            self.add_word(word.as_str());
        }
        self.words = words;
    }

    fn normalize_text(&mut self, plain_text: &str) -> Vec<String> {
        let mut normalized_text = Vec::<String>::new();
        for el in plain_text.split(config::WHITESPACE).clone().into_iter(){
            if el.contains(config::BREAK_LINE) {
                let lb_idx: usize = el.find(config::BREAK_LINE).unwrap();
                let l_word: &str = &el[..lb_idx];
                let r_word: &str = &el[lb_idx+1..];
                normalized_text.push(String::from(l_word));
                normalized_text.push(config::BREAK_LINE_MARKER.to_owned());
                normalized_text.push(String::from(r_word));
                continue;
            }
            normalized_text.push(String::from(el));
        }
        return normalized_text;
    }

    fn add_word(&mut self, word: &str) {
        if !self.repetitions.contains_key(&String::from(word)) {
            self.repetitions.insert(String::from(word), 1);
            return;
        }
        let repetitions = self.repetitions.get(&String::from(word)).unwrap();
        self.repetitions.insert(String::from(word), repetitions+1);
    }

    pub fn drain_repetitions(&mut self) -> HashMap<String, String> {
        let mut words_and_aliases: HashMap<String, String> = HashMap::new();
        let entries: Vec<(String, i32)> = self.repetitions.clone().into_iter().collect();
        let mut count: usize = 0;
        for (key, _) in entries {
            
            words_and_aliases.insert(key.clone(), format!("{}", &count));
            count += 1;
        }
        return words_and_aliases;
    }
}


pub fn compress_text(path: PathBuf) {
    let opt_text = get_file_as_string(&path);
    if opt_text.is_err() {
        panic!("Ocorreu um erro ao abrir arquivo!");
    }
    let mut handler = RepetitionHandler::new();
    handler.read_text(opt_text.unwrap().as_str());
    let words_and_markers = handler.drain_repetitions();

    let compression_result = writer::write_compressed_file(handler.words, &words_and_markers);
    if compression_result.is_err() {
        panic!("Ocorreu um erro ao comprimir arquivo");
    }

    let key_creation_result = writer::write_key_file(&words_and_markers, None);

    if key_creation_result.is_err() {
        panic!("Ocorreu um erro ao gerar arquivo de chaves");
    }
}

fn get_file_as_string(path: &PathBuf) -> Result<String, ErrorKind> {
    let opt_file = fs::read_to_string(path);
    if opt_file.is_err() {
        eprintln!("O caminho '{}' não está correto ou não existe!", path.display());
        return Err(ErrorKind::NotFound);
    }
    return Ok(opt_file.unwrap())
}