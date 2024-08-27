pub mod stack;

#[cfg(test)]
mod test_stack {
    use crate::node::Node;
    use crate::datastruct::stack::stack::Stack;
    #[test]
    fn test_stack() {
        let mut stack = Stack::new(); 
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack, Stack {
            len: 3,
            top: Some(Box::new(Node { 
                data: 3, 
                next: Some(Box::new(Node { 
                    data: 2, 
                    next: Some(Box::new(Node { 
                        data: 1, 
                        next: None 
                    })) 
                })) 
            }))
        }); 
        assert_eq!(stack.peek(), Some(&3));
        *stack.peek_mut().unwrap() = 4;
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.peek(), Some(&2));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
    }
}
