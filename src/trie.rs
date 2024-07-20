use std::fmt::{Debug, Formatter};

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
}

impl Trie {
    pub fn new() -> Self {
        Self { root: TrieNode::new() }
    }


    pub fn insert(&mut self, word: &String) {
        let normalized_word = word.to_lowercase();

        let mut current_node = &mut self.root;

        for character in normalized_word.chars() {
            let (index, is_overflowing) = (character as usize).overflowing_sub('a' as usize);

            if is_overflowing { continue; }

            if current_node.children[index].is_none() {
                current_node.children[index] = Some(Box::new(TrieNode::new()));
            }

            current_node = current_node.children[index].as_deref_mut().unwrap();
        }

        current_node.has_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut node = &self.root;

        let normalized_word = word.to_lowercase();

        for ch in normalized_word.chars() {
            let (index, is_overflowing) = (ch as usize).overflowing_sub('a' as usize);

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
}