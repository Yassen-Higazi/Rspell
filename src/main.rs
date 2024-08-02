use crate::languages::Languages;

mod trie;
mod rspell;
mod languages;

fn main() {
    let path = "./test.txt".to_string();

    let spell_checker = rspell::Rspell::from(Languages::EN);

    spell_checker.check_file(path);

    let word = String::from("mestake");

    let suggestions = spell_checker.get_suggestions(&word);

    println!("Suggestions for \"{word}\" are: {:?}", suggestions)
}