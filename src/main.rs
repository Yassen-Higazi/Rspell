mod trie;
mod rspell;
mod constants;

fn main() {
    let path = "./test.txt".to_string();

    let spell_checker = rspell::Rspell::new();

    spell_checker.check_file(path);

    let word = String::from("mestake");

    let suggestions = spell_checker.get_suggestions(&word);

    println!("Suggestions for \"{word}\" are: {:?}", suggestions)
}