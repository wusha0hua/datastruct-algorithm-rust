use crate::node::DoublyNode;
use crate::y_combinator::y;
pub use crate::datastruct::link_list::LinkListTrait;

use std::cmp::PartialEq;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct DoublyLinkList<T: PartialEq> {
    pub len: usize,
    pub head: Option<Rc<RefCell<DoublyNode<T>>>>,
    pub tail: Option<Rc<RefCell<DoublyNode<T>>>>,
}

impl<T: PartialEq> LinkListTrait<T> for DoublyLinkList<T> {
    fn is_empty(&self) -> bool {
        self.head == None
    }
    fn len(&self) -> usize {
        self.len
    }
    fn insert(&mut self, index: usize, data: T) -> bool {
        if index > self.len {
            return false;
        }
        match self.head.as_ref() {
            Some(_) if index == self.len => {
                self.push_back(data); 
            }
            Some(_) if index == 0 => {
                self.push_front(data);
            }
            Some(head_rc) => {
                let next_rc = Self::get_node_by_index(head_rc, index, 0);
                let mut new_node = DoublyNode::from(data) ;
                new_node.next = Some(Rc::clone(&next_rc));
                let new_rc = match next_rc.borrow().prev.as_ref() {
                    Some(prev_rc) => {
                        new_node.prev = Some(Rc::clone(prev_rc));
                        let new_rc = Rc::new(RefCell::new(new_node));
                        prev_rc.borrow_mut().next = Some(Rc::clone(&new_rc));
                        new_rc
                    }
                    None => {
                        new_node.prev = None;
                        let new_rc = Rc::new(RefCell::new(new_node));
                        new_rc
                    }
                };
                next_rc.borrow_mut().prev = Some(Rc::clone(&new_rc));
                self.len += 1;
            }
            None => {
                self.push_front(data);
            }
        }
        true
    }
    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }
        match self.head.as_ref() {
            Some(_) if index == 0 => {
                self.pop_front()
            } 
            Some(_) if index == self.len - 1 => {
                self.pop_back()
            }
            Some(head_rc) => {
                let remove_rc = Self::get_node_by_index(head_rc, index, 0);
                let mut next = remove_rc.borrow_mut().next.take();
                let mut prev = remove_rc.borrow_mut().prev.take();
                match next.as_mut() {
                    Some(next_rc) => next_rc.borrow_mut().prev = prev.clone(),
                    None => (),
                }
                match prev.as_mut() {
                    Some(prev_rc) => prev_rc.borrow_mut().next = next,
                    None => (),
                }
                self.len -= 1;
                Some(Rc::try_unwrap(remove_rc).ok().unwrap().into_inner().data)
            }
            None => None,
        }
    }
    fn contains(&self, target: &T) -> Option<usize> {
        let mut cur = self.head.clone();
        let mut index = 0;
        while let Some(cur_rc) = cur {
            if cur_rc.borrow().data == *target {
                return Some(index);
            }
            cur = cur_rc.borrow().next.clone(); 
            index += 1;
        }
        None
    }
    fn clear(&mut self) {
        while let Some(_) = self.pop_front(){}
    }
}

impl<T: PartialEq> PartialEq for DoublyLinkList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {return false;}
        match &self.head {
            Some(node1) => {
                match &other.head {
                    Some(node2) => node2 == node1,
                    None => false,
                }
            }
            None => {
                match &other.head {
                    Some(_) => false,
                    None => true,
                }
            }
        }
    }
}

impl<T: PartialEq>  DoublyLinkList<T> {
    pub fn new() -> Self {
        DoublyLinkList { 
            len: 0, 
            head: None, 
            tail: None,
        }
    }
    pub fn push_back(&mut self, data: T) {
        let mut new_node = DoublyNode::from(data);
        match self.tail.take() {
            Some(tail_rc) =>  {
                new_node.prev = Some(Rc::clone(&tail_rc)); 
                let new_rc = Rc::new(RefCell::new(new_node));
                tail_rc.borrow_mut().next = Some(Rc::clone(&new_rc));
                self.tail = Some(new_rc);
            }
            None => {
                let node = Rc::new(RefCell::new(new_node));
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(&node));
            }
        }
        self.len += 1;
    }
    pub fn push_front(&mut self, data: T) {
        let mut new_node = DoublyNode::from(data);
        match self.head.take() {
            Some(head_rc) => {
                new_node.next = Some(Rc::clone(&head_rc));
                let new_rc = Rc::new(RefCell::new(new_node));
                head_rc.borrow_mut().prev = Some(Rc::clone(&new_rc));
                self.head = Some(new_rc);
            }
            None => {
                let node = Rc::new(RefCell::new(new_node));
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(&node));
            }
        }
        self.len += 1;
    }
    pub fn pop_back(&mut self) -> Option<T> {
        match self.tail.take() {
            Some(tail_rc) => {
                match tail_rc.borrow_mut().prev.take() {
                    Some(prev_rc) => {
                        prev_rc.borrow_mut().next = None;
                        self.tail = Some(Rc::clone(&prev_rc));
                    }
                    None => self.head = None,
                }
                self.len -= 1;
                Some(Rc::try_unwrap(tail_rc).ok().unwrap().into_inner().data)
            } 
            None => None,
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head_rc) => {
                match head_rc.borrow_mut().next.take() {
                    Some(next_rc) => {
                        next_rc.borrow_mut().prev = None;
                        self.head = Some(Rc::clone(&next_rc));
                    }
                    None => self.tail = None,
                }
                self.len -= 1;
                Some(Rc::try_unwrap(head_rc).ok().unwrap().into_inner().data)
            }
            None => None,
        }
    }
    fn get_node_by_index(node: &Rc<RefCell<DoublyNode<T>>>, index: usize, current: usize) -> Rc<RefCell<DoublyNode<T>>> {
        if current == index {
            return Rc::clone(node);
        }
        if let Some(next) = node.borrow().next.as_ref() {
            return Self::get_node_by_index(next, index, current + 1);
        }
        panic!("DoublyLinkList::get_node_by_index error\tindex: {}\tcurrent: {}", index, current);
    }
}
