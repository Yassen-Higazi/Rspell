use std::fmt::{Debug, Formatter};

use crate::languages::Language;

#[derive(Clone)]
struct TrieNode {
    has_word: bool,

    children: Vec<Option<Box<TrieNode>>>,
}

impl Debug for TrieNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TrieNode[has_word: {}]", self.has_word)
    }
}


impl TrieNode {
    pub fn new() -> Self {
        Self { has_word: false, children: vec![None; 26] }
    }
}

#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
    language: Language,
}

impl Trie {
    pub fn new(language: Language) -> Self {
        Self { root: TrieNode::new(), language }
    }

    pub fn insert(&mut self, word: &String) {
        let normalized_word = Trie::normalize_word(word);

        let mut current_node = &mut self.root;

        for character in normalized_word.chars() {
            let (index, is_overflowing) = (character as usize).overflowing_sub(self.language.start_char as usize);

            if is_overflowing { continue; }

            if current_node.children[index].is_none() {
                current_node.children[index] = Some(Box::new(TrieNode::new()));
            }

            current_node = current_node.children[index].as_deref_mut().unwrap();
        }

        current_node.has_word = true;
    }

    pub fn search(&self, word: &String) -> bool {
        let mut node = &self.root;

        let normalized_word = Trie::normalize_word(word);

        for ch in normalized_word.chars() {
            let (index, is_overflowing) = (ch as usize).overflowing_sub(self.language.start_char as usize);

            if is_overflowing { return node.has_word; }

            if index >= 26 { continue; }

            if let Some(child) = &node.children[index] {
                node = child;
            } else {
                return false;
            }
        }

        return node.has_word;
    }

    fn normalize_word(word: &String) -> String {
        let lower_word = word.to_lowercase();

        return lower_word.trim().to_owned();
    }
}

impl From<Language> for Trie {
    fn from(value: Language) -> Self {
        Trie::new(value)
    }
}