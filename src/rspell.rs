use std::fs;
use std::time::Instant;

use levenshtein::levenshtein;

use crate::constants::DICTIONARIES;
use crate::trie::Trie;

pub struct Rspell {
    trie: Trie,
    dict: Vec<String>,
}

// TODO: Handle Names
impl Rspell {
    pub fn new() -> Self {
        let mut trie = Trie::new();

        let dict = Self::load_dictionaries();

        for word in &dict {
            trie.insert(word);
        }

        Self { trie, dict }
    }

    fn load_dictionaries() -> Vec<String> {
        let mut splits: Vec<String> = Vec::new();

        for dict in DICTIONARIES {
            let contents = fs::read_to_string(dict)
                .expect("Could not read the file");

            let mut split = contents.split("\n").map(|w| { String::from(w) }).collect::<Vec<String>>();

            splits.append(&mut split);
        }

        splits
    }

    // TODO: find better way for suggestions
    pub fn get_suggestions(&self, word: &String) -> Vec<String> {
        let mut suggestions: Vec<String> = Vec::new();

        let found = self.trie.search(word);

        if found {
            suggestions.push(word.clone());

            return suggestions;
        };

        for sug in &self.dict {
            let distance = levenshtein(&word, &sug);

            if distance <= 1 {
                suggestions.push(sug.clone());
            }
        }

        suggestions
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
        let mut word_count = 0;
        let mut typos_count = 0;

        for (line_number, line) in text.split("\n").enumerate() {
            for (word_number, word) in line.split_whitespace().enumerate() {
                word_count += 1;

                let found = self.trie.search(&word.to_string());


                if !found {
                    typos_count += 1;
                    println!("Spelling mistake at \"{}\" [line: {}, word: {}]", word, line_number + 1, word_number + 1);
                }
            }
        }

        println!("Number of words: {word_count}, Number of Spelling mistakes: {typos_count}");
    }
}