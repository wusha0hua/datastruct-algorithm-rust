use std::{cell::RefCell, rc::Rc};

use crate::node::TreeNode;
use super::TreeTrait;

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryTree<T: PartialOrd> {
    size: usize,
    root: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Clone + PartialOrd> BinaryTree<T> {
    pub fn from_pre_in_order_traversal(pre_order: &Vec<T>, in_order: &Vec<T>) -> Self {
        let size = pre_order.len();
        Self {
            size,
            root: Self::pre_in(pre_order, in_order, 0, 0, pre_order.len() - 1),
        }
    }
    fn pre_in(pre_order: &Vec<T>, in_order: &Vec<T>, pre_left: usize, in_left: usize, in_right: usize) -> Option<Rc<RefCell<TreeNode<T>>>> {
        if in_left > in_right || pre_left > pre_order.len() {
            return None;
        }
        let mut node = TreeNode::from(pre_order[pre_left].clone());
        let mut index = 0;
        for i in 0..pre_order.len() {
            if in_order[i] == pre_order[pre_left] {
                index = i;
                break;
            }
        }
        if index != 0 {
            node.left = Self::pre_in(pre_order, in_order, pre_left + 1, in_left, index - 1);
            node.right = Self::pre_in(pre_order, in_order, pre_left + 1 + index - in_left, index + 1, in_right);
        }
        Some(Rc::new(RefCell::new(node)))
    }
}

impl<T: PartialOrd + Clone> TreeTrait<T> for BinaryTree<T> {
   fn pre_order_traversal(&self) -> Vec<T> {
       let mut traversal = Vec::new();
       let mut stack = Vec::new();
       if let Some(root) = &self.root {
           stack.push(Rc::clone(root));
           while let Some(node) = stack.pop() {
               traversal.push(node.borrow().data.clone());
               if let Some(right) = &node.borrow().right {
                   stack.push(Rc::clone(right));
               }
               if let Some(left) = &node.borrow().left {
                   stack.push(Rc::clone(left));
               }
           }
       }
    traversal
    }
    fn in_order_traversal(&self) -> Vec<T> {
        let mut traversal = Vec::new();
        let mut stack = Vec::new();
        match &self.root {
            Some(root) => {
                let mut node = Some(Rc::clone(root)); 
                stack.push(Rc::clone(root));
                while stack.len() != 0 {
                    while let Some(n) = &node {
                        if let Some(left) = &Rc::clone(n).borrow().left {
                            node = Some(Rc::clone(left));
                            stack.push(Rc::clone(left));
                        } else {break;}
                    }
                    let node_rc = stack.pop().unwrap();
                    traversal.push(node_rc.borrow().data.clone());
                    if let Some(right) = &Rc::clone(&node_rc).borrow().right {
                        node = Some(Rc::clone(right)); 
                        stack.push(Rc::clone(right));
                    } else {
                        node = None;
                    }
                }
            }
            None => (),
        }
        traversal
    }
    fn post_order_traversal(&self) -> Vec<T> {
        let mut traversal = Vec::new();
        fn recursive_post_order_traversal<T: Clone + PartialOrd>(root: Option<Rc<RefCell<TreeNode<T>>>>, traversal: &mut Vec<T>) {
            match root {
                Some(root) => {
                    recursive_post_order_traversal(root.borrow().left.clone(), traversal);
                    recursive_post_order_traversal(root.borrow().right.clone(), traversal);
                    traversal.push(root.borrow().data.clone());
                }
                None => (),
            } 
        }
        recursive_post_order_traversal(self.root.clone(), &mut traversal);
        traversal
    }
    fn level_order_traversal(&self) -> Vec<Vec<T>> {
        todo!()
    }
    fn depth(&self) -> usize {todo!()}
    fn contains(&self, data: &T) -> bool {todo!()}
    fn size(&self) -> usize {todo!()}
}
