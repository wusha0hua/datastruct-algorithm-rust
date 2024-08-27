use std::collections::HashMap;
use std::hash::Hash;
use std::{cmp::PartialEq, rc::Rc};
use std::cell::RefCell;


#[derive(Debug, Clone)]
pub struct Node<T: PartialEq> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

#[derive(Debug, Clone)]
pub struct DoublyNode<T: PartialEq> {
    pub data: T,
    pub next: Option<Rc<RefCell<DoublyNode<T>>>>,
    pub prev: Option<Rc<RefCell<DoublyNode<T>>>>,
}

#[derive(Debug, Clone)]
pub struct TreeNode<T: PartialOrd> {
    pub data: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

#[derive(Debug, Clone)]
pub struct BalancedTreeNode<T: PartialOrd> {
    pub data: T,
    pub height: usize,
    pub left: Option<Rc<RefCell<BalancedTreeNode<T>>>>,
    pub right: Option<Rc<RefCell<BalancedTreeNode<T>>>>,   
}

#[derive(Debug, Clone)]
pub struct BTreeNode<T: PartialOrd + Default> {
    pub keys: Vec<T>,
    pub key_count: usize,
    pub next: Vec<Option<Box<BTreeNode<T>>>>,
}

#[derive(Debug, Clone)]
pub struct TireNode<T: Eq + Hash> {
    pub count: usize,
    pub next: HashMap<T, Box<TireNode<T>>>,
}

#[derive(Debug, Clone)]
pub struct GraphNode {
    pub id: usize,
}

impl<T: PartialEq> Node<T> {
    pub fn from(data: T) -> Self {
        Node {
            data,
            next: None,
        }
    }
    pub fn push(&mut self, data: T) {
        match &mut self.next {
            Some(node) => node.push(data),
            None => self.next = Some(Box::new(Node::from(data))),
        } 
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.data != other.data {
            return false;
        }
        match &self.next {
            Some(node1) => {
                match &other.next {
                    Some(node2) => node1.eq(node2), 
                    None => false,
                }
            } 
            None => {
                if let None = other.next {true}
                else {false}
            }
        }
    }
}

impl<T: PartialEq> DoublyNode<T> {
    pub fn from(data: T) -> DoublyNode<T> {
        DoublyNode { 
            data, 
            next: None, 
            prev: None,
        }
    }
}

impl<T: PartialEq> PartialEq for DoublyNode<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.data != other.data {
            return false;
        }
        match &self.next {
            Some(node1) => {
                match &other.next {
                    Some(node2) => node1.eq(node2), 
                    None => false,
                }
            } 
            None => {
                if let None = other.next {true}
                else {false}
            }
        }
    }
}

impl<T: PartialOrd> TreeNode<T> {
    pub fn from(data: T) -> Self {
        Self { 
            data, 
            left: None, 
            right: None 
        }
    }
}

impl<T: PartialOrd> PartialEq for TreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
         if self.data != other.data {return false;}
         match self {
            TreeNode {data: _, left: Some(left1), right: Some(right1)} => 
                match other {
                    TreeNode {data: _, left: Some(left2), right: Some(right2)} => {
                        left1.eq(left2) | right1.eq(right2)                     
                    }
                    _ => false,
                },
            TreeNode {data: _, left: None, right: Some(right1)} => 
                match other {
                    TreeNode {data: _, left: None, right: Some(right2)} => {
                        right1.eq(right2)
                    }
                    _ => false,
                },
            TreeNode {data: _, left: Some(left1), right: None} => 
                match other {
                    TreeNode {data: _, left: Some(left2), right: None} => {
                        left1.eq(left2)
                    }
                    _ => false,
                },
            TreeNode {data: _, left: None, right: None} =>
                match other {
                    TreeNode {data: _, left: None, right: None} => {
                        true
                    }
                    _ => false,
                },
        }
     
    }
}

impl<T: PartialOrd> BalancedTreeNode<T> {
    pub fn from(data: T) -> Self {
        Self { 
            data, 
            height: 1,
            left: None, 
            right: None 
        }
    }
}

impl<T: PartialOrd> PartialEq for BalancedTreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
         if self.data != other.data {return false;}
         if self.height != other.height {return false;}
         match self {
            BalancedTreeNode {data: _, height: _, left: Some(left1), right: Some(right1)} => 
                match other {
                    BalancedTreeNode {data: _, height: _, left: Some(left2), right: Some(right2)} => {
                        left1.eq(left2) & right1.eq(right2)                     
                    }
                    _ => false,
                },
            BalancedTreeNode {data: _, height: _, left: None, right: Some(right1)} => 
                match other {
                    BalancedTreeNode {data: _, height: _, left: None, right: Some(right2)} => {
                        right1.eq(right2)
                    }
                    _ => false,
                },
            BalancedTreeNode {data: _, height: _, left: Some(left1), right: None} => 
                match other {
                    BalancedTreeNode {data: _, height: _, left: Some(left2), right: None} => {
                        left1.eq(left2)
                    }
                    _ => false,
                },
            BalancedTreeNode {data: _, height: _, left: None, right: None} =>
                match other {
                    BalancedTreeNode {data: _, height: _, left: None, right: None} => {
                        true
                    }
                    _ => false,
                },
        }
     
    }
}

impl<T: PartialOrd + Default + Clone> BTreeNode<T> {
    pub fn new() -> Self {
        Self { 
            keys: Vec::new(), 
            key_count: 0, 
            next: Vec::new(), 
        }
    }
    pub fn from(degree: usize, data: T) -> Self {
        let mut keys = vec![T::default(); degree];
        keys[0] = data;
        Self {
            keys,
            key_count: 1,
            next: vec![None; degree + 1],
        }
    }
}

impl<T: PartialOrd + Default> PartialEq for BTreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.key_count != other.key_count {
            return false;
        }
        for i in 0..self.key_count {
            if self.keys[i] != other.keys[i] {
                return false;
            }
        }
        for i in 0..self.key_count + 1 {
            if self.next[i] != other.next[i] {
                return false;
            }
        }
        true
    }
}

impl<T: Eq + Hash> PartialEq for TireNode<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.count != other.count {return false;}
        if self.next.len() != other.next.len() {return false;}
        let mut flag = true;
        for (k, v1) in self.next.iter() {
            match other.next.get(k) {
                Some(v2) => if !v1.eq(v2) {flag = false}
                None => flag = false,
            } 
        }
        flag
    }
}

impl<T: Eq + Hash> TireNode<T> {
    pub fn new() -> Self {
        Self {
            count: 0,
            next: HashMap::new(),
        }
    }
}
