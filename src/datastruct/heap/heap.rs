
#[derive(Debug, Clone)]
pub enum HeapOrder {
    MAX,
    MIN,
}

#[derive(Debug, Clone)]
pub struct Heap<T: PartialOrd> {
    pub heap: Vec<T>,
    pub order: HeapOrder, 
}

impl<T: PartialOrd> Heap<T> {
    pub fn new() -> Self {
        Self {
            heap: Vec::new(),
            order: HeapOrder::MIN,
        }
    }
    pub fn push(&mut self, data: T) {
        match &self.order {
            HeapOrder::MAX => {
                let mut cur = self.heap.len();
                self.heap.push(data);
                while cur != 0 {
                    let parent = if cur % 2 == 0 {
                        (cur - 1) / 2
                    } else {
                        cur / 2
                    };
                    if self.heap[parent] < self.heap[cur] {
                        self.heap.swap(parent, cur);
                        cur = parent;
                    } else {
                        break;
                    }
                }
            }
            HeapOrder::MIN => {
                let mut cur = self.heap.len();
                self.heap.push(data);
                while cur != 0 {
                    let parent = if cur % 2 == 0 {
                        (cur - 1) / 2
                    } else {
                        cur / 2
                    };
                    if self.heap[parent] > self.heap[cur] {
                        self.heap.swap(parent, cur);
                        cur = parent;
                    } else {
                        break;
                    }
                }
            }
        }
    } 
    pub fn pop(&mut self) -> Option<T> {
        if self.heap.len() == 0 {return None;}
        match &self.order {
            HeapOrder::MAX => {
                let mut cur = 0;
                let len = self.heap.len();
                self.heap.swap(0, len - 1);
                let data = self.heap.pop();
                while cur * 2 + 1 < len - 1 {
                    let left = cur * 2 + 1;
                    let right = cur * 2 + 2;
                    if right < len - 1 {
                        if self.heap[right] > self.heap[left] {
                            if self.heap[right] > self.heap[cur] {
                                self.heap.swap(right, cur);
                                cur = right;
                            } else {
                                break;
                            }
                        } else {
                             if self.heap[left] > self.heap[cur] {
                                self.heap.swap(left, cur);
                                cur = left;
                            } else {
                                break;
                            }
                        }
                    } else {
                        if self.heap[left] > self.heap[cur] {
                            self.heap.swap(left, cur);
                            cur = left;
                        } else {
                            break;
                        }
                    }
                }
                data
            }
            HeapOrder::MIN => {
                let mut cur = 0;
                let len = self.heap.len();
                self.heap.swap(0, len - 1);
                let data = self.heap.pop();
                while cur * 2 + 1 < len - 1 {
                    let left = cur * 2 + 1;
                    let right = cur * 2 + 2;
                    if right < len - 1 {
                        if self.heap[right] < self.heap[left] {
                            if self.heap[right] < self.heap[cur] {
                                self.heap.swap(right, cur);
                                cur = right;
                            } else {
                                break;
                            }
                        } else {
                             if self.heap[left] < self.heap[cur] {
                                self.heap.swap(left, cur);
                                cur = left;
                            } else {
                                break;
                            }
                        }
                    } else {
                        if self.heap[left] < self.heap[cur] {
                            self.heap.swap(left, cur);
                            cur = left;
                        } else {
                            break;
                        }
                    }
                }
                data

            }
        }
    }
    pub fn set_heap_order_min(&mut self) {
        if let HeapOrder::MAX = self.order {
            self.order = HeapOrder::MIN;
            let mut vec = Vec::new();
            while let Some(n) = self.heap.pop() {
                vec.push(n);
            }
            for n in vec {
                self.push(n);
            }
        }
    }
    pub fn set_heap_order_max(&mut self) {
        if let HeapOrder::MIN = self.order {
            self.order = HeapOrder::MAX;
            let mut vec = Vec::new();
            while let Some(n) = self.heap.pop() {
                vec.push(n);
            }
            for n in vec {
                self.push(n);
            }

        }
    }
}
