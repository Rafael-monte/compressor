use std::{collections::HashMap, io::{self, ErrorKind}, fs::{File, self}};

use crate::config;

struct RepetitionHandler {
    repetitions: HashMap<String, i32>
}


impl RepetitionHandler {
    pub fn new() -> Self {
        let map: HashMap<String, i32> = HashMap::new();
        return RepetitionHandler { repetitions: map };
    }


    pub fn read_text(&mut self, text: &str) {
        let normalized_text = text.replace("\n", " ");
        let words: Vec<&str> = normalized_text.split(' ').collect();
        for word in words {
            self.add_word(word);
        }
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
        let mut entries: Vec<(String, i32)> = self.repetitions.clone().into_iter().collect();
        entries.sort_by(|f_entry, s_entry| {
            s_entry.1.cmp(&f_entry.1)
        });

        entries.into_iter().for_each(|entry| {
            words_and_aliases.insert(entry.0.clone(), String::from(format!("{}", entry.1)));
        });

        return words_and_aliases;
    }
}


pub fn read_file_and_get_repetitions() {
    let opt_text = get_file_as_string();
    if opt_text.is_err() {
        panic!("Ocorreu um erro ao abrir arquivo!");
    }
    let mut handler = RepetitionHandler::new();
    handler.read_text(opt_text.unwrap().as_str());
    let words_and_markers = handler.drain_repetitions();
    for (word, marker) in words_and_markers.into_iter() {
        println!("[{}] <=> [{}]", word, marker);
    }
}

fn get_file_as_string() -> Result<String, ErrorKind> {
    let opt_file = fs::read_to_string(config::FEED_FILE_PATH);
    if opt_file.is_err() {
        eprintln!("O caminho '{}' não está correto ou não existe!", config::FEED_FILE_PATH);
        return Err(ErrorKind::NotFound);
    }
    return Ok(opt_file.unwrap())
}