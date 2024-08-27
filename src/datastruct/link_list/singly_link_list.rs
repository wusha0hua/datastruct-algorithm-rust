use crate::node::Node;
use crate::y_combinator::y;
pub use crate::datastruct::link_list::LinkListTrait;

use std::cmp::PartialEq;

#[derive(Debug, Clone)]
pub struct SinglyLinkList<T: PartialEq> {
    pub len: usize,
    pub head: Option<Box<Node<T>>>,
    pub is_recursive: bool,
}

impl<T: PartialEq> LinkListTrait<T> for SinglyLinkList<T> {
    fn is_empty(&self) -> bool {
        self.len == 0 
    }
    fn len(&self) -> usize {
        self.len
    }
    fn insert(&mut self, index: usize, data: T) -> bool {
        if self.is_recursive {
            fn recursive_insert<T: PartialEq>(node: &mut Option<Box<Node<T>>>, index: usize, data: T, current: usize) -> bool {
                if current == index {
                    let mut new_node = Box::new(Node::from(data));
                    new_node.next = node.take();
                    *node = Some(new_node);
                    true
                } else {
                    match node {
                        Some(node) => {
                            recursive_insert(&mut node.next, index, data, current + 1)                        
                        } 
                        None => false,
                    }
                }
            } 
            if recursive_insert(&mut self.head, index, data, 0) {
                self.len += 1;
                true
            } else {
                false
            }
        } else {
            let mut count = 0;
            let mut cur = &mut self.head;
            while count < index {
                match cur {
                   Some(node) => cur = &mut node.next,
                   None => return false,
                }
                count += 1
            }
            *cur = Some(Box::new(Node {data, next: cur.take()}));
            self.len += 1;
            true
        }
    }
    fn remove(&mut self, index: usize) -> Option<T> {
        if self.is_recursive {
            fn recursive_remove<T: PartialEq>(node: &mut Option<Box<Node<T>>>, index: usize, current: usize) -> Option<T> {
                if current == index {
                    match node {
                        Some(n) => {
                            let next_node = n.next.take();
                            let remove_node = node.take();
                            *node = next_node;
                            Some(remove_node.unwrap().data)
                        }
                        None => None,
                    }
                } else {
                    match node {
                        Some(node) => recursive_remove(&mut node.next, index, current + 1),
                        None => None,
                    }
                }
            }
            if let Some(remove_node) = recursive_remove(&mut self.head, index, 0) {
                self.len -= 1;
                Some(remove_node)
            } else {
                None
            }
        } else {
            let mut cur = &mut self.head; 
            let mut count = 0;
            while count < index {
                match cur {
                    Some(node) => cur = &mut node.next,
                    None => return None,
                }
                count += 1;
            }
            match cur {
                Some(cur_node) => {
                    let next_node = cur_node.next.take();
                    let remove_node = cur.take();
                    *cur = next_node;
                    self.len -= 1;
                    Some(remove_node.unwrap().data)
                }
                None => None,
            }
        }
    }
    fn contains(&self, target: &T) -> Option<usize> {
        if self.is_recursive {
            fn recursive_contains<T: PartialEq>(node: &Option<Box<Node<T>>>, target: &T, index: usize) -> Option<usize> {
                match node {
                    Some(node) => {
                        if node.data == *target {
                            Some(index)
                        } else {
                            recursive_contains(&node.next, target, index + 1)
                        }
                    }
                    None => None,
                }
            }
            recursive_contains(&self.head, target, 0)
        } else {
            let mut cur = &self.head;
            let mut count = 0;
            while let Some(node) = cur {
                if node.data == *target {
                    return Some(count);
                }
                cur = &node.next;
                count += 1;
            }
            None
        }
    }
    fn clear(&mut self) {
        while let Some(_) = self.remove(0) {}
    }
}

impl<T: PartialEq> SinglyLinkList<T> {
    pub fn new() -> Self {
        SinglyLinkList {
            len: 0,
            head: None,
            is_recursive: false,
        }
    }
    pub fn enable_recursive_algorithm(&mut self) {self.is_recursive = true;}
    pub fn disable_recursive_algorithm(&mut self) {self.is_recursive = false;}
}

impl<T: PartialEq> PartialEq for SinglyLinkList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            false
        }
        else {
            match &self.head {
                Some(node1) => {
                    if let Some(node2) = &other.head {
                        *node2 == *node1
                    } else {
                        false
                    } 
                } 
                None => {
                    if let None = other.head {
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }
}
