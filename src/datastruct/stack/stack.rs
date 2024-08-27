use crate::node::Node;

#[derive(Debug, Clone)]
pub struct Stack<T: PartialEq> {
    pub len: usize,
    pub top: Option<Box<Node<T>>>
}

impl<T: PartialEq> Stack<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            top: None,
        }
    }
    pub fn push(&mut self, data: T) {
        self.len += 1;
        self.top = Some(Box::new(Node {data, next: self.top.take()}));
    }
    pub fn pop(&mut self) -> Option<T> {
        match self.top.take() {
            Some(mut top) => {
                self.top = top.next.take();
                self.len -= 1;
                Some(top.data)
            }
            None => None,
        }
    }
    pub fn peek(&self) -> Option<&T> {
        match &self.top {
            Some(top) => Some(&top.data),
            None => None,
        }
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        match &mut self.top {
            Some(top) => Some(&mut top.data),
            None => None,
        }
    }
}

impl<T: PartialEq> PartialEq for Stack<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {return false;}
        match (&self.top, &other.top) {
            (Some(node1), Some(node2)) => node1.eq(node2),
            (None, None) => true,
            _ => false,
        }
    }
}
