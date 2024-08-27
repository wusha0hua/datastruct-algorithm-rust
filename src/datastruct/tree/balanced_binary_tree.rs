use crate::node::{BalancedTreeNode, TreeNode};
use crate::datastruct::tree::TreeTrait;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct BalancedBinaryTree<T: PartialOrd> {
    pub size: usize,
    pub root: Option<Rc<RefCell<BalancedTreeNode<T>>>>,
}

impl<T: PartialOrd + Clone> TreeTrait<T> for BalancedBinaryTree<T> {
   fn pre_order_traversal(&self) -> Vec<T> {
       todo!()
   }
   fn in_order_traversal(&self) -> Vec<T> {
       todo!()
   }
   fn post_order_traversal(&self) -> Vec<T> {
       todo!()
   }
   fn level_order_traversal(&self) -> Vec<Vec<T>> {
       todo!()
   }
   fn depth(&self) -> usize {
       todo!()
   }
   fn contains(&self, data: &T) -> bool {
       todo!()
   }
   fn size(&self) -> usize {
       todo!()
   }
}
impl<T: PartialOrd> PartialEq for BalancedBinaryTree<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {return false;}
        match (&self.root, &other.root) {
            (Some(root1), Some(root2)) => root1 == root2,
            (None, None) => true,
            _ => false,
        }
    }
}

impl<T: PartialOrd + std::fmt::Debug> BalancedBinaryTree<T> {
    pub fn new() -> Self {
        Self { 
            size: 0, 
            root: None 
        }
    }
    pub fn append(&mut self, data: T) {
        let (root, _) = Self::recurisve_append(self.root.take(), data);
        self.root = Some(root);
        self.size += 1;
    }
    pub fn delete(&mut self, data: &T) -> Option<T> {
        match self.root.take() {
            Some(root) => {
                let mut remove = None;
                self.root = Self::recursive_delete(root, data, &mut remove);
                match remove.take() {
                    Some(remove) => {
                        self.size -= 1;
                        Some(remove)
                    }
                    None => None,
                }
            }
            None => None, 
        }
    }
    fn recurisve_append(root: Option<Rc<RefCell<BalancedTreeNode<T>>>>, data: T) -> (Rc<RefCell<BalancedTreeNode<T>>>, bool) {
        match root {
            Some(mut root) => if root.borrow().data > data {
                let (mut left, is_xl) = Self::recurisve_append(root.borrow_mut().left.take(), data);
                let left_height = left.borrow().height;
                let right_height = Self::get_right_height(&root);
                if left_height - right_height == 2 {
                    if is_xl {
                        root.borrow_mut().left = Some(left);
                        root = Self::right_rorate(root);
                    } else {
                        left = Self::left_rorate(left);
                        root.borrow_mut().left = Some(left);
                        root = Self::right_rorate(root);
                    }
                } else {
                    root.borrow_mut().left = Some(left);
                    let (left_height, right_height) = Self::get_left_right_height(&root);
                    root.borrow_mut().height = left_height.max(right_height) + 1;
                }
                (root, true)
            } else {
                let (mut right, is_xl) = Self::recurisve_append(root.borrow_mut().right.take(), data);
                let right_height = right.borrow().height;
                let left_height = Self::get_left_height(&root);
                if right_height - left_height == 2 {
                    if is_xl {
                        right = Self::right_rorate(right);
                        root.borrow_mut().right = Some(right);
                        root = Self::left_rorate(root);
                    } else {
                        root.borrow_mut().right  = Some(right);
                        root = Self::left_rorate(root); 
                    }
                } else {
                    root.borrow_mut().right = Some(right);
                    let (left_height, right_height) = Self::get_left_right_height(&root);
                    root.borrow_mut().height = left_height.max(right_height) + 1;
                }
                (root, false)
            }
            None => (Rc::new(RefCell::new(BalancedTreeNode::from(data))), true), 
        }
    }
    fn recursive_delete(root: Rc<RefCell<BalancedTreeNode<T>>>, data: &T, remove: &mut Option<T>) -> Option<Rc<RefCell<BalancedTreeNode<T>>>> {
       if root.borrow().data == *data {
           let root = Rc::try_unwrap(root).ok().unwrap().into_inner();
           let (replace, remove_data) = Self::delete_node(root);
           *remove = Some(remove_data);
           return replace;
       } else if root.borrow().data > *data {
           let left = match root.borrow_mut().left.take() {
               Some(left) => Self::recursive_delete(left, data, remove),
               None => None,
           };
           root.borrow_mut().left = left;
       } else {
           let right = match root.borrow_mut().right.take() {
               Some(right) => Self::recursive_delete(right, data, remove),
               None => None,
           };
           root.borrow_mut().right = right;
       }
       Self::update_height(&root);
       let root = Self::update_balance(root);
       Some(root)
    }
    fn delete_node(to_be_remove: BalancedTreeNode<T>) -> (Option<Rc<RefCell<BalancedTreeNode<T>>>>, T) {
        let data = to_be_remove.data;
        match (to_be_remove.left, to_be_remove.right) {
            (Some(left), Some(right)) => {
                let (_, replace) = Self::find_replace_node(Rc::clone(&left));
                replace.borrow_mut().right = Some(right);
                if Rc::ptr_eq(&left, &replace) == false {
                    replace.borrow_mut().left = Some(left);
                } else {
                    replace.borrow_mut().left = None;
                }
                Self::update_height(&replace);
                let replace = Self::update_balance(replace);
                (Some(replace), data)
            }
            (Some(left), None) => (Some(left), data),
            (None, Some(right)) => (Some(right), data),
            (None, None) => (None, data),
        }
    }
    fn find_replace_node(node: Rc<RefCell<BalancedTreeNode<T>>>) -> (Option<Rc<RefCell<BalancedTreeNode<T>>>>, Rc<RefCell<BalancedTreeNode<T>>>) {
        let r = node.borrow_mut().right.take();
        let l = node.borrow_mut().left.take();
        let ((r, replace), l) = match (r, l) {
            (Some(right), Some(left)) => (Self::find_replace_node(right), Some(left)),
            (Some(right), None) => (Self::find_replace_node(right), None),
            (None, Some(left)) => return (Some(left), Rc::clone(&node)),
            (None, None) => return (None, Rc::clone(&node)), 
        };
        node.borrow_mut().right = r;
        node.borrow_mut().left = l;
        Self::update_height(&node);
        let node = Self::update_balance(node);
        (Some(node), replace)
    }
    fn get_left_right_height(root: &Rc<RefCell<BalancedTreeNode<T>>>) -> (usize, usize) {
        match (root.borrow().left.as_ref(), root.borrow().right.as_ref()) {
            (Some(left), Some(right)) => (left.borrow().height, right.borrow().height),
            (Some(left), None) => (left.borrow().height, 0),
            (None, Some(right)) => (0, right.borrow().height),
            (None, None) => (0, 0),
        }
    }
    fn get_left_height(root: &Rc<RefCell<BalancedTreeNode<T>>>) -> usize {
        match root.borrow().left.as_ref() {
            Some(left) => left.borrow().height,
            None => 0,
        }
    }
    fn get_right_height(root: &Rc<RefCell<BalancedTreeNode<T>>>) -> usize {
        match root.borrow().right.as_ref() {
            Some(right) => right.borrow().height,
            None => 0,
        }
    }
    fn update_height(root: &Rc<RefCell<BalancedTreeNode<T>>>) {
        let (left_height, right_height) = Self::get_left_right_height(root);
        root.borrow_mut().height = left_height.max(right_height) + 1;
    }
    fn update_balance(root: Rc<RefCell<BalancedTreeNode<T>>>) -> Rc<RefCell<BalancedTreeNode<T>>> {
        let (root_left_height, root_right_height) = Self::get_left_right_height(&root); 
        let left = root.borrow_mut().left.take();
        let right = root.borrow_mut().right.take();
        if root_left_height > root_right_height && root_left_height - root_right_height == 2 {
            let left = match left {
                Some(left) => left,
                None => unreachable!("update balance: get left node error"),
            };
            let (left_left_height, left_right_height) = Self::get_left_right_height(&left); 
            if left_left_height > left_right_height {
                // ll
                root.borrow_mut().left = Some(left);
                root.borrow_mut().right = right;
                return Self::right_rorate(root);
            } else {
                //lr
                let left = Self::left_rorate(left);
                root.borrow_mut().left = Some(left);
                root.borrow_mut().right = right;
                return Self::right_rorate(root);
            }
        } else if root_right_height > root_left_height && root_right_height - root_left_height == 2 {
            let right = match right {
                Some(right) => right,
                None => unreachable!("update balance: get right node error"),
            };
            let (right_left_height, right_right_height) = Self::get_left_right_height(&right);
            if right_left_height > right_right_height {
                // rl
                let right = Self::right_rorate(right);
                root.borrow_mut().left = left;
                root.borrow_mut().right = Some(right);
                return Self::left_rorate(root);
            } else {
                // rr
                root.borrow_mut().left = left;
                root.borrow_mut().right = Some(right);
                return Self::left_rorate(root);
            }
        }
        root.borrow_mut().left = left;
        root.borrow_mut().right = right;
        root
    }
    fn right_rorate(root: Rc<RefCell<BalancedTreeNode<T>>>) -> Rc<RefCell<BalancedTreeNode<T>>> {
        let root_left = root.borrow_mut().left.take();
        match root_left {
            Some(left) => {
                let left_right = left.borrow_mut().right.take();
                root.borrow_mut().left= left_right; 
                let (left_height, right_height) = Self::get_left_right_height(&root);
                root.borrow_mut().height = left_height.max(right_height) + 1;
                left.borrow_mut().right = Some(root);
                let (left_height, right_height) = Self::get_left_right_height(&left);
                left.borrow_mut().height = left_height.max(right_height) + 1;
                left
            }
            None => unreachable!("right rorate error"),
        }
    }
    fn left_rorate(root: Rc<RefCell<BalancedTreeNode<T>>>) -> Rc<RefCell<BalancedTreeNode<T>>> {
        let root_right = root.borrow_mut().right.take();
        match root_right {
            Some(right) => {
                let right_left = right.borrow_mut().left.take();
                root.borrow_mut().right = right_left;
                let (left_height, right_height) = Self::get_left_right_height(&root);
                root.borrow_mut().height = left_height.max(right_height) + 1;               
                right.borrow_mut().left = Some(root);
                let (left_height, right_height) = Self::get_left_right_height(&right);
                right.borrow_mut().height = left_height.max(right_height) + 1;
                right
            }
            None => unreachable!("left rorate error"),
        }
    }
}
