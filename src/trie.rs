use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone)]
struct TrieNode {
    has_word: bool,

    children: HashMap<String, TrieNode>,
}

impl Debug for TrieNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TrieNode[has_word: {}]", self.has_word)
    }
}

impl TrieNode {
    pub fn new() -> Self {
        Self { has_word: false, children: HashMap::new() }
    }
}

#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
}


impl Trie {
    pub fn new() -> Self {
        Self { root: TrieNode::new() }
    }

    pub fn insert(&mut self, word: &String) {
        let mut current_node = &mut self.root;

        let normalized_word = Trie::normalize_word(word);

        for grapheme in UnicodeSegmentation::graphemes(normalized_word.as_str(), true) {
            if !current_node.children.contains_key(grapheme) {
                current_node.children.insert(grapheme.to_string(), TrieNode::new());
            }

            current_node = current_node.children.get_mut(grapheme).unwrap();
        }

        current_node.has_word = true;
    }

    pub fn search(&self, word: &String) -> bool {
        let mut node = &self.root;

        let normalized_word = Trie::normalize_word(word);

        for grapheme in UnicodeSegmentation::graphemes(normalized_word.as_str(), true) {
            if node.children.contains_key(grapheme) {
                node = &node.children[grapheme];
            } else {
                return false;
            }
        }

        node.has_word
    }

    fn normalize_word(word: &String) -> String {
        if word.is_empty() { return word.to_owned(); }

        let lower_word = word.to_lowercase();

        let mut word = lower_word.trim().to_string();

        word = Trie::remove_punctuation(&word);

        // TODO: handle words like (I'll, I,ve, ...etc)

        word.to_owned()
    }

    fn remove_punctuation(word: &String) -> String {
        let new_word: String = word.chars().filter(|&c| !c.is_ascii_punctuation()).collect();

        new_word
    }
}