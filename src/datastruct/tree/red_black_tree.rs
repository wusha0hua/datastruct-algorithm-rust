
#[derive(Debug, PartialEq)]
pub struct RedBlackTree<T: PartialOrd + std::fmt::Debug> {
    size: usize,
    root: Option<Box<RedBlackNode<T>>>,
}

#[derive(Debug, PartialEq, Clone)]
struct RedBlackNode<T: PartialOrd + std::fmt::Debug> {
    color: Color,
    key: T, 
    left: Option<Box<RedBlackNode<T>>>,
    right: Option<Box<RedBlackNode<T>>>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Direction {
    Left,
    Right,
    Changed,
    None,
}

#[derive(Debug, PartialEq)]
enum DeleteState<'a, T> where T: PartialOrd + std::fmt::Debug {
    FindForDeleteNode(&'a T),
    FindForPredecessor,
    FindForSuccessor,
    TryAdjust(Option<Box<RedBlackNode<T>>>, Box<RedBlackNode<T>>),
    Done(Option<T>),
}


impl<T: PartialOrd + std::fmt::Debug> RedBlackTree<T> {
    pub fn new() -> Self {
        Self { 
            size: 0, 
            root: None, 
        }
    }    
    pub fn append(&mut self, key: T) {
        match self.root.take() {
            Some(root) => {
                let (mut root, _) = root.append(key);
                root.color = Color::Black;
                self.root = Some(root);
            }
            None => self.root = Some(Box::new(RedBlackNode::from(key))),
        }
        self.size += 1;
    }
    pub fn delete(&mut self, key: &T) -> Option<T> {
        match self.root.take() {
            Some(root) => match root.delete(DeleteState::FindForDeleteNode(key)) {
                (root, DeleteState::Done(reuslt)) => {
                    self.root = root;
                    match reuslt {
                        Some(result) => {
                            self.size -= 1;
                            Some(result)
                        }
                        None => None,
                    }
                }
                (_, DeleteState::TryAdjust(node, to_be_deleted)) => {
                    match node {
                        Some(root) => {
                            self.root = Some(root);
                            self.size -= 1;
                            Some(to_be_deleted.key)
                        }
                        None => {
                            let (root, key) = RedBlackNode::delete_root(to_be_deleted);
                            self.root = root;
                            self.size -= 1;
                            Some(key)
                        }
                    }
                }
                state @ _ => unreachable!("delete state error: {:?}, line: {}", state, std::line!()),
            }
            None => None,
        }
    }
}

impl<T: PartialOrd + std::fmt::Debug> RedBlackNode<T> {
    pub fn from(key: T) -> Self {
        Self { 
            color: Color::Red, 
            key, 
            left: None, 
            right: None,
        }
    }
    fn append(mut self, key: T) -> (Box<RedBlackNode<T>>, Direction) {
        if self.key > key {
            match self.left.take() {
                Some(left) => {
                    let (left, direction) = left.append(key);
                    let (_, right_color) = self.get_child_color();
                    match direction {
                        Direction::Left if left.color == Color::Red && right_color == Color::Red => {
                            self.left = Some(left);
                            self.change_color_for_nodes();
                            (Box::new(self), Direction::Changed)
                        }
                        Direction::Left if left.color == Color::Red && right_color == Color::Black => {
                            (self.append_ll(left), Direction::None)
                        }
                        Direction::Right if left.color == Color::Red && right_color == Color::Red=> {
                            self.left = Some(left);
                            self.change_color_for_nodes();
                            (Box::new(self), Direction::Changed)
                        }
                        Direction::Right if left.color == Color::Red && right_color == Color::Black => {
                            (self.append_lr(left), Direction::None)
                        }
                        Direction::Changed => {
                            self.left = Some(left);
                            (Box::new(self), Direction::Left)
                        }
                        _ => {
                            self.left = Some(left);
                            (Box::new(self), Direction::None)
                        }
                    }
                }
                None => {
                    self.left = Some(Box::new(RedBlackNode::from(key)));
                    (Box::new(self), Direction::Left)
                }
            } 
        } else {
            match self.right.take() {
                Some(right) => {
                    let (right, direction) = right.append(key);
                    let (left_color, _) = self.get_child_color();
                    match direction {
                        Direction::Left if right.color == Color::Red && left_color == Color::Red => {
                            self.right = Some(right);
                            self.change_color_for_nodes();
                            (Box::new(self), Direction::Changed)
                        }
                        Direction::Left if right.color == Color::Red && left_color == Color::Black => {
                            (self.append_rl(right), Direction::None)
                        }
                        Direction::Right if right.color == Color::Red && left_color == Color::Red=> {
                            self.right = Some(right);
                            self.change_color_for_nodes();
                            (Box::new(self), Direction::Changed)
                        }
                        Direction::Right if right.color == Color::Red && left_color == Color::Black => {
                            (self.append_rr(right), Direction::None)
                        }
                        Direction::Changed => {
                            self.right = Some(right);
                            (Box::new(self), Direction::Right)
                        }
                        _ => {
                            self.right = Some(right);
                            (Box::new(self), Direction::None)
                        }
                    }
                }
                None => {
                    self.right = Some(Box::new(RedBlackNode::from(key)));
                    (Box::new(self), Direction::Right)
                }
            }
        }
    }
    fn delete(mut self, state: DeleteState<T>) -> (Option<Box<RedBlackNode<T>>>, DeleteState<T>) {
        match state {
            DeleteState::FindForDeleteNode(key) => {
                if self.key == *key {
                    match (self.left.take(), self.right.take()) {
                        (Some(left), Some(right)) => {
                            // let (left, state) = left.delete(DeleteState::FindForPredecessor);
                            // self.left = left;
                            // self.right = Some(right);
                            let (right, state) = right.delete(DeleteState::FindForSuccessor);
                            self.left = Some(left);
                            self.right = right;
                            let state = match state {
                                DeleteState::Done(Some(mut key)) => {
                                    std::mem::swap(&mut key, &mut self.key);
                                    DeleteState::Done(Some(key))
                                }
                                DeleteState::TryAdjust(node, mut to_be_deleted) => {
                                    std::mem::swap(&mut to_be_deleted.key, &mut self.key);
                                    DeleteState::TryAdjust(node, to_be_deleted)
                                }
                                result @ _ => unreachable!("delete error: {:?}, line: {}", result, std::line!()),
                            };
                            self.adjust(state)
                        }
                        (left, right) => {
                            self.left = left;
                            self.right = right;
                            (None, DeleteState::TryAdjust(None, Box::new(self)))
                        }
                    }
                } else if self.key > *key {
                    match self.left.take() {
                        Some(left) => {
                            let (left, state) = left.delete(state);
                            self.left = left;
                            self.adjust(state)
                        }
                        None => (Some(Box::new(self)), DeleteState::Done(None)),
                    }
                } else {
                    match self.right.take() {
                        Some(right) => {
                            let (right, state) = right.delete(state);
                            self.right = right;
                            self.adjust(state)
                        }
                        None => (Some(Box::new(self)), DeleteState::Done(None)),
                    }
                }
            }  
            DeleteState::FindForPredecessor => match self.right.take() {
                Some(right) => {
                    let (right, state) = right.delete(DeleteState::FindForPredecessor);
                    self.right = right;
                    self.adjust(state)
                }
                None => (None, DeleteState::TryAdjust(None, Box::new(self))),
            }
            DeleteState::FindForSuccessor => match self.left.take() {
                Some(left) => {
                    let (left, state) = left.delete(DeleteState::FindForSuccessor);
                    self.left = left;
                    self.adjust(state)
                }
                None => (None, DeleteState::TryAdjust(None, Box::new(self))),
            }
            _ => todo!()
        }
    }
    fn adjust(mut self, state: DeleteState<T>) -> (Option<Box<RedBlackNode<T>>>, DeleteState<T>) {
        if let DeleteState::TryAdjust(node, mut to_be_deleted) = state {
            match (to_be_deleted.left.take(), to_be_deleted.right.take()) {
                (Some(mut delete_left), None) => match (self.left.take(), self.right.take()) {
                    (Some(left), None) => {
                        self.left = Some(left);
                        delete_left.set_color_black();
                        self.right = Some(delete_left);
                        (Some(Box::new(self)), DeleteState::Done(Some(to_be_deleted.key)))
                    } 
                    (None, Some(right)) => {
                        self.right = Some(right);
                        delete_left.set_color_black();
                        self.left = Some(delete_left);
                        (Some(Box::new(self)), DeleteState::Done(Some(to_be_deleted.key)))
                    }
                    result @ _ => unreachable!("adjust error: {:?}, line: {}", result, std::line!()),
                }
                (None, Some(mut delete_right)) => match (self.left.take(), self.right.take()) {
                     (Some(left), None) => {
                        self.left = Some(left);
                        delete_right.set_color_black();
                        self.right = Some(delete_right);
                        (Some(Box::new(self)), DeleteState::Done(Some(to_be_deleted.key)))
                    } 
                    (None, Some(right)) => {
                        self.right = Some(right);
                        delete_right.set_color_black();
                        self.left = Some(delete_right);
                        (Some(Box::new(self)), DeleteState::Done(Some(to_be_deleted.key)))
                    }
                    result @ _ => unreachable!("adjust error: {:?}, line: {}", result, std::line!()),                   
                }
                (None, None) => {
                    match to_be_deleted.color {
                        Color::Red => (Some(Box::new(self)), DeleteState::Done(Some(to_be_deleted.key))),
                        Color::Black => match (self.left.take(), self.right.take()) {
                            (Some(mut left), None) => {
                                match left.color {
                                    Color::Black => match (left.left.take(), left.right.take()) {
                                        (Some(sub_left), sub_right) if sub_left.color == Color::Red => {
                                            self.right = node;
                                            left.right = sub_right;
                                            let root = self.delete_ll(left, sub_left);
                                            (Some(root), DeleteState::Done(Some(to_be_deleted.key)))
                                        }
                                        (sub_left, Some(sub_right)) if sub_right.color == Color::Red => {
                                            self.right = node;
                                            left.left = sub_left;
                                            let root = self.delete_lr(left, sub_right);
                                            (Some(root), DeleteState::Done(Some(to_be_deleted.key)))
                                        }
                                        (sub_left, sub_right) => {
                                            left.set_color_red();
                                            left.left = sub_left;
                                            left.right = sub_right;
                                            self.right = node;
                                            self.left = Some(left);
                                            match self.color {
                                                Color::Red => {
                                                    self.set_color_black();
                                                    (Some(Box::new(self)), DeleteState::Done(Some(to_be_deleted.key)))
                                                }
                                                Color::Black => (None, DeleteState::TryAdjust(Some(Box::new(self)), to_be_deleted)),
                                            }
                                        }
                                    }
                                    Color::Red => {
                                        self.change_color();
                                        left.change_color();
                                        self.left = left.right.take();
                                        let (root, state) = self.adjust(DeleteState::TryAdjust(node, to_be_deleted));
                                        //left.right = root;
                                        //(Some(left), state)
                                        match &state {
                                            DeleteState::TryAdjust(_, _) => left.adjust(state),
                                            DeleteState::Done(_) => {
                                                left.right = root;
                                                (Some(left), state)
                                            }
                                            result @ _ => unreachable!("adjust error: {:?}, {}", result, std::line!()),
                                        }
                                    }
                                }
                            }
                            (None, Some(mut right)) => {
                                match right.color {
                                     Color::Black => match (right.left.take(), right.right.take()) {
                                        (sub_left, Some(sub_right)) if sub_right.color == Color::Red => {
                                            self.left = node;
                                            right.left = sub_left;
                                            let root = self.delete_rr(right, sub_right);
                                            (Some(root), DeleteState::Done(Some(to_be_deleted.key)))
                                        }
                                        (Some(sub_left), sub_right) if sub_left.color == Color::Red => {
                                            self.left = node;
                                            right.right = sub_right;
                                            let root = self.delete_rl(right, sub_left);
                                            (Some(root), DeleteState::Done(Some(to_be_deleted.key)))
                                        }
                                        (sub_left, sub_right) => {
                                            right.set_color_red();
                                            right.left = sub_left;
                                            right.right = sub_right;
                                            self.left = node;
                                            self.right = Some(right);
                                            match self.color {
                                                Color::Red => {
                                                    self.set_color_black();
                                                    (Some(Box::new(self)), DeleteState::Done(Some(to_be_deleted.key)))
                                                }
                                                Color::Black => (None, DeleteState::TryAdjust(Some(Box::new(self)), to_be_deleted)),
                                            }
                                        }
                                    }                                   
                                    Color::Red => {
                                        self.change_color();
                                        right.change_color();
                                        self.right = right.left.take();
                                        let (root, state) = self.adjust(DeleteState::TryAdjust(node, to_be_deleted));
                                        //right.left = root;
                                        //(Some(right), state)
                                        match &state {
                                            DeleteState::TryAdjust(_, _) => right.adjust(state),
                                            DeleteState::Done(_) => {
                                                right.left = root;
                                                (Some(right), state)
                                            }
                                            result @ _ => unreachable!("adjust error: {:?}, {}", result, std::line!()),
                                        }
                                    }
                                }
                            }
                            result @ _ => unreachable!("adjust error: {:?}, line: {}\n self: {:?}", result, std::line!(), self)
                        }
                    }
                }
                result @ _ => unreachable!("adjust error: {:?}, line: {}", result, std::line!()),
            } 
        } else {
            (Some(Box::new(self)), state)
        }
    }
    fn delete_root(mut root: Box<RedBlackNode<T>>) -> (Option<Box<RedBlackNode<T>>>, T) {
        match (root.left.take(), root.right.take()) {
            (Some(mut left), None) => {
                left.set_color_black();
                (Some(left), root.key)
            }
            (None, Some(mut right)) => {
                right.set_color_black();
                (Some(right), root.key)
            }
            (None, None) => (None, root.key),
            result @ _ => unreachable!("delete root error: {:?}", result),
        }
    }
    fn append_ll(mut self, mut left: Box<RedBlackNode<T>>) -> Box<RedBlackNode<T>> {
        self.change_color();
        left.change_color();
        self.left = Some(left);
        self.right_rorate()
    }
    fn append_rr(mut self, mut right: Box<RedBlackNode<T>>) -> Box<RedBlackNode<T>> {
        self.change_color();
        right.change_color();
        self.right = Some(right);
        self.left_rorate()
    }
    fn append_lr(mut self, mut left: Box<RedBlackNode<T>>) -> Box<RedBlackNode<T>> {
        self.change_color();
        left.right = match left.right.take() {
            Some(mut right) => {
                right.change_color();
                Some(right)
            }
            None => unreachable!("lr error: {:?}", left),
        };
        left = left.left_rorate();
        self.left = Some(left);
        self.right_rorate()
    }
    fn append_rl(mut self, mut right: Box<RedBlackNode<T>>) -> Box<RedBlackNode<T>> {
        self.change_color();
        right.left = match right.left.take() {
            Some(mut left) => {
                left.change_color();
                Some(left)
            }
            None => unreachable!("rl error: {:?}", right),
        };
        right = right.right_rorate();
        self.right = Some(right);
        self.left_rorate()
    }
    fn delete_ll(mut self, mut left: Box<RedBlackNode<T>>, mut sub_left: Box<RedBlackNode<T>>) -> Box<RedBlackNode<T>> {
        sub_left.color = left.color;
        left.color = self.color;
        self.set_color_black();
        left.left = Some(sub_left);
        self.left = Some(left);
        self.right_rorate()
    }
    fn delete_rr(mut self, mut right: Box<RedBlackNode<T>>, mut sub_right: Box<RedBlackNode<T>>) -> Box<RedBlackNode<T>> {
        sub_right.color = right.color;
        right.color = self.color;
        self.set_color_black();
        right.right = Some(sub_right);
        self.right = Some(right);
        self.left_rorate()
    }
    fn delete_lr(mut self, mut left: Box<RedBlackNode<T>>, mut sub_right: Box<RedBlackNode<T>>) -> Box<RedBlackNode<T>> {
        sub_right.color = self.color;
        self.set_color_black();
        left.right = Some(sub_right);
        self.left = Some(left.left_rorate());
        self.right_rorate()
    }
    fn delete_rl(mut self, mut right: Box<RedBlackNode<T>>, mut sub_left: Box<RedBlackNode<T>>) -> Box<RedBlackNode<T>> {
        sub_left.color = self.color;
        self.set_color_black();
        right.left = Some(sub_left);
        self.right = Some(right.right_rorate());
        self.left_rorate()
    }
    fn change_color_for_nodes(&mut self) {
        self.change_color();
        let (left, right) = match (self.left.take(), self.right.take()) {
            (Some(mut left), Some(mut right)) => {
                left.change_color();
                right.change_color();
                (Some(left), Some(right))
            }
            (Some(mut left), None) => {
                left.change_color();
                (Some(left), None)
            }
            (None, Some(mut right)) => {
                right.change_color();
                (None, Some(right))
            }
            (None, None) => {
                (None, None)
            }
        };
        self.left = left;
        self.right = right;
    }
    fn left_rorate(mut self) -> Box<RedBlackNode<T>> {
        match self.right.take() {
            Some(mut right) => {
                self.right = right.left.take();
                right.left = Some(Box::new(self));
                right
            }
            None => unreachable!("left rorate error"),
        }
    }
    fn right_rorate(mut self) -> Box<RedBlackNode<T>> {
        match self.left.take() {
            Some(mut left) => {
                self.left = left.right.take();
                left.right = Some(Box::new(self));
                left
            }
            None => unreachable!("right rorate error"),
        }
    }

    fn get_child_color(&self) -> (Color, Color) {
        match (self.left.as_ref(), self.right.as_ref()) {
            (Some(left), Some(right)) => (left.color, right.color),
            (Some(left), None) => (left.color, Color::Black),
            (None, Some(right)) => (Color::Black, right.color),
            (None, None) => (Color::Black, Color::Black),
        }
    }
    fn set_color_black(&mut self) {
        self.color = Color::Black;
    }
    fn set_color_red(&mut self){
        self.color = Color::Red;
    }
    fn change_color(&mut self) {
        match self.color {
            Color::Red => self.color = Color::Black,
            Color::Black => self.color = Color::Red,
        }
    }
}


impl RedBlackTree<i32> {
    pub fn gen_data_for_test_append_base() -> (Self, Vec<i32>) {
        let rbt = Self {
            size: 3,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 2, 
                left: Some(Box::new(RedBlackNode::<i32>::from(1))), 
                right: Some(Box::new(RedBlackNode::<i32>::from(3))) 
            })),
        };
        (rbt, vec![2, 1, 3])
    }
    pub fn gen_data_for_test_append_change_color_once() -> (Self, Vec<i32>) {
        let mut node = RedBlackNode::from(8);
        node.change_color();
        let rbt = Self {
            size: 4,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 7, 
                left: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 5, 
                    left: Some(Box::new(RedBlackNode::from(1))), 
                    right: None, 
                })), 
                right: Some(Box::new(node))
            }))
        };
        (rbt, vec![7, 5, 8, 1])
    }
    pub fn gen_data_for_test_append_ll() -> (Self, Vec<i32>) {
        let rbt = Self {
            size: 3,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 5, 
                left: Some(Box::new(RedBlackNode::from(1))), 
                right: Some(Box::new(RedBlackNode::from(7))),
            }))
        };
        (rbt, vec![7, 5, 1])
    }
    pub fn gen_data_for_test_append_rr() -> (Self, Vec<i32>) {
        let rbt = Self {
            size: 3,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 9, 
                left: Some(Box::new(RedBlackNode::from(7))), 
                right: Some(Box::new(RedBlackNode::from(12))) 
            }))
        };
        (rbt, vec![7, 9, 12])
    }
    pub fn gen_data_for_test_append_lr() -> (Self, Vec<i32>) {
        let rbt = Self {
            size: 3,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 5, 
                left: Some(Box::new(RedBlackNode::from(3))), 
                right: Some(Box::new(RedBlackNode::from(7))) 
            }))
        };
        (rbt, vec![7, 3, 5])
    }
    pub fn gen_data_for_test_append_rl() -> (Self, Vec<i32>) {
        let rbt = Self {
            size: 3,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 8, 
                left: Some(Box::new(RedBlackNode::from(7))), 
                right: Some(Box::new(RedBlackNode::from(12))) 
            }))
        };
        (rbt, vec![7, 12, 8])
    }
    pub fn gen_data_for_test_append_multiple_adjust() -> (Self, Vec<i32>) {
        let rbt = Self {
            size: 11,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 15, 
                left: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 8, 
                    left: Some(Box::new(RedBlackNode { 
                        color: Color::Black, 
                        key: 6, 
                        left: Some(Box::new(RedBlackNode::from(5))), 
                        right: None,
                    })), 
                    right: Some(Box::new(RedBlackNode { color: Color::Black, key: 9, left: None, right: None })) 
                })), 
                right: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 18, left: Some(Box::new(RedBlackNode { 
                        color: Color::Black, 
                        key: 17, 
                        left: None, 
                        right: None 
                    })), 
                    right: Some(Box::new(RedBlackNode { 
                        color: Color::Red, 
                        key: 27, 
                        left: Some(Box::new(RedBlackNode { 
                            color: Color::Black, 
                            key: 23, 
                            left: None, 
                            right: Some(Box::new(RedBlackNode::from(25))) 
                        })), 
                        right: Some(Box::new(RedBlackNode { 
                            color: Color::Black, 
                            key: 34, 
                            left: None, 
                            right: None 
                        })) 
                    })) 
                }))
            }))
        };
        (rbt, vec![17, 18, 23, 34, 27, 15, 9, 6, 8, 5, 25])
    }
    pub fn gen_data_for_delete_just_a_node() -> (RedBlackTree<i32>, Vec<i32>, Vec<i32>) {
        let rbt = RedBlackTree {
            size: 0,
            root: None,
        };
        (rbt, vec![5], vec![5])
    }
    pub fn gen_data_for_delete_node_with_a_left_child() -> (RedBlackTree<i32>, Vec<i32>, Vec<i32>) {
        let mut node1 = RedBlackNode::from(1);
        let mut node2 = RedBlackNode::from(20);
        node1.change_color();
        node2.change_color();
        let rbt = RedBlackTree {
            size: 3,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 10, 
                left: Some(Box::new(node1)), 
                right: Some(Box::new(node2)),
            }))
        };       
        (rbt, vec![10, 20, 5, 1], vec![5])
    }
    pub fn gen_data_for_delete_node_with_a_right_child() -> (RedBlackTree<i32>, Vec<i32>, Vec<i32>) {
        let mut node1 = RedBlackNode::from(10);
        let mut node2 = RedBlackNode::from(40);
        node1.change_color();
        node2.change_color();
        let rbt = RedBlackTree {
            size: 3,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 20, 
                left: Some(Box::new(node1)), 
                right: Some(Box::new(node2)),
            }))
        };
        (rbt, vec![10, 20, 30, 40], vec![30])
    }
    pub fn gen_data_for_delete_red_without_children() -> (RedBlackTree<i32>, Vec<i32>, Vec<i32>) {
        let rbt = RedBlackTree {
            size: 1,
            root: Some(Box::new(RedBlackNode { color: Color::Black, key: 10, left: None, right: None }))
        };
        (rbt, vec![10, 20], vec![20])
    }
    pub fn gen_data_for_delete_ll() -> (RedBlackTree<i32>, Vec<i32>, Vec<i32>) {
        let rbt = RedBlackTree {
            size: 4,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 5, 
                left: Some(Box::new(RedBlackNode { color: Color::Black, key: 2, left: None, right: None })), 
                right: Some(Box::new(RedBlackNode { color: Color::Black, key: 7, left: Some(Box::new(RedBlackNode::from(6))), right: None }))
            }))
        };
        (rbt, vec![7, 5, 8, 2, 6], vec![8])
    }
    pub fn gen_data_for_delete_rr() -> (RedBlackTree<i32>, Vec<i32>, Vec<i32>) {
        let rbt = RedBlackTree {
            size: 4,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 7, 
                left: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 5, 
                    left: None,
                    right: Some(Box::new(RedBlackNode::from(6))) 
                })), 
                right: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 12, 
                    left: None, 
                    right: None 
                }))
            }))
        };
        (rbt, vec![5, 1, 7, 6, 12], vec![1])
    }
    pub fn gen_data_for_delete_lr() -> (RedBlackTree<i32>, Vec<i32>, Vec<i32>) {
        let rbt = RedBlackTree {
            size: 3,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 6, 
                left: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 5, 
                    left: None, 
                    right: None 
                })), 
                right: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 7, 
                    left: None, 
                    right: None 
                })) 
            }))
        };
        (rbt, vec![7, 5, 8, 6], vec![8])
    }
    pub fn gen_data_for_delete_rl() -> (RedBlackTree<i32>, Vec<i32>, Vec<i32>) {
        let rbt = RedBlackTree {
            size: 3,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 8, 
                left: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 6, 
                    left: None, 
                    right: None 
                })), 
                right: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 12, 
                    left: None, 
                    right: None 
                })) 
            }))
        };
        (rbt, vec![6, 5, 12, 8], vec![5])
    }
    pub fn gen_data_for_delete_black_sbling_and_black_children_and_black_parent() -> (RedBlackTree<i32>, RedBlackTree<i32>, i32) {
        let mut node6 = RedBlackNode::from(6);
        let mut node9 = RedBlackNode::from(9);
        let mut node17 = RedBlackNode::from(17);
        let mut node25 = RedBlackNode::from(25);
        let mut node34 = RedBlackNode::from(34);
        node6.change_color();
        node9.change_color();
        node17.change_color();
        node25.change_color();
        node34.change_color();

        let rbt_test = RedBlackTree {
            size: 9,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 15, 
                left: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 8, 
                    left: Some(Box::new(node6.clone())), 
                    right: Some(Box::new(node9)),
                })), 
                right: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 18, 
                    left: Some(Box::new(node17.clone())), 
                    right: Some(Box::new(RedBlackNode { 
                        color: Color::Red, 
                        key: 27, 
                        left: Some(Box::new(node25.clone())), 
                        right: Some(Box::new(node34.clone()))
                    }))
                })) 
            }))
        };
        node6.change_color();
        let rbt_standard = RedBlackTree {
            size: 8,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 18, 
                left: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 15, 
                    left: Some(Box::new(RedBlackNode { 
                        color: Color::Black, 
                        key: 8, 
                        left: Some(Box::new(node6)), 
                        right: None 
                    })), 
                    right: Some(Box::new(node17)),
                })), 
                right: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 27, 
                    left: Some(Box::new(node25)), 
                    right: Some(Box::new(node34)), 
                })) 
            }))
        };
        (rbt_test, rbt_standard, 9)
    }
    pub fn gen_data_for_delete_black_sbling_and_black_children_and_red_parent() -> (RedBlackTree<i32>, RedBlackTree<i32>, i32) {
        let mut node4 = RedBlackNode::from(4);
        let mut node17 = RedBlackNode::from(17);
        let mut node28 = RedBlackNode::from(28);
        node4.change_color();
        node17.change_color();
        node28.change_color();
        let rbt_test = RedBlackTree {
            size: 5,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 10, 
                left: Some(Box::new(node4.clone())), 
                right: Some(Box::new(RedBlackNode { 
                    color: Color::Red, 
                    key: 25, 
                    left: Some(Box::new(node17.clone())), 
                    right: Some(Box::new(node28)) 
                })) 
            }))
        };
        node17.change_color();
        let rbt_standard = RedBlackTree {
            size: 4,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 10, 
                left: Some(Box::new(node4)), 
                right: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 25, 
                    left: Some(Box::new(node17)), 
                    right: None 
                })) 
            }))
        };
        (rbt_test, rbt_standard, 28)
    }
    pub fn gen_data_for_delete_black_sbling_and_black_children_and_root_parent() -> (RedBlackTree<i32>, RedBlackTree<i32>, i32) {
        let mut node4 = RedBlackNode::from(4);
        let mut node25 = RedBlackNode::from(25);
        node4.change_color();
        node25.change_color();
        let rbt_test = RedBlackTree {
            size: 3,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 10, 
                left: Some(Box::new(node4.clone())), 
                right: Some(Box::new(node25)) 
            }))
        };
        node4.change_color();
        let rbt_standard = RedBlackTree {
            size: 2,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 10, 
                left: Some(Box::new(node4)), 
                right: None 
            }))
        };
        (rbt_test, rbt_standard, 25)
    }
    pub fn gen_data_for_delete_red_sbling() -> (RedBlackTree<i32>, RedBlackTree<i32>, i32) {
        let mut node9 = RedBlackNode::from(9);
        let mut node17 = RedBlackNode::from(17);
        let mut node21 = RedBlackNode::from(21);
        node9.change_color();
        node17.change_color();
        node21.change_color();
        let rbt_test = RedBlackTree {
            size: 5,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 18, 
                left: Some(Box::new(RedBlackNode { 
                    color: Color::Red, 
                    key: 15, 
                    left: Some(Box::new(node9.clone())), 
                    right: Some(Box::new(node17.clone())) 
                })), 
                right: Some(Box::new(node21)) 
            }))
        };
        node17.change_color();
        let rbt_standard = RedBlackTree {
            size: 4,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 15, 
                left: Some(Box::new(node9)), 
                right: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 18, 
                    left: Some(Box::new(node17)), 
                    right: None 
                })) 
            }))
        };
        (rbt_test, rbt_standard, 21)
    }
    pub fn gen_data_for_delete() -> (RedBlackTree<i32>, Vec<RedBlackTree<i32>>, Vec<i32>) {
        let mut rbts = Vec::new();
        let mut node6 = RedBlackNode::from(6);
        let mut node9 = RedBlackNode::from(9);
        let mut node10 = RedBlackNode::from(10);
        let mut node13 = RedBlackNode::from(13);
        let mut node15 = RedBlackNode::from(15);
        let mut node17 = RedBlackNode::from(17);
        let mut node18 = RedBlackNode::from(18);
        let mut node23 = RedBlackNode::from(23);
        let mut node25 = RedBlackNode::from(25);
        let mut node27 = RedBlackNode::from(27);
        let mut node34 = RedBlackNode::from(34);
        let mut node37 = RedBlackNode::from(37);
        node6.change_color();
        node17.change_color();
        let rbt_test = RedBlackTree {
            size: 12,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 15, 
                left: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 9, 
                    left: Some(Box::new(node6.clone())), 
                    right: Some(Box::new(RedBlackNode { 
                        color: Color::Black, 
                        key: 13, 
                        left: Some(Box::new(node10.clone())), 
                        right: None 
                    })) 
                })), 
                right: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 18, 
                    left: Some(Box::new(node17.clone())), 
                    right: Some(Box::new(RedBlackNode { 
                        color: Color::Red, 
                        key: 27, 
                        left: Some(Box::new(RedBlackNode { 
                            color: Color::Black, 
                            key: 23, 
                            left: None, 
                            right: Some(Box::new(node25.clone())) 
                        })), 
                        right: Some(Box::new(RedBlackNode { 
                            color: Color::Black, 
                            key: 34, 
                            left: None, 
                            right: Some(Box::new(node37.clone())) 
                        })) 
                    })) 
                })) 
            }))
        };
        node6.set_color_black();
        node10.set_color_red();
        node17.set_color_black();
        node25.set_color_black();
        node37.set_color_red();
        let rbt = RedBlackTree {
            size: 11,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 15, 
                left: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 9, 
                    left: Some(Box::new(node6.clone())), 
                    right: Some(Box::new(RedBlackNode { 
                        color: Color::Black, 
                        key: 13, 
                        left: Some(Box::new(node10.clone())), 
                        right: None 
                    }))
                })), 
                right: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 23, 
                    left: Some(Box::new(node17.clone())), 
                    right: Some(Box::new(RedBlackNode { color: Color::Red, 
                        key: 27, 
                        left: Some(Box::new(node25.clone())), 
                        right: Some(Box::new(RedBlackNode { 
                            color: Color::Black, 
                            key: 34, 
                            left: None, 
                            right: Some(Box::new(node37.clone())) 
                        })) 
                    })) 
                })) 
            }))
        };
        rbts.push(rbt);
        node6.set_color_black();
        node10.set_color_red();
        node17.set_color_black();
        node27.set_color_black();
        node37.set_color_black();
        let rbt = RedBlackTree {
            size: 10,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 15, 
                left: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 9, 
                    left: Some(Box::new(node6.clone())), 
                    right: Some(Box::new(RedBlackNode { 
                        color: Color::Black, 
                        key: 13, 
                        left: Some(Box::new(node10.clone())), 
                        right: None 
                    })) 
                })), 
                right: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 23, 
                    left: Some(Box::new(node17.clone())), 
                    right: Some(Box::new(RedBlackNode { 
                        color: Color::Red, 
                        key: 34, 
                        left: Some(Box::new(node27.clone())), 
                        right: Some(Box::new(node37.clone())) 
                    })) 
                })) 
            }))
        };
        rbts.push(rbt);
        node6.set_color_black();
        node10.set_color_red();
        node27.set_color_red();
        node37.set_color_black();
        let rbt = RedBlackTree {
            size: 9,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 17, 
                left: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 9, 
                    left: Some(Box::new(node6.clone())), 
                    right: Some(Box::new(RedBlackNode { 
                        color: Color::Black, 
                        key: 13, 
                        left: Some(Box::new(node10.clone())), 
                        right: None 
                    })) 
                })), 
                right: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 34, 
                    left: Some(Box::new(RedBlackNode { 
                        color: Color::Black, 
                        key: 23, 
                        left: None, 
                        right: Some(Box::new(node27.clone())) 
                    })), 
                    right: Some(Box::new(node37.clone())) 
                })) 
            }))
        };
        rbts.push(rbt);
        node9.set_color_black();
        node13.set_color_black();
        let rbt = RedBlackTree {
            size: 8,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 17, 
                left: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 10, 
                    left: Some(Box::new(node9.clone())), 
                    right: Some(Box::new(node13.clone())), 
                })), 
                right: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 34, 
                    left: Some(Box::new(RedBlackNode { 
                        color: Color::Black, 
                        key: 23, 
                        left: None, 
                        right: Some(Box::new(node27.clone())) 
                    })), 
                    right: Some(Box::new(node37.clone())) 
                })) 
            }))
        };
        rbts.push(rbt);
        node9.set_color_red();
        node27.set_color_red();
        node37.set_color_black();
        let rbt = RedBlackTree {
            size: 7,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 17, 
                left: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 10, 
                    left: Some(Box::new(node9.clone())), 
                    right: None, 
                })), 
                right: Some(Box::new(RedBlackNode { 
                    color: Color::Red, 
                    key: 34, 
                    left: Some(Box::new(RedBlackNode { 
                        color: Color::Black, 
                        key: 23, 
                        left: None, 
                        right: Some(Box::new(node27.clone())) 
                    })), 
                    right: Some(Box::new(node37.clone())) 
                })) 
            }))
        };
        rbts.push(rbt);
        node9.set_color_red();
        node23.set_color_black();
        node34.set_color_black();
        let rbt = RedBlackTree {
            size: 6,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 17, 
                left: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 10, 
                    left: Some(Box::new(node9.clone())), 
                    right: None, 
                })), 
                right: Some(Box::new(RedBlackNode { 
                    color: Color::Red, 
                    key: 27, 
                    left: Some(Box::new(RedBlackNode { 
                        color: Color::Black, 
                        key: 23, 
                        left: None, 
                        right: None, 
                    })), 
                    right: Some(Box::new(node34.clone())) 
                })) 
            }))
        };
        rbts.push(rbt);
        node9.set_color_red();
        node23.set_color_red();
        let rbt = RedBlackTree {
            size: 5,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 17, 
                left: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 10, 
                    left: Some(Box::new(node9.clone())), 
                    right: None, 
                })), 
                right: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 34, 
                    left: Some(Box::new(node23.clone())),
                    right: None,
                })) 
            }))
        };
        rbts.push(rbt);
        node34.set_color_black();
        let rbt = RedBlackTree {
            size: 4,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 23, 
                left: Some(Box::new(RedBlackNode { 
                    color: Color::Black, 
                    key: 10, 
                    left: Some(Box::new(node9.clone())), 
                    right: None, 
                })), 
                right: Some(Box::new(node34.clone())),
            }))
        };
        rbts.push(rbt);
        node9.set_color_black();
        node23.set_color_black();
        let rbt = RedBlackTree {
            size: 3,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 10, 
                left: Some(Box::new(node9.clone())), 
                right: Some(Box::new(node23.clone())),
            }))
        };
        rbts.push(rbt);
        node23.set_color_red();
        let rbt = RedBlackTree {
            size: 2,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 10, 
                left: None, 
                right: Some(Box::new(node23.clone())),
            }))
        };
        rbts.push(rbt);
        node23.set_color_black();
        let rbt = RedBlackTree {
            size: 1,
            root: Some(Box::new(RedBlackNode { 
                color: Color::Black, 
                key: 23, 
                left: None, 
                right: None,
            }))
        };
        rbts.push(rbt);
        rbts.push(RedBlackTree::new());
        (rbt_test, rbts, vec![18, 25, 15, 6, 13, 37, 27, 17, 34, 9, 10, 23])
    }
}
