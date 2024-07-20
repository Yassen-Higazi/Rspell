use std::fs;
use std::time::Instant;

use levenshtein::levenshtein;

use crate::trie::Trie;

pub struct Rspell {
    trie: Trie,
    dict: Vec<String>,
}

impl Rspell {
    pub fn new() -> Self {
        let mut trie = Trie::new();

        let contents = fs::read_to_string("./dictionary.txt")
            .expect("Could not read the file");

        let split = contents.split("\n").map(|w| { String::from(w) }).collect::<Vec<String>>();

        for word in &split {
            trie.insert(word);
        }

        Self { trie, dict: split }
    }

    pub fn get_suggestions(&self, word: String) -> Vec<String> {
        let mut suggestions: Vec<String> = Vec::new();


        for sug in &self.dict {
            let distance = levenshtein(&word, &sug);

            if distance <= 1 {
                suggestions.push(sug.clone());
            }
        }

        return suggestions;
    }

    pub fn check_file(&self, file_path: String) {
        let now = Instant::now();

        let text = fs::read_to_string(file_path)
            .expect("Could not read the file");

        self.check_text(text);

        let elapsed_time = now.elapsed();

        println!("Running check_file() took {} Milliseconds.", elapsed_time.as_millis());
    }


    pub fn check_text(&self, text: String) {
        for (line_number, line) in text.split("\n").enumerate() {
            for (word_number, word) in line.split_whitespace().enumerate() {
                let found = self.trie.search(word.to_string());


                if !found {
                    println!("Spelling mistake at \"{}\" [line: {}, word: {}]", word, line_number + 1, word_number + 1);
                }
            }
        }
    }
}