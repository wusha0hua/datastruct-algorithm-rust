use crate::node::TreeNode;
use crate::datastruct::tree::TreeTrait;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct BinarySearchTree<T: PartialOrd> {
    pub size: usize,
    pub root: Option<Rc<RefCell<TreeNode<T>>>>,
    pub is_recursive: bool,
}

impl<T: PartialOrd + Clone> TreeTrait<T> for BinarySearchTree<T> {
   fn pre_order_traversal(&self) -> Vec<T> {
        let mut traversal = Vec::new();
        if self.is_recursive {
            fn recursive_pre_order_traversal<T: Clone + PartialOrd>(root: Option<Rc<RefCell<TreeNode<T>>>>, traversal: &mut Vec<T>) {
                match root {
                    Some(root) => {
                        traversal.push(root.borrow().data.clone());
                        recursive_pre_order_traversal(root.borrow().left.clone(), traversal);
                        recursive_pre_order_traversal(root.borrow().right.clone(), traversal);
                    }
                    None => (),
                } 
            }
            recursive_pre_order_traversal(self.root.clone(), &mut traversal);
        } else {
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
        }
        traversal
   }
   fn in_order_traversal(&self) -> Vec<T> {
        let mut traversal = Vec::new();
        if self.is_recursive {
            fn recursive_in_order_traversal<T: Clone + PartialOrd>(root: Option<Rc<RefCell<TreeNode<T>>>>, traversal: &mut Vec<T>) {
                match root {
                    Some(root) => {
                        recursive_in_order_traversal(root.borrow().left.clone(), traversal);
                        traversal.push(root.borrow().data.clone());
                        recursive_in_order_traversal(root.borrow().right.clone(), traversal);
                    }
                    None => (),
                } 
            }
            recursive_in_order_traversal(self.root.clone(), &mut traversal);
        } else {
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
        }
        traversal
   }
   fn post_order_traversal(&self) -> Vec<T> {
        let mut traversal = Vec::new();
        if self.is_recursive {
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

        } else {
            let mut stack = Vec::<Rc<RefCell<TreeNode<T>>>>::new();
            match &self.root {
                Some(root) => {
                    let mut node = Some(Rc::clone(root));
                    let mut left_prev_stack = Vec::<Rc<RefCell<TreeNode<T>>>>::new();
                    let mut right_prev_stack = Vec::<Rc<RefCell<TreeNode<T>>>>::new();
                    stack.push(Rc::clone(root));
                    while !stack.is_empty() {
                        while let Some(node_rc) = node.clone() {
                            if let Some(left) = &node_rc.borrow().left {
                                match left_prev_stack.last() {
                                    Some(last) => {
                                        if last != left {
                                            node = Some(Rc::clone(left));   
                                            stack.push(Rc::clone(left));
                                            left_prev_stack.push(Rc::clone(left));
                                            continue;
                                        }
                                    }
                                    None => {
                                        node = Some(Rc::clone(left));   
                                        stack.push(Rc::clone(left));
                                        left_prev_stack.push(Rc::clone(left));
                                        continue;
                                    }
                                }
                            }
                            match &node_rc.borrow().right {
                                Some(right) => {
                                    match right_prev_stack.last() {
                                        Some(last) => {
                                            if last == right {
                                                node = None;
                                                continue; // equal to break
                                            } else {
                                                node = Some(Rc::clone(right));
                                                stack.push(Rc::clone(right));
                                                right_prev_stack.push(Rc::clone(right));
                                            }
                                        } 
                                        None => {
                                            node = Some(Rc::clone(right));
                                            stack.push(Rc::clone(right));
                                            right_prev_stack.push(Rc::clone(right));
                                        }
                                    }
                                } 
                                None => {
                                    node = None;
                                }
                            }
                        }
                        let last_rc = stack.pop().unwrap();
                        traversal.push(last_rc.borrow().data.clone());
                        match left_prev_stack.pop() {
                            Some(last) => {
                                if !matches!(&last_rc.borrow().left, Some(_)) {
                                     left_prev_stack.push(last);
                                }
                            } 
                            None => (),
                        }
                        match right_prev_stack.pop() {
                            Some(last) => {
                                if !matches!(&last_rc.borrow().right, Some(_)) {
                                     right_prev_stack.push(last);
                                }
                            } 
                            None => (),
                        }

                        if let Some(last_rc) = stack.last() {
                            node = Some(Rc::clone(&last_rc));
                        } else {
                            node = None;
                        } 
                    } 
                } 
                None => (),
            } 
        }
        traversal
   }
   fn level_order_traversal(&self) -> Vec<Vec<T>> {
        let mut traversal = Vec::new();
        let mut queue = Vec::new();
        if let Some(root) = &self.root {
           queue.push(Rc::clone(root));
        } else {
            return traversal;
        }
        while queue.len() != 0 {
            let len = queue.len();
            let mut path = Vec::new();
            for _ in 0..len {
                let node = queue.remove(0);
                path.push(node.borrow().data.clone()); 
                if let Some(left) = &node.clone().borrow().left {
                    queue.push(Rc::clone(left));
                }
                if let Some(right) = &node.clone().borrow().right {
                    queue.push(Rc::clone(right));
                }
            }
            traversal.push(path);
        }
        traversal
   }
   fn depth(&self) -> usize {
        if self.is_recursive {
            fn recursive_depth<T: PartialOrd>(root: Rc<RefCell<TreeNode<T>>>, depth: &mut usize, mut cur_depth: usize) {
                 cur_depth += 1;
                 if cur_depth > *depth {*depth = cur_depth;}
                 if let Some(left) = &root.borrow().left {
                    recursive_depth(Rc::clone(left), depth, cur_depth);
                 }
                 if let Some(right) = &root.borrow().right {
                     recursive_depth(Rc::clone(right), depth, cur_depth);
                 }
            }
            let mut depth = 0;
            if let Some(root) = &self.root {
                recursive_depth(Rc::clone(root), &mut depth, 0);
            }
            depth
        } else {
            let mut depth = 0;
            let mut queue = Vec::new();
            if let Some(root) = &self.root {
               queue.push(Rc::clone(root));
            } else {return 0;}
            while queue.len() != 0 {
                let len = queue.len();
                for _ in 0..len {
                    let node = queue.remove(0);
                    if let Some(left) = &node.clone().borrow().left {
                        queue.push(Rc::clone(left));
                    }
                    if let Some(right) = &node.clone().borrow().right {
                        queue.push(Rc::clone(right));
                    }
                }
                depth += 1;
            }
            depth
        }
   }
   fn contains(&self, data: &T) -> bool {
        if self.is_recursive {
            fn recursive_contains<T: PartialOrd>(root: Rc<RefCell<TreeNode<T>>>, data: &T) -> bool {
                if root.borrow().data == *data {
                    true
                } else if root.borrow().data > *data {
                    if let Some(left) = &root.borrow().left {
                        recursive_contains(Rc::clone(left), data)
                    } else {
                        false
                    }
                } else {
                    if let Some(right) = &root.borrow().right {
                        recursive_contains(Rc::clone(right), data)
                    } else {
                        false
                    }
                }
            }
            match &self.root {
                Some(root) => recursive_contains(Rc::clone(root), data),
                None => false,
            }
        } else {
            match &self.root {
                Some(root) => {
                    let mut root = Rc::clone(root); 
                    loop {
                        if root.borrow().data == *data {
                            return true;
                        } else if root.borrow().data > *data {
                            if let Some(left) = &Rc::clone(&root).borrow().left {
                                root = Rc::clone(left);
                            } else {
                                return false;
                            }
                        } else {
                            if let Some(right) = &Rc::clone(&root).borrow().right {
                                root = Rc::clone(right)
                            } else {
                                return false;
                            }
                        }
                    }
                }
                None => false,
            }
        }
   }
   fn size(&self) -> usize {
        self.size
   }
}

impl<T: PartialOrd> PartialEq for BinarySearchTree<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {return false;}
        match (&self.root, &other.root) {
            (Some(root1), Some(root2)) => root1 == root2,
            (None, None) => true,
            _ => false,
        }
    }
}

impl<T: PartialOrd> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            root: None,
            is_recursive: false,
        }
    }    
    pub fn append(&mut self, data: T) {
        if self.is_recursive {
            fn recursive_append<T: PartialOrd>(root: Rc<RefCell<TreeNode<T>>>, data: T) {
                if root.borrow().data > data {
                    match BinarySearchTree::get_left_right_node(Rc::clone(&root)) {
                        (Some(left), _) => {
                            recursive_append(left, data);
                        }
                        (None, _) => {
                            root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::from(data))));
                        }
                    }
                } else {
                    match BinarySearchTree::get_left_right_node(Rc::clone(&root)) {
                        (_, Some(right)) => {
                            recursive_append(right, data);
                        }
                        (_, None) => {
                            root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::from(data))));
                        }
                    }
                }
            }
            match self.root.as_ref() {
                Some(root) => recursive_append(Rc::clone(root), data),
                None => self.root = Some(Rc::new(RefCell::new(TreeNode::from(data)))),
            }
        } else {
            let new_node = TreeNode::from(data);
            match &mut self.root {
                Some(root) => {
                    let mut root = Rc::clone(root);
                    loop {
                        if root.borrow().data > new_node.data {
                            match Self::get_left_right_node(Rc::clone(&root)) {
                                (Some(left), _) => {
                                    root = left;
                                }
                                (None, _) => {
                                    root.borrow_mut().left = Some(Rc::new(RefCell::new(new_node)));
                                    break;
                                }
                            }
                        } else {
                            match Self::get_left_right_node(Rc::clone(&root)) {
                                (_, Some(right)) => {
                                    root = right;
                                }
                                (_, None) => {
                                    root.borrow_mut().right = Some(Rc::new(RefCell::new(new_node)));
                                    break;
                                }
                            }
                        }
                    }
                }
                None => {
                    self.root = Some(Rc::new(RefCell::new(new_node)));
                }
            }   
        }
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
    pub fn enable_recursive_algorithm(&mut self) {self.is_recursive = true;}
    pub fn disable_recursive_algorithm(&mut self) {self.is_recursive = false;}
    
    fn get_left_right_node(root: Rc<RefCell<TreeNode<T>>>) -> (Option<Rc<RefCell<TreeNode<T>>>>, Option<Rc<RefCell<TreeNode<T>>>>) {
        (root.borrow().left.clone(), root.borrow().right.clone())
    }
    fn recursive_delete(root: Rc<RefCell<TreeNode<T>>>, data: &T, remove: &mut Option<T>) -> Option<Rc<RefCell<TreeNode<T>>>> {
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
       Some(root)
    }
    fn delete_node(to_be_remove: TreeNode<T>) -> (Option<Rc<RefCell<TreeNode<T>>>>, T) {
        let data = to_be_remove.data;
        match (to_be_remove.left, to_be_remove.right) {
            (Some(left), Some(right)) => {
                let (_, replace) = Self::find_replace_node(Rc::clone(&left));
                replace.borrow_mut().right = Some(right);
                //replace.borrow_mut().left = Some(left);
                if Rc::ptr_eq(&replace, &left) == false {
                    replace.borrow_mut().left = Some(left);
                } else {
                    replace.borrow_mut().left = None;
                }
                (Some(replace), data)
            }
            (Some(left), None) => (Some(left), data),
            (None, Some(right)) => (Some(right), data),
            (None, None) => (None, data),
        }
    }
    fn find_replace_node(node: Rc<RefCell<TreeNode<T>>>) -> (Option<Rc<RefCell<TreeNode<T>>>>, Rc<RefCell<TreeNode<T>>>) {
        let r = node.borrow_mut().right.take();
        let l = node.borrow_mut().left.take();
        let ((r, replace), l) = match (r, l) {
            (Some(right), Some(left)) => (Self::find_replace_node(right), Some(left)),
            (Some(right), None) => (Self::find_replace_node(right), None),
            (None, Some(left)) => return (Some(left), Rc::clone(&node)),
            (None, None) => ((None, Rc::clone(&node)), None), 
        };
        node.borrow_mut().right = r;
        node.borrow_mut().left = l;
        (Some(node), replace)
    }
}

