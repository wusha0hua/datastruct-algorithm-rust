use crate::datastruct::link_list::doubly_link_list::DoublyLinkList;
pub struct Queue<T: PartialEq> {
    linklist: DoublyLinkList<T>,
}

impl<T: PartialEq> Queue<T> {
    pub fn new() -> Self {
        Self {
            linklist: DoublyLinkList::new(),
        }
    }
    pub fn push(&mut self, data: T) {
        self.linklist.push_back(data);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.linklist.pop_front()
    }
}
