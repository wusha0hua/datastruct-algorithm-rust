use crate::node::{self, BTreeNode};

#[derive(Debug, Clone)]
pub struct BTree<T: PartialOrd + Default + std::fmt::Debug> {
    pub size: usize,
    pub degree: usize,
    pub root: Option<Box<BTreeNode<T>>>,
}

impl<T> PartialEq for BTree<T> where T: PartialOrd + Default + std::fmt::Debug {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        match (self.root.as_ref(), other.root.as_ref()) {
            (Some(r1), Some(r2)) => r1 == r2,
            (None, None) => true,
            _ => false,
        }
    }
}

impl<T> BTree<T>  where T: PartialOrd + Default + Clone + std::fmt::Debug {
    pub fn new(mut degree: usize) -> Self {
        if degree < 3 {
            degree = 3;
        }
        Self { 
            size: 0, 
            degree, 
            root: None,
        }
    } 
    pub fn append(&mut self, data: T) {
        match Self::recursive_append(self.root.take(), data, self.degree) {
            (None, Some(left), _) => self.root = Some(left),
            (Some(data), Some(left), Some(right)) => self.root = Some(Self::merge_node(None, (data, left, right), 0)),
            result @ _ => unreachable!("append result error: {:?}", result),
        }
        self.size += 1;
    }
    pub fn delete(&mut self, data: &T) -> Option<T> {
        match self.root.take() {
            Some(root) => match Self::recursive_delete(root, data) {
                (root, Some(data)) => {
                    self.size -= 1;
                    self.root = if self.size == 0 {
                        None
                    } else {
                        root
                    };
                    Some(data)
                } 
                (root, None) => {
                    self.root = root;
                    None
                }
            },
            None => None,
        }
    }
    fn recursive_append(root: Option<Box<BTreeNode<T>>>, data: T, degree: usize) -> (Option<T>, Option<Box<BTreeNode<T>>>, Option<Box<BTreeNode<T>>>) {
        match root {
            Some(mut root) => {
                if Self::is_leef_node(&root) {
                    let mut index = root.key_count;
                    root.keys[index] = data;
                    root.key_count += 1;
                    loop {
                        if index > 0 && root.keys[index] < root.keys[index - 1] {
                            root.keys.swap(index, index - 1);
                        } else {
                            break;
                        }
                        index -= 1;
                    }
                    if root.key_count == root.keys.len() {
                        Self::split_node(root)
                    } else {
                        (None, Some(root), None)
                    }
                } else {
                    let mut index = root.key_count;
                    if root.keys[index - 1] < data {
                        match Self::recursive_append(root.next[index].take(), data, degree) {
                            (None, Some(left), _) => root.next[index] = Some(left),
                            (Some(data), Some(left), Some(right)) => root = Self::merge_node(Some(root), (data, left, right), index),
                            result @ _ => unreachable!("recursive append error: {:?}, line: {}", result, std::line!()),
                        }
                    } else {
                        index -= 1;
                        loop {
                            if index > 0 && root.keys[index - 1] >= data {
                                index -= 1;
                            }
                            else {
                                break;
                            }
                        } 
                        match Self::recursive_append(root.next[index].take(), data, degree) {
                            (None, Some(left), _) => root.next[index] = Some(left),
                            (Some(data), Some(left), Some(right)) => root = Self::merge_node(Some(root), (data, left, right), index),
                            result @ _ => unreachable!("recursive append error: {:?}, line: {}", result, std::line!()), 
                        }
                    }
                    if root.key_count == root.keys.len() {
                        Self::split_node(root)
                    } else {
                        (None, Some(root), None)
                    }
                }
            }
            None => (None, Some(Box::new(BTreeNode::from(degree, data))), None),
        }
    }
    fn recursive_delete(mut root: Box<BTreeNode<T>>, data: &T) -> (Option<Box<BTreeNode<T>>>, Option<T>) {
        let index = root.key_count;
        if root.keys[index - 1] < *data {
            match root.next[index].take() {
                Some(next) => {
                    let (next, data) = Self::recursive_delete(next, data);
                    root.next[index] = next;
                    let root = Self::adjust_node(root, index);
                    (Some(root), data)
                }
                None => (Some(root), None),
            }    
        } else {
            let mut index = index;
            while index > 0 {
                if root.keys[index - 1] == *data {
                    index -= 1;
                    break;
                } else if index > 1 && root.keys[index - 1] > *data && root.keys[index - 2] < *data {
                    index -= 1;
                    break;
                }
                index -= 1;
            }
            if root.keys[index] == *data {
                match Self::find_replace_node(root.next[index].take()) {
                    Some((next, mut data)) => {
                        root.next[index] = Some(next);
                        std::mem::swap(&mut root.keys[index], &mut data);
                        let root = Self::adjust_node(root, index);
                        (Some(root), Some(data))
                    }
                    None => {
                        let mut data = T::default();
                        std::mem::swap(&mut root.keys[index], &mut data);
                        for i in index..root.key_count - 1 {
                            root.keys.swap(i, i + 1);
                        }
                        root.key_count -= 1;
                        (Some(root), Some(data))
                    }
                }
            } else {
                match root.next[index].take() {
                    Some(next) => {
                        let (next, data) = Self::recursive_delete(next, data);
                        root.next[index] = next;
                        let root = Self::adjust_node(root, index);
                        (Some(root), data)           
                    }
                    None => (Some(root), None),
                }
            }
        }
    }
    fn split_node(mut node: Box<BTreeNode<T>>) -> (Option<T>, Option<Box<BTreeNode<T>>>, Option<Box<BTreeNode<T>>>) {
        let degree = node.keys.len();
        let mid = (degree - 1) / 2;
        let left_count = mid;
        let right_count = degree - 1 - left_count;
        let mut left = BTreeNode::new();
        let mut right = BTreeNode::new();
        right.keys = node.keys.split_off(mid + 1); 
        right.key_count = right_count;
        right.keys.resize(degree, T::default());
        right.next = node.next.split_off(mid + 1);
        right.next.resize(degree + 1, None);
        let data = node.keys.pop().unwrap();
        left.keys = node.keys;
        left.key_count = left_count;
        left.keys.resize(degree, T::default());
        left.next = node.next;
        left.next.resize(degree + 1, None);
        (Some(data), Some(Box::new(left)), Some(Box::new(right)))
    }
    fn merge_node(node: Option<Box<BTreeNode<T>>>, (data, left, right): (T, Box<BTreeNode<T>>, Box<BTreeNode<T>>), index: usize) -> Box<BTreeNode<T>> {
        match node {
            Some(mut node) => {
                let count = node.key_count;
                node.keys[count] = data;
                for i in (index + 1..count + 1).rev(){
                    node.keys.swap(i, i - 1);
                }
                node.key_count += 1;
                node.next[count + 1] = Some(right);
                node.next[index] = Some(left);
                for i in (index + 2..=count + 1).rev() {
                    node.next.swap(i, i - 1);
                }
                node
            }
            None => {
                let degree = left.keys.len();
                let mut root = BTreeNode::from(degree, data);
                root.next[0] = Some(left);
                root.next[1] = Some(right);
                Box::new(root)
            }
        }
    }
    fn find_replace_node(node: Option<Box<BTreeNode<T>>>) -> Option<(Box<BTreeNode<T>>, T)> {
        match node {
            Some(mut node) => {
                if Self::is_leef_node(&node) {
                    let index = node.key_count - 1;
                    let mut data = T::default();
                    std::mem::swap(&mut node.keys[index], &mut data);
                    node.key_count -= 1;
                    Some((node, data))
                } else {
                    let index = node.key_count;
                    match Self::find_replace_node(node.next[index].take()) {
                        Some((next, data)) => {
                            node.next[index] = Some(next);
                            let node = Self::adjust_node(node, index);
                            Some((node, data))
                        }
                        None => unreachable!("find replace node error"),
                    }
                }
            }
            None => None,
        }
    }
    fn adjust_node(mut node: Box<BTreeNode<T>>, index: usize) -> Box<BTreeNode<T>> {
        let threshold = (node.keys.len() - 1) / 2;
        if index == 0 {
            match (node.next[0].take(), node.next[1].take()) {
                (Some(mut left), Some(mut right)) => {
                    if left.key_count >= threshold {
                        node.next[0] = Some(left);
                        node.next[1] = Some(right);
                    } else if right.key_count == threshold {
                        Self::adjust_by_merge_with_brother(&mut node, left, right, index);
                    } else {
                        Self::adjust_by_take_a_key_from_brother(&mut node, &mut left, &mut right, index, true);
                        node.next[0] = Some(left);
                        node.next[1] = Some(right);
                    }
                } 
                result @ _ => unreachable!("adjust node error: {:?}, line: {}", result, std::line!()),
            }
        } else if index == node.key_count {
            match (node.next[index - 1].take(), node.next[index].take()) {
                (Some(mut left), Some(mut right)) => {
                    if right.key_count >= threshold {
                        node.next[index] = Some(right);
                        node.next[index - 1] = Some(left);
                    } else if left.key_count == threshold {
                        Self::adjust_by_merge_with_brother(&mut node, left, right, index - 1);
                    } else {
                        Self::adjust_by_take_a_key_from_brother(&mut node, &mut right, &mut left, index, false);
                        node.next[index] = Some(right);
                        node.next[index - 1] = Some(left);
                    }
                } 
                result @ _ => unreachable!("adjust node error: {:?}, line: {}", result, std::line!()),
            }
        } else {
             match (node.next[index - 1].take(), node.next[index].take(), node.next[index + 1].take()) {
                (Some(mut left), Some(mut mid), Some(mut right)) => {
                    if mid.key_count >= threshold {
                        node.next[index - 1] = Some(left);
                        node.next[index] = Some(mid);
                        node.next[index + 1] = Some(right);
                    } else if left.key_count > threshold {
                        node.next[index + 1] = Some(right);
                        Self::adjust_by_take_a_key_from_brother(&mut node, &mut mid, &mut left, index, false);
                        node.next[index] = Some(mid);
                        node.next[index - 1] = Some(left);
                    } else if right.key_count > threshold {
                        node.next[index - 1] = Some(left);
                        Self::adjust_by_take_a_key_from_brother(&mut node, &mut mid, &mut right, index, true);
                        node.next[index] = Some(mid);
                        node.next[index + 1] = Some(right);
                    } else {
                        node.next[index + 1] = Some(right);
                        Self::adjust_by_merge_with_brother(&mut node, mid, left, index - 1);
                    }
                } 
                result @ _ => unreachable!("adjust node error: {:?}, line: {}", result, std::line!()),
            }           
        }
        node
    }
    fn adjust_by_take_a_key_from_brother(parent: &mut Box<BTreeNode<T>>, target: &mut Box<BTreeNode<T>>, brother: &mut Box<BTreeNode<T>>, index: usize, option: bool) {
        if option {
            let (mut data, next) = Self::left_shift_remove(brother);
            std::mem::swap(&mut parent.keys[index], &mut data);
            Self::right_shift_insert(target, (data, next));
        } else {
            let (mut data, next) = Self::right_shift_remove(brother);
            std::mem::swap(&mut parent.keys[index - 1], &mut data);
            Self::left_shift_insert(target, (data, next));
        }
    }
    fn adjust_by_merge_with_brother(parent: &mut Box<BTreeNode<T>>, target: Box<BTreeNode<T>>, brother: Box<BTreeNode<T>>, index: usize) {
        let data = Self::remove_data(parent, index);
        let next = Self::merge(brother, target, data);
        parent.next[index] = Some(next);
    }
    fn merge(mut main: Box<BTreeNode<T>>, mut sub: Box<BTreeNode<T>>, data: T) -> Box<BTreeNode<T>> {
        let mut main_index = main.key_count;
        main.keys[main_index] = data;
        main.key_count += 1;
        main_index += 1;
        for i in 0..sub.key_count {
            std::mem::swap(&mut main.keys[main_index + i], &mut sub.keys[i]);
        }
        main.key_count += sub.key_count;
        for i in 0..sub.key_count + 1 {
            std::mem::swap(&mut main.next[main_index + i], &mut sub.next[i]);
        }
        main
    } 
    fn remove_data(node: &mut Box<BTreeNode<T>>, index: usize) -> T {
        let mut data = T::default();
        std::mem::swap(&mut node.keys[index], &mut data);
        for i in index..node.key_count {
            node.keys.swap(i, i + 1);
        }
        for i in index + 1..node.key_count {
            node.next.swap(i, i + 1);
        }
        node.key_count -= 1;
        data
    }
    fn right_shift_remove(node: &mut Box<BTreeNode<T>>) -> (T, Option<Box<BTreeNode<T>>>) {
        let mut data = T::default();
        let index = node.key_count - 1;
        std::mem::swap(&mut node.keys[index], &mut data);
        node.key_count -= 1; 
        (data, node.next[index + 1].take())
    }
    fn left_shift_remove(node: &mut Box<BTreeNode<T>>) -> (T, Option<Box<BTreeNode<T>>>) {
        let mut data = T::default();
        let index = node.key_count;
        std::mem::swap(&mut node.keys[0], &mut data);
        node.key_count -= 1;
        let next = node.next[0].take();
        for i in 1..index {
            node.keys.swap(i, i - 1);
        }
        for i in 1..index + 1 {
            node.next.swap(i, i - 1);
        }
        (data, next)
    }
    fn right_shift_insert(node: &mut Box<BTreeNode<T>>, (data, next): (T, Option<Box<BTreeNode<T>>>)) {
        let index = node.key_count;
        node.keys[index] = data;
        node.key_count += 1;
        node.next[index + 1] = next;
    }
    fn left_shift_insert(node: &mut Box<BTreeNode<T>>, (data, next): (T, Option<Box<BTreeNode<T>>>)) {
        let index = node.key_count;
        node.keys[index] = data;
        for i in (1..index + 1).rev() {
            node.keys.swap(i, i - 1);
        }
        node.key_count += 1;
        node.next[index + 1] = next;
        for i in (1..=index + 1).rev() {
            node.next.swap(i, i - 1);
        }
    }
    fn is_leef_node(node: &Box<BTreeNode<T>>) -> bool {
        for i in 0..node.key_count + 1 {
            if let Some(_) = &node.next[i] {
                return false;
            }
        }
        true
    }
}
