use std::collections::HashMap;

use crate::node::TireNode;

#[derive(Debug, Clone)]
pub struct Trie {
    pub words_count: usize,
    pub next: HashMap<char, Box<TireNode<char>>>,
}

impl PartialEq for Trie {
    fn eq(&self, other: &Self) -> bool {
        if self.words_count != other.words_count {return false;} 
        self.next == other.next
    }
}

impl Trie {
    pub fn new() -> Self {
        Self {
            words_count: 0,
            next: HashMap::new(),
        }
    }
    pub fn insert_word(&mut self, word: &str) {
        let mut word = word.chars();
        let mut node: &mut Box<TireNode<char>> = match word.next() {
            Some(first_char) => match self.next.get_mut(&first_char) {
                Some(node) => node,
                None => {
                    self.next.insert(first_char, Box::new(TireNode::new()));
                    self.next.get_mut(&first_char).expect("trie tree create new node error")
                }
            }
            None => return,
        };
        for c in word {
            if node.next.contains_key(&c) {
                node = node.next.get_mut(&c).expect("tire tree get next node error");
            } else {
                node.next.insert(c, Box::new(TireNode::new()));
                node = node.next.get_mut(&c).expect("tire tree get next node error");
            }
        }
        node.count += 1;
        self.words_count += 1;
    }
    pub fn delete_word(&mut self, word: &str) -> bool {
        use std::str::Chars;
        let mut word = word.chars();
        fn recursive_delete_word(node: &mut Box<TireNode<char>>, mut chars: Chars) -> bool {
            match chars.next() {
                Some(c) => match node.next.get_mut(&c) {
                    Some(next) => {
                        let is_delete = recursive_delete_word(next, chars);
                        if is_delete {
                            if next.count == 0 && next.next.len() == 0 {
                                node.next.remove(&c);
                            }                    
                        } 
                        is_delete
                    }
                    None => false,
                },
                None => {
                    if node.count == 0 {
                        false
                    } else {
                        node.count -= 1;
                        true
                    }
                }
            }
        }
        match word.next() {
            Some(first_char) => {
                match self.next.get_mut(&first_char) {
                    Some(first_node) => {
                        let is_delete = recursive_delete_word(first_node, word);
                        if is_delete {
                            if first_node.count == 0 && first_node.next.len() == 0 {
                                self.next.remove(&first_char);
                            }
                            self.words_count -= 1;
                        }
                        return is_delete;
                    }
                    None => return false,
                }
            }
            None => return true,
        }
    }
    pub fn query_word(&self, word: &str) -> bool {
        let mut word = word.chars();
        match word.next() {
            Some(first_char) => {
                match self.next.get(&first_char) {
                    Some(first_node) => {
                        let mut node = first_node;
                        for c in word {
                            match node.next.get(&c) {
                                Some(next) => {
                                    node = next; 
                                } 
                                None => return false,
                            }
                        }
                        if node.count == 0 {
                            false
                        } else {
                            true
                        }
                    }
                    None => false,
                }
            }
            None => true,
        }
    }
    pub fn query_prefix(&self, prefix: &str) -> Vec<String> {
        let mut result = Vec::new();
        let prefix_str = prefix.to_string();
        let mut prefix_char = prefix.chars();
        match prefix_char.next() {
            Some(first_char) => match self.next.get(&first_char) {
                Some(first_node) => {
                    let mut node = first_node;
                    for c in prefix_char {
                        match node.next.get(&c) {
                            Some(next) => node = next, 
                            None => return result,
                        }
                    }
                    fn recursive_match_words(node: &Box<TireNode<char>>, prefix: String, result: &mut Vec<String>) {
                        for (c, next) in node.next.iter() {
                            let mut new_prefix = prefix.clone();
                            new_prefix.push(*c);
                            recursive_match_words(next, new_prefix, result);
                        }        
                        if node.count > 0 {
                            result.push(prefix);
                        }
                    }
                    recursive_match_words(node, prefix_str, &mut result);
                }
                None => return result,
            }
            None => return result,
        }
        result
    }
}


