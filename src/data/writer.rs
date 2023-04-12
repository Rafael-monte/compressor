use std::{io::ErrorKind, collections::HashMap};

pub fn write_compressed_file(words: Vec<String>, words_and_repetitions: HashMap<String, String>) -> Result<(), ErrorKind> {
    let mut compressed_text = String::new();
    for word in words {
        let marker = find_marker_for_word(word.as_str(), &words_and_repetitions);
        compressed_text.push_str(marker.as_str());

    }

    println!("{}", &compressed_text);

    return Ok(());
}



fn find_marker_for_word(word: &str, words_and_repetitions: &HashMap<String, String>) -> String {
    let marker = words_and_repetitions.get(&String::from(word));
    return marker.unwrap().clone()
} 