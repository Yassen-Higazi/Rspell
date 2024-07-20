mod trie;
mod rspell;

fn main() {
    let path = "./test.txt".to_string();

    let spell_checker = rspell::Rspell::new();

    spell_checker.check_file(path);
}