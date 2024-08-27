

pub mod singly_link_list;
pub mod doubly_link_list;

pub trait LinkListTrait<T: PartialEq> {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn insert(&mut self, index: usize, data: T) -> bool;
    fn remove(&mut self, index: usize) -> Option<T>;
    fn contains(&self, target: &T) -> Option<usize>;
    fn clear(&mut self);
}


#[cfg(test)]
mod test_singly_link_list {
    use crate::datastruct::link_list::singly_link_list::LinkListTrait;
    use crate::datastruct::link_list::singly_link_list::SinglyLinkList;
    use crate::node::Node;
    use std::collections::HashMap;
    fn mix_nodes_to_singly_link_list<T: PartialEq + Clone>(mut nodes: Vec<Node<T>>) -> HashMap<String, SinglyLinkList<T>>{
        fn to_string(vec: Vec<usize>) -> String {
            let mut result = String::new();
            if vec.len() == 0 {
                return result;
            } else {
                result += &vec[0].to_string();
            }
            for i in 1..vec.len() {
                result = result + "-" + &vec[i].to_string();
            }
            result
        }
        fn backtracking<T: PartialEq + Clone>(map: &mut HashMap<String, (usize, Box<Node<T>>)>, nodes: &Vec<Node<T>>, index: usize, node: Option<Box<Node<T>>>, path: &mut Vec<usize>) {
            if let Some(n) = &node {
                let len = path.len();
                let mut path: Vec<usize> = path.clone().into_iter().map(|x| nodes.len() - x).collect();
                path.reverse();
                map.insert(to_string(path), (len, n.clone()));
            } 
            for i in index..nodes.len() {
                let mut head = nodes[i].clone();
                head.next = node.clone();
                path.push(i);
                backtracking(map, nodes, i + 1, Some(Box::new(head)), path);
                path.pop();
            }
        }
       
        let mut node_map = HashMap::<String, (usize, Box<Node<T>>)>::new();
        let mut path = Vec::<usize>::new();
        nodes.reverse();
        backtracking(&mut node_map, &nodes, 0, None, &mut path);
        let mut linklist_map = HashMap::<String, SinglyLinkList<T>>::new();
        for (name, (len, head)) in node_map {
            linklist_map.insert(name, SinglyLinkList { 
                len, 
                head: Some(head),
                is_recursive: false,
            });
        }
        linklist_map
    }
    #[test]
    fn test_init() {
        let example: SinglyLinkList<i32> = SinglyLinkList::<i32> {
            len: 0,
            head: None,
            is_recursive: false,
        };
        let linklist = SinglyLinkList::<i32>::new();
        assert_eq!(example, linklist);
    }
    #[test]
    fn test_empty() {
         let example1: SinglyLinkList<i32> = SinglyLinkList::<i32> {
            len: 0,
            head: None,
            is_recursive: false,
        };
        let mut example2 = SinglyLinkList::<i32>::new();
        assert_eq!(example1.is_empty(), true);
        assert_eq!(example2.is_empty(), true);
        example2.insert(0, 0);
        assert_eq!(example2.is_empty(), false);
        example2.remove(0);
        assert_eq!(example2.is_empty(), true);
    }
    #[test]
    fn test_insert() {
        let linklist_map = mix_nodes_to_singly_link_list(vec![
            Node::<i32>::from(1),
            Node::from(2),
            Node::from(3),
            Node::from(4),
        ]);
        let mut linklist_test = SinglyLinkList::<i32>::new();
        linklist_test.insert(0, 2);
        assert_eq!(linklist_test.insert(2, 1), false);
        assert_eq!(linklist_test, linklist_map["2"]);
        linklist_test.insert(0, 1);
        assert_eq!(linklist_test, linklist_map["1-2"]);
        linklist_test.insert(2, 4);
        assert_eq!(linklist_test, linklist_map["1-2-4"]);
        linklist_test.insert(2, 3);
        assert_eq!(linklist_test, linklist_map["1-2-3-4"]);
        

        let mut linklist_test = SinglyLinkList::<i32>::new();
        linklist_test.enable_recursive_algorithm();
        linklist_test.insert(0, 2);
        assert_eq!(linklist_test.insert(2, 1), false);
        assert_eq!(linklist_test, linklist_map["2"]);
        linklist_test.insert(0, 1);
        assert_eq!(linklist_test, linklist_map["1-2"]);
        linklist_test.insert(2, 4);
        assert_eq!(linklist_test, linklist_map["1-2-4"]);
        linklist_test.insert(2, 3);
        assert_eq!(linklist_test, linklist_map["1-2-3-4"]);

    }
    #[test]
    fn test_remove() {
        let linklist_map = mix_nodes_to_singly_link_list(vec![
            Node::<i32>::from(1),
            Node::from(2),
            Node::from(3),
            Node::from(4),
        ]);
        let mut linklist_test = SinglyLinkList::<i32>::new();
        linklist_test.insert(0, 2);
        assert_eq!(linklist_test.insert(2, 1), false);
        assert_eq!(linklist_test, linklist_map["2"]);
        linklist_test.insert(0, 1);
        assert_eq!(linklist_test, linklist_map["1-2"]);
        linklist_test.insert(2, 4);
        assert_eq!(linklist_test, linklist_map["1-2-4"]);
        linklist_test.insert(2, 3);
        assert_eq!(linklist_test, linklist_map["1-2-3-4"]);
        
        let mut linklist_test_iterative = linklist_test.clone();
        assert_eq!(linklist_test_iterative.remove(5), None);
        assert_eq!(linklist_test_iterative.remove(2), Some(3));
        assert_eq!(linklist_test_iterative, linklist_map["1-2-4"]);
        assert_eq!(linklist_test_iterative.remove(2), Some(4));
        assert_eq!(linklist_test_iterative, linklist_map["1-2"]);
        assert_eq!(linklist_test_iterative.remove(0), Some(1));
        assert_eq!(linklist_test_iterative, linklist_map["2"]);
        assert_eq!(linklist_test_iterative.remove(0), Some(2));
        assert_eq!(linklist_test_iterative, SinglyLinkList::new());
        assert_eq!(linklist_test_iterative.remove(0), None);

        let mut linklist_test_recursive = linklist_test;
        linklist_test_recursive.enable_recursive_algorithm();
        assert_eq!(linklist_test_recursive.remove(5), None);
        assert_eq!(linklist_test_recursive.remove(2), Some(3));
        assert_eq!(linklist_test_recursive, linklist_map["1-2-4"]);
        assert_eq!(linklist_test_recursive.remove(2), Some(4));
        assert_eq!(linklist_test_recursive, linklist_map["1-2"]);
        assert_eq!(linklist_test_recursive.remove(0), Some(1));
        assert_eq!(linklist_test_recursive, linklist_map["2"]);
        assert_eq!(linklist_test_recursive.remove(0), Some(2));
        assert_eq!(linklist_test_recursive, SinglyLinkList::new());
        assert_eq!(linklist_test_recursive.remove(0), None);

    }
    #[test]
    fn test_len() {
        let mut linklist_test = SinglyLinkList::<String>::new();
        assert_eq!(linklist_test.len(), 0);
        linklist_test.insert(0, "asdaf".to_string());
        assert_eq!(linklist_test.len(), 1);
        linklist_test.insert(1, "qwe".to_string());
        assert_eq!(linklist_test.len(), 2);
        linklist_test.remove(1);
        assert_eq!(linklist_test.len(), 1);
        linklist_test.remove(0);
        assert_eq!(linklist_test.len(), 0);
        linklist_test.remove(0);
        assert_eq!(linklist_test.len(), 0);
    }
    #[test]
    fn test_contains() {
         let mut linklist_test = SinglyLinkList::<i32>::new();
        linklist_test.insert(0, 5);  
        linklist_test.insert(0, i32::max_value());
        linklist_test.insert(0, i32::min_value());
        assert_eq!(linklist_test.contains(&5), Some(2));
        assert_eq!(linklist_test.contains(&i32::min_value()), Some(0));
        assert_eq!(linklist_test.contains(&i32::max_value()), Some(1));
        assert_eq!(linklist_test.contains(&10), None);
    }
    #[test]
    fn test_clear() {
        let mut linklist_test = SinglyLinkList::<i32>::new();
        linklist_test.clear();
        assert_eq!(linklist_test, SinglyLinkList::new());
        linklist_test.insert(0, 1);
        linklist_test.insert(0, 2);
        linklist_test.clear();
        assert_eq!(linklist_test, SinglyLinkList::new());
    }
}

#[cfg(test)]
mod test_doubly_link_list {
    use std::collections::linked_list;

    use super::doubly_link_list::DoublyLinkList;
    use super::LinkListTrait;
    #[test]
    fn test_is_empty() {
        let linklist_test = DoublyLinkList::<i32>::new();
        assert_eq!(linklist_test.is_empty(), true);
    }
    #[test]
    fn test_push_pop() {
        let mut linklist = DoublyLinkList::<i32>::new();
        for i in 0..5 {
            linklist.push_back(i);
        }
        for i in (0..5).rev() {
            assert_eq!(linklist.pop_back(), Some(i));
        }
        assert_eq!(linklist.pop_back(), None);
        assert_eq!(linklist.pop_front(), None);
        for i in 0..5 {
            linklist.push_front(i);
        }
        for i in (0..5).rev() {
            assert_eq!(linklist.pop_front(), Some(i));
        }
        linklist.push_front(2);
        linklist.push_front(1);
        linklist.push_back(3);
        assert_eq!(linklist.pop_back(), Some(3));
        assert_eq!(linklist.pop_front(), Some(1));
    }
    #[test]
    fn test_insert() {
        let mut linklist = DoublyLinkList::new();
        for i in (1..6).step_by(2) {
            linklist.push_back(i); 
        }
        // linklist = [1, 3, 5]
        assert_eq!(linklist.insert(0, 0), true);
        assert_eq!(linklist.insert(2, 2), true);
        assert_eq!(linklist.insert(4, 4), true);
        assert_eq!(linklist.insert(6, 6), true);
        assert_eq!(linklist.insert(8, 8), false);
        // linklist = [0, 1, 2, 3, 4, 5, 6]
        for i in 0..7 {
            assert_eq!(linklist.pop_front(), Some(i));
        }
    }
    #[test]
    fn test_remove() {
        let mut linked_list = DoublyLinkList::new();
        for i in 0..5 {
            linked_list.push_back(i);
        }
        // linklist = [0, 1, 2, 3, 4]
        assert_eq!(linked_list.remove(0), Some(0));
        // linklist = [1, 2, 3, 4]
        assert_eq!(linked_list.remove(1), Some(2));
        // linklist = [1, 3, 4]
        assert_eq!(linked_list.remove(2), Some(4));
        // linklist = [1, 3]
        assert_eq!(linked_list.remove(3), None);
    }
    #[test]
    fn test_countains() {
        let mut linked_list = DoublyLinkList::new();
        for i in 0..5 {
            linked_list.push_back(i);
        }
        assert_eq!(linked_list.contains(&0), Some(0));
        assert_eq!(linked_list.contains(&2), Some(2));
        assert_eq!(linked_list.contains(&4), Some(4));
        assert_eq!(linked_list.contains(&5), None);
    }
    #[test]
    fn test_clear() {
        let mut linked_list = DoublyLinkList::new();
        for i in 0..5 {
            linked_list.push_front(i);
        }
        linked_list.clear();
        assert_eq!(linked_list, DoublyLinkList::new());
    }
}
