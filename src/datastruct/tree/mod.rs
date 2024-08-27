pub mod binary_tree;
pub mod binary_search_tree;
pub mod balanced_binary_tree;
pub mod b_tree;
pub mod red_black_tree;

use std::rc::Rc;
use std::cell::RefCell;


pub trait TreeTrait<T: PartialOrd + Clone> {
   fn pre_order_traversal(&self) -> Vec<T> ;
   fn in_order_traversal(&self) -> Vec<T>;
   fn post_order_traversal(&self) -> Vec<T>;
   fn level_order_traversal(&self) -> Vec<Vec<T>>;
   fn depth(&self) -> usize;
   fn contains(&self, data: &T) -> bool;
   fn size(&self) -> usize;
}

#[cfg(test)]
mod test_binary_search_tree {
    use std::rc::Rc;
    use std::cell::RefCell;

    use crate::datastruct::tree::{TreeTrait, binary_search_tree::BinarySearchTree};
    use crate::node::TreeNode;
    #[test]
    fn test_append() {
        let mut bst = BinarySearchTree::<i32>::new();
        let mut bst_recurive = BinarySearchTree::<i32>::new();
        bst_recurive.enable_recursive_algorithm();
        assert_eq!(bst, BinarySearchTree::new()); 
        bst.append(5); 
        bst_recurive.append(5);
        let example = BinarySearchTree {
            size: 1,
            root: Some(Rc::new(RefCell::new(TreeNode { 
                data: 5, 
                left: None, 
                right: None 
            }))),
            is_recursive: false,
        };
        assert_eq!(bst, example);
        assert_eq!(bst_recurive, example);
        bst.append(1);
        bst_recurive.append(1);
        let emaple = BinarySearchTree {
            size: 2,
            root: Some(Rc::new(RefCell::new(TreeNode { 
                data: 5, 
                left: Some(Rc::new(RefCell::new(TreeNode { 
                    data: 1, 
                    left: None, 
                    right: None,
                }))), 
                right: None 
            }))),
            is_recursive: false,
        };
        assert_eq!(bst, emaple);
        assert_eq!(bst_recurive, emaple);
        bst.append(6);
        bst_recurive.append(6);
        let emaple = BinarySearchTree {
            size: 3,
            root: Some(Rc::new(RefCell::new(TreeNode { 
                data: 5, 
                left: Some(Rc::new(RefCell::new(TreeNode { 
                    data: 1, 
                    left: None, 
                    right: None,
                }))), 
                right: Some(Rc::new(RefCell::new(TreeNode { 
                    data: 6, 
                    left: None, 
                    right: None 
                }))),
            }))),
            is_recursive: false,
        };
        assert_eq!(bst, emaple);
        assert_eq!(bst_recurive, emaple);
        bst.append(4);
        bst_recurive.append(4);
        let emaple = BinarySearchTree {
            size: 4,
            root: Some(Rc::new(RefCell::new(TreeNode { 
                data: 5, 
                left: Some(Rc::new(RefCell::new(TreeNode { 
                    data: 1, 
                    left: None, 
                    right: Some(Rc::new(RefCell::new(TreeNode { 
                        data: 4, 
                        left: None, 
                        right: None 
                    }))),
                }))), 
                right: Some(Rc::new(RefCell::new(TreeNode { 
                    data: 6, 
                    left: None, 
                    right: None 
                }))),
            }))),
            is_recursive: false,
        };
        assert_eq!(bst, emaple);
        assert_eq!(bst_recurive, emaple);
    }
    #[test]
    fn test_pre_order_traversal() {
        let mut bst = BinarySearchTree::<i32>::new();
        let nums = vec![7, 4, 1, 5, 6, 8, 9];
        let pre_order_traversal = vec![7, 4, 1, 5, 6, 8, 9];
        for n in nums {
            bst.append(n);
        }
        assert_eq!(bst.pre_order_traversal(), pre_order_traversal);
        bst.enable_recursive_algorithm();
        assert_eq!(bst.pre_order_traversal(), pre_order_traversal);
    }
    #[test]
    fn test_in_order_traversal() {
        let mut bst = BinarySearchTree::<i32>::new();
        let nums = vec![7, 4, 1, 5, 6, 8, 9];
        let in_order_traversal = vec![1, 4, 5, 6, 7, 8, 9];
        for n in nums {
            bst.append(n);
        }
        assert_eq!(bst.in_order_traversal(), in_order_traversal);
        bst.enable_recursive_algorithm();
        assert_eq!(bst.in_order_traversal(), in_order_traversal);
    }
    #[test]
    fn test_post_order_traversal() {
        let mut bst = BinarySearchTree::<i32>::new();
        let nums = vec![7, 4, 1, 5, 6, 8, 9];
        let post_order_traversal = vec![1, 6, 5, 4, 9, 8, 7];
        for n in nums {
            bst.append(n);
        }
        assert_eq!(bst.post_order_traversal(), post_order_traversal);
        bst.enable_recursive_algorithm();
        assert_eq!(bst.post_order_traversal(), post_order_traversal);
        use rand::{thread_rng, Rng};
        for _ in 0..10 {
            let mut bst = BinarySearchTree::<i32>::new();
            for _ in 0..100 {
                bst.append(thread_rng().gen());    
            }
            let iterative_post_order = bst.post_order_traversal();
            bst.enable_recursive_algorithm();
            let recursive_post_order = bst.post_order_traversal();
            assert_eq!(iterative_post_order, recursive_post_order);
        }
    }
    #[test]
    fn test_level_order_traversal() {
        let mut bst = BinarySearchTree::<i32>::new();
        let nums = vec![7, 4, 1, 5, 6, 8, 9];
        let level_order_traversal = vec![vec![7], vec![4, 8], vec![1, 5, 9], vec![6]];
        for n in nums {
            bst.append(n);
        }
        assert_eq!(bst.level_order_traversal(), level_order_traversal);
    }
    #[test]
    fn test_depth() {
        let mut bst = BinarySearchTree::<i32>::new();
        let nums = vec![7, 4, 1, 5, 6, 8, 9];
        for n in nums {
            bst.append(n);
        }
        assert_eq!(bst.depth(), 4);
        bst.enable_recursive_algorithm();
        assert_eq!(bst.depth(), 4);
    }
    #[test]
    fn test_contains() {
        let mut bst = BinarySearchTree::<i32>::new();
        let nums = vec![7, 4, 1, 5, 6, 8, 9];
        for n in nums {
            bst.append(n);
        }
        assert_eq!(bst.contains(&5), true);
        assert_eq!(bst.contains(&9), true);
        assert_eq!(bst.contains(&3), false);
        bst.enable_recursive_algorithm();
        assert_eq!(bst.contains(&5), true);
        assert_eq!(bst.contains(&9), true);
        assert_eq!(bst.contains(&3), false);
    }
    #[test]
    fn test_delete() {
        let mut bst = BinarySearchTree::<i32>::new();
        bst.append(5);
        assert_eq!(bst.delete(&5), Some(5));
        assert_eq!(bst, BinarySearchTree::new());
        assert_eq!(bst.delete(&10), None);
        bst.append(5);
        bst.append(4);
        bst.delete(&4);
        assert_eq!(bst, BinarySearchTree {
            size: 1,
            root: Some(Rc::new(RefCell::new(TreeNode { 
                data: 5, 
                left: None, 
                right: None 
            }))),
            is_recursive: false,
        });
        let mut bst = BinarySearchTree::new();
        bst.append(6);
        bst.append(3);
        bst.append(8);
        bst.append(5);
        bst.append(4);
        assert_eq!(bst.delete(&6), Some(6));
        assert_eq!(bst, BinarySearchTree {
            size: 4,
            root: Some(Rc::new(RefCell::new(TreeNode { 
                data: 5, 
                left: Some(Rc::new(RefCell::new(TreeNode { 
                    data: 3, 
                    left: None, 
                    right: Some(Rc::new(RefCell::new(TreeNode::from(4)))), 
                }))), 
                right: Some(Rc::new(RefCell::new(TreeNode::from(8)))),
            }))),
            is_recursive: false,
        });
    }
}

#[cfg(test)]
mod test_balanced_binary_tree {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::datastruct::tree::TreeTrait;
    use crate::datastruct::tree::balanced_binary_tree::BalancedBinaryTree;
    use crate::node::{self, BalancedTreeNode};
    fn get_nodes(count: usize) -> Vec<Rc<RefCell<BalancedTreeNode<i32>>>> {
        let mut nodes = Vec::new();
        for i in 0..count {
            nodes.push(Rc::new(RefCell::new(BalancedTreeNode::from(i as i32))));
        }
        nodes
    }
    #[test]
    fn test_append_base() {
        let mut avl = BalancedBinaryTree::new();
        let nodes = get_nodes(4);
        let mut standard = BalancedBinaryTree::new();
        avl.append(1);
        standard.size = 1;
        standard.root = Some(nodes[1].clone());
        assert_eq!(avl, standard);
        avl.append(0);
        let node = nodes[1].clone();
        node.borrow_mut().height = 2;
        node.borrow_mut().left = Some(nodes[0].clone());
        standard.size = 2;
        standard.root = Some(node);
        assert_eq!(avl, standard);
        avl.append(2);
        let node = nodes[1].clone();
        node.borrow_mut().height = 2;
        node.borrow_mut().left = Some(nodes[0].clone());
        node.borrow_mut().right = Some(nodes[2].clone());
        standard.size = 3;
        standard.root = Some(node);
        assert_eq!(avl, standard);
        avl.append(3);
        let node2 = nodes[2].clone();
        node2.borrow_mut().right = Some(nodes[3].clone());
        node2.borrow_mut().height = 2;
        let node = nodes[1].clone();
        node.borrow_mut().height = 3;
        node.borrow_mut().left = Some(nodes[0].clone());
        node.borrow_mut().right = Some(node2);
        standard.size = 4;
        standard.root = Some(node);
        assert_eq!(avl, standard);
        let mut avl = BalancedBinaryTree::new();
        avl.append(3);
        avl.append(0);
        avl.append(2);
        avl.append(1);
        let nodes = get_nodes(4);
        let node0 = nodes[0].clone();
        node0.borrow_mut().height = 2;
        node0.borrow_mut().right = Some(nodes[1].clone());
        let node2 = nodes[2].clone();
        node2.borrow_mut().height = 3;
        node2.borrow_mut().left = Some(node0);
        node2.borrow_mut().right = Some(nodes[3].clone());
        let standard = BalancedBinaryTree {
            size: 4,
            root: Some(node2),
        };
        assert_eq!(avl, standard);

    }
    #[test]
    fn test_append_ll() {
        let mut avl = BalancedBinaryTree::new();
        avl.append(2);
        avl.append(1);
        avl.append(0);
        let nodes = get_nodes(3);
        let node = nodes[1].clone();
        node.borrow_mut().height = 2;
        node.borrow_mut().left = Some(nodes[0].clone());
        node.borrow_mut().right = Some(nodes[2].clone());
        let standard = BalancedBinaryTree {
            size: 3,
            root: Some(node),
        };
        assert_eq!(avl, standard);
    }
    #[test]
    fn test_append_rr() {
        let mut avl = BalancedBinaryTree::new();
        avl.append(0);
        avl.append(1);
        avl.append(2);
        let nodes = get_nodes(3);
        let node = nodes[1].clone();
        node.borrow_mut().height = 2;
        node.borrow_mut().left = Some(nodes[0].clone());
        node.borrow_mut().right = Some(nodes[2].clone());
        let standard = BalancedBinaryTree {
            size: 3,
            root: Some(node),
        };
        assert_eq!(avl, standard);
    }
    #[test]
    fn test_append_lr() {
        let mut avl = BalancedBinaryTree::new();
        avl.append(4);
        avl.append(1);
        avl.append(5);
        avl.append(0);
        avl.append(3);
        avl.append(2);
        let nodes = get_nodes(6);
        let node1 = nodes[1].clone();
        node1.borrow_mut().height = 2;
        node1.borrow_mut().left = Some(nodes[0].clone());
        node1.borrow_mut().right = Some(nodes[2].clone());
        let node4 = nodes[4].clone();
        node4.borrow_mut().height = 2;
        node4.borrow_mut().right = Some(nodes[5].clone());
        let node3 = nodes[3].clone();
        node3.borrow_mut().height = 3;
        node3.borrow_mut().left = Some(node1);
        node3.borrow_mut().right = Some(node4);
        let standard = BalancedBinaryTree {
            size: 6,
            root: Some(node3),
        };
        assert_eq!(avl, standard);
    }
    #[test]
    fn test_append_rl() {
        let mut avl = BalancedBinaryTree::new();
        avl.append(1);
        avl.append(0);
        avl.append(4);
        avl.append(2);
        avl.append(5);
        avl.append(3);
        let nodes = get_nodes(6);
        let node1 = nodes[1].clone();
        node1.borrow_mut().height = 2;
        node1.borrow_mut().left = Some(nodes[0].clone());
        let node4 = nodes[4].clone();
        node4.borrow_mut().height = 2;
        node4.borrow_mut().left = Some(nodes[3].clone());
        node4.borrow_mut().right = Some(nodes[5].clone());
        let node2 = nodes[2].clone();
        node2.borrow_mut().height = 3;
        node2.borrow_mut().left = Some(node1);
        node2.borrow_mut().right = Some(node4);
        let standard = BalancedBinaryTree {
            size: 6,
            root: Some(node2),
        };
        assert_eq!(avl, standard);
    }
    #[test]
    fn test_delete_ll() {
        let mut avl = BalancedBinaryTree::new();
        avl.append(2);
        avl.append(1);
        avl.append(3);
        avl.append(0);
        assert_eq!(avl.delete(&3), Some(3));
        let nodes = get_nodes(4);
        let node = nodes[1].clone();
        node.borrow_mut().height = 2;
        node.borrow_mut().left = Some(nodes[0].clone());
        node.borrow_mut().right = Some(nodes[2].clone());
        assert_eq!(avl, BalancedBinaryTree {
            size: 3,
            root: Some(node),
        });
    }
    #[test]
    fn test_delete_rr() {
        let mut avl = BalancedBinaryTree::new();
        avl.append(1);
        avl.append(0);
        avl.append(2);
        avl.append(3);
        assert_eq!(avl.delete(&0), Some(0));
        let nodes = get_nodes(4);
        let node = nodes[2].clone();
        node.borrow_mut().height = 2;
        node.borrow_mut().left = Some(nodes[1].clone());
        node.borrow_mut().right = Some(nodes[3].clone());
        assert_eq!(avl, BalancedBinaryTree {
            size: 3,
            root: Some(node),
        });
    }
    #[test]
     fn test_delete_lr() {
        let mut avl = BalancedBinaryTree::new();
        avl.append(4);
        avl.append(1);
        avl.append(5);
        avl.append(0);
        avl.append(3);
        avl.append(6);
        avl.append(2);
        assert_eq!(avl.delete(&6), Some(6));
        let nodes = get_nodes(6);
        let node1 = nodes[1].clone();
        node1.borrow_mut().height = 2;
        node1.borrow_mut().left = Some(nodes[0].clone());
        node1.borrow_mut().right = Some(nodes[2].clone());
        let node4 = nodes[4].clone();
        node4.borrow_mut().height = 2;
        node4.borrow_mut().right = Some(nodes[5].clone());
        let node3 = nodes[3].clone();
        node3.borrow_mut().height = 3;
        node3.borrow_mut().left = Some(node1);
        node3.borrow_mut().right = Some(node4);
        let standard = BalancedBinaryTree {
            size: 6,
            root: Some(node3),
        };
        assert_eq!(avl, standard);
    }   
    #[test]
    fn test_delete_rl() {
        let mut avl = BalancedBinaryTree::new();
        avl.append(2);
        avl.append(1);
        avl.append(5);
        avl.append(0);
        avl.append(3);
        avl.append(6);
        avl.append(4);
        assert_eq!(avl.delete(&0), Some(0));
        let nodes = get_nodes(7);
        let node2 = nodes[2].clone();
        node2.borrow_mut().height = 2;
        node2.borrow_mut().left = Some(nodes[1].clone());
        let node5 = nodes[5].clone();
        node5.borrow_mut().height = 2;
        node5.borrow_mut().left = Some(nodes[4].clone());
        node5.borrow_mut().right = Some(nodes[6].clone());
        let node3 = nodes[3].clone();
        node3.borrow_mut().height = 3;
        node3.borrow_mut().left = Some(node2);
        node3.borrow_mut().right = Some(node5);
        let standard = BalancedBinaryTree {
            size: 6,
            root: Some(node3),
        };
        assert_eq!(avl, standard);
    }
}

#[cfg(test)]
mod test_b_tree {
    use std::char::MAX;

    use super::b_tree::BTree;
    use crate::node::BTreeNode;
    fn gen_node<T: PartialOrd + std::fmt::Debug + Default + Clone>(degree: usize, mut keys: Vec<T>) -> BTreeNode<T> {
        let count = keys.len();
        keys.resize(degree, T::default());
        BTreeNode { 
            keys, 
            key_count: count, 
            next: vec![None; degree + 1],
        }
    }
    #[test]
    fn test_append_to_root_without_overflow() {
        let mut bt = BTree::new(4);
        bt.append(4);
        bt.append(1);
        bt.append(3);
        let standard = BTree {
            size: 3,
            degree: 4,
            root: Some(Box::new(BTreeNode { 
                keys: vec![1, 3, 4, i32::default()], 
                key_count: 3, 
                next: vec![None; 4 + 1] 
            })),
        };
        assert_eq!(bt, standard);
    }
    #[test]
    fn test_append_to_root_with_upoverflow() {
        let mut bt = BTree::new(4);
        bt.append(1);
        bt.append(2);
        bt.append(3);
        bt.append(4);
        let standard = BTree {
            size: 4,
            degree: 4,
            root: Some(Box::new(BTreeNode { 
                keys: vec![2, i32::default(), i32::default(), i32::default()], 
                key_count: 1, 
                next: vec![
                    Some(Box::new(BTreeNode::from(4, 1))),
                    Some(Box::new(BTreeNode { 
                        keys: vec![3, 4, i32::default(), i32::default()], 
                        key_count: 2, 
                        next: vec![None; 4 + 1],
                    })),
                    None,
                    None,
                ],
            })),
        };
        assert_eq!(bt, standard);
    }
    #[test]
    fn test_append_to_leaf_without_overflow() {
        let mut bt = BTree::new(4);
        bt.append(10);
        bt.append(20);
        bt.append(30);
        bt.append(40);
        bt.append(5);
        bt.append(6);
        bt.append(35);
        let standard = BTree {
            size: 7,
            root: Some(Box::new(BTreeNode { 
                keys: vec![20, i32::default(), i32::default(), i32::default()], 
                key_count: 1, 
                next: vec![
                    Some(Box::new(BTreeNode { 
                        keys: vec![5, 6, 10, i32::default()], 
                        key_count: 3, 
                        next: vec![None; 4 + 1], 
                    })),
                    Some(Box::new(BTreeNode { 
                        keys: vec![30, 35, 40, i32::default()], 
                        key_count: 3, 
                        next: vec![None; 4 + 1] 
                    })),
                    None,
                    None,
                    None,
                ], 
            })),
            degree: 4,
        };
        assert_eq!(bt, standard);
    }
    #[test]
    fn test_append_to_leaf_with_once_upoverflow() {
        let mut bt = BTree::new(4);
        bt.append(10);
        bt.append(20);
        bt.append(30);
        bt.append(40);
        bt.append(5);
        bt.append(6);
        bt.append(35);
        bt.append(15);
        let standard = BTree {
            size: 8,
            root: Some(Box::new(BTreeNode { 
                keys: vec![6, 20, i32::default(), i32::default()], 
                key_count: 2, 
                next: vec![
                    Some(Box::new(BTreeNode::from(4, 5))),
                    Some(Box::new(BTreeNode { 
                        keys: vec![10, 15, i32::default(), i32::default()], 
                        key_count: 2, 
                        next: vec![None; 4 + 1], 
                    })),
                    Some(Box::new(BTreeNode { 
                        keys: vec![30, 35, 40, i32::default()], 
                        key_count: 3, 
                        next: vec![None; 4 + 1] 
                    })),
                    None,
                    None,
                ], 
            })),
            degree: 4,
        };
        assert_eq!(bt, standard);
    }
    #[test]
    fn test_append_with_multiple_upoverflow() {
        let mut bt = BTree::new(4);
        bt.append(100);
        bt.append(200);
        bt.append(300);
        bt.append(400);
        bt.append(50);
        bt.append(150);
        bt.append(190);
        bt.append(25);
        bt.append(75);
        bt.append(90);
        bt.append(10);
        bt.append(35);
        bt.append(0);
        let standard = BTree {
            size: 13,
            degree: 4,
            root: Some(Box::new(BTreeNode { 
                keys: vec![50, i32::default(), i32::default(), i32::default()], 
                key_count: 1, 
                next: vec![
                    Some(Box::new(BTreeNode { 
                        keys: vec![10, i32::default(), i32::default(), i32::default()], 
                        key_count: 1, 
                        next: vec![
                            Some(Box::new(BTreeNode::from(4, 0))), 
                            Some(Box::new(gen_node(4, vec![25, 35]))),                            
                            None, 
                            None, 
                            None
                        ], 
                    })),
                    Some(Box::new(BTreeNode { 
                        keys: vec![100, 200, i32::default(), i32::default()], 
                        key_count: 2, 
                        next: vec![
                            Some(Box::new(gen_node(4, vec![75, 90]))),
                            Some(Box::new(gen_node(4, vec![150, 190]))),
                            Some(Box::new(gen_node(4, vec![300, 400]))),
                        ], 
                    })),
                    None,
                    None,
                    None,
                ],
            })),
        };
        assert_eq!(bt, standard);
    }
    #[test]
    fn test_delete_without_underoverflow() {
        let mut bt = BTree::new(4);
        bt.append(10);
        bt.append(20);
        bt.append(30);
        bt.append(40);
        bt.append(15);
        bt.append(18);
        bt.append(50);
        let standard = BTree {
            size: 7,
            degree: 4,
            root: Some(Box::new(BTreeNode { 
                keys: vec![20, i32::default(), i32::default(), i32::default()], 
                key_count: 1, 
                next: vec![
                    Some(Box::new(gen_node(4, vec![10, 15, 18]))),
                    Some(Box::new(gen_node(4, vec![30, 40, 50]))),
                    None,
                    None,
                    None,
                ],
            })),
        };
        assert_eq!(bt, standard);
        assert_eq!(bt.delete(&14), None);
        assert_eq!(bt.delete(&15), Some(15));
        assert_eq!(bt.delete(&50), Some(50));
        bt.append(19);
        assert_eq!(bt.delete(&20), Some(20));
        let standard = BTree {
            size: 5,
            degree: 4,
            root: Some(Box::new(BTreeNode { 
                keys: vec![19, i32::default(), i32::default(), i32::default()], 
                key_count: 1, 
                next: vec![
                    Some(Box::new(gen_node(4, vec![10, 18]))),
                    Some(Box::new(gen_node(4, vec![30, 40]))),
                    None,
                    None,
                    None,
                ],
            })),
        };
        assert_eq!(bt, standard);
    }
    #[test]
    fn test_delete_to_leaf_with_once_underoverflow() {
        let mut bt = BTree::new(4);
        bt.append(1);
        bt.append(2);
        bt.append(3);
        bt.append(4);
        bt.delete(&1);
        let standard = BTree {
            size: 3,
            degree: 4,
            root: Some(Box::new(BTreeNode { 
                keys: vec![3, i32::default(), i32::default(), i32::default()], 
                key_count: 1, 
                next: vec![
                    Some(Box::new(BTreeNode::from(4, 2))),
                    Some(Box::new(BTreeNode::from(4, 4))),
                    None,
                    None,
                    None
                ], 
            }))
        };
        assert_eq!(bt, standard);
    }
    #[test]
    fn test_delete_adjust_by_merge() {
        let mut bt = BTree::new(4);
        bt.append(1);
        bt.append(2);
        bt.append(3);
        bt.append(4);
        bt.append(5);
        bt.append(6);
        bt.delete(&6);
        bt.delete(&3);
        let standard = BTree {
            size: 4,
            degree: 4,
            root: Some(Box::new(BTreeNode { 
                keys: vec![4, i32::default(), i32::default(), i32::default()], 
                key_count: 1, 
                next: vec![
                    Some(Box::new(gen_node(4, vec![1, 2]))),
                    Some(Box::new(BTreeNode::from(4, 5))),
                    None
                ], 
            }))
        };
        assert_eq!(bt, standard);
    }
}

#[cfg(test)]
mod test_red_black_tree {
    use super::red_black_tree::RedBlackTree;

    #[test]
    fn test_append_base() {
        let (standard, nums) = RedBlackTree::gen_data_for_test_append_base() ;
        let mut rbt = RedBlackTree::new();
        for n in nums {
            rbt.append(n);
        }
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_append_change_color_once() {
        let (standard, nums) = RedBlackTree::gen_data_for_test_append_change_color_once() ;
        let mut rbt = RedBlackTree::new();
        for n in nums {
            rbt.append(n);
        }
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_append_ll() {
        let (standard, nums) = RedBlackTree::gen_data_for_test_append_ll() ;
        let mut rbt = RedBlackTree::new();
        for n in nums {
            rbt.append(n);
        }
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_append_rr() {
        let (standard, nums) = RedBlackTree::gen_data_for_test_append_rr() ;
        let mut rbt = RedBlackTree::new();
        for n in nums {
            rbt.append(n);
        }
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_append_lr() {
        let (standard, nums) = RedBlackTree::gen_data_for_test_append_lr() ;
        let mut rbt = RedBlackTree::new();
        for n in nums {
            rbt.append(n);
        }
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_append_rl() {
        let (standard, nums) = RedBlackTree::gen_data_for_test_append_rl() ;
        let mut rbt = RedBlackTree::new();
        for n in nums {
            rbt.append(n);
        }
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_append_multiple_adjsut() {
        let (standard, nums) = RedBlackTree::gen_data_for_test_append_multiple_adjust() ;
        let mut rbt = RedBlackTree::new();
        for n in nums {
            rbt.append(n);
        }
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_delete_node_with_a_left_child() {
        let (standard, append, delete) = RedBlackTree::gen_data_for_delete_node_with_a_left_child() ;
        let mut rbt = RedBlackTree::new();
        for n in append {
            rbt.append(n);
        }
        for n in delete {
            assert_eq!(rbt.delete(&n), Some(n));
        } 
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_delete_node_with_a_right_child() {
        let (standard, append, delete) = RedBlackTree::gen_data_for_delete_node_with_a_right_child() ;
        let mut rbt = RedBlackTree::new();
        for n in append {
            rbt.append(n);
        }
        for n in delete {
            assert_eq!(rbt.delete(&n), Some(n));
        }
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_delete_red_without_children() {
        let (standard, append, delete) = RedBlackTree::gen_data_for_delete_red_without_children() ;
        let mut rbt = RedBlackTree::new();
        for n in append {
            rbt.append(n);
        }
        for n in delete {
            assert_eq!(rbt.delete(&n), Some(n));
        }
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_delete_ll() {
        let (standard, append, delete) = RedBlackTree::gen_data_for_delete_ll() ;
        let mut rbt = RedBlackTree::new();
        for n in append {
            rbt.append(n);
        }
        for n in delete {
            assert_eq!(rbt.delete(&n), Some(n));
        }
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_delete_rr() {
        let (standard, append, delete) = RedBlackTree::gen_data_for_delete_rr() ;
        let mut rbt = RedBlackTree::new();
        for n in append {
            rbt.append(n);
        }
        for n in delete {
            assert_eq!(rbt.delete(&n), Some(n));
        }
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_delete_lr() {
        let (standard, append, delete) = RedBlackTree::gen_data_for_delete_lr() ;
        let mut rbt = RedBlackTree::new();
        for n in append {
            rbt.append(n);
        }
        for n in delete {
            assert_eq!(rbt.delete(&n), Some(n));
        }
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_delete_rl() {
        let (standard, append, delete) = RedBlackTree::gen_data_for_delete_rl() ;
        let mut rbt = RedBlackTree::new();
        for n in append {
            rbt.append(n);
        }
        for n in delete {
            assert_eq!(rbt.delete(&n), Some(n));
        }
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_delete_just_a_node() {
        let (standard, append, delete) = RedBlackTree::gen_data_for_delete_just_a_node() ;
        let mut rbt = RedBlackTree::new();
        for n in append {
            rbt.append(n);
        }
        for n in delete {
            assert_eq!(rbt.delete(&n), Some(n));
        }
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_delete_black_sbling_and_black_children_and_black_parent() {
        let (mut rbt, standard, delete) = RedBlackTree::gen_data_for_delete_black_sbling_and_black_children_and_black_parent() ;
        assert_eq!(rbt.delete(&delete), Some(delete));
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_delete_black_sbling_and_black_children_and_red_parent() {
        let (mut rbt, standard, delete) = RedBlackTree::gen_data_for_delete_black_sbling_and_black_children_and_red_parent() ;
        assert_eq!(rbt.delete(&delete), Some(delete));
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_delete_black_sbling_and_black_children_and_root_parent() {
        let (mut rbt, standard, delete) = RedBlackTree::gen_data_for_delete_black_sbling_and_black_children_and_root_parent() ;
        assert_eq!(rbt.delete(&delete), Some(delete));
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_delete_red_sbling() {
        let (mut rbt, standard, delete) = RedBlackTree::gen_data_for_delete_red_sbling() ;
        assert_eq!(rbt.delete(&delete), Some(delete));
        assert_eq!(rbt, standard);
    }
    #[test]
    fn test_delete() {
        let (mut rbt, standards, nums) = RedBlackTree::gen_data_for_delete();
        let mut index = 0;
        for (i, n) in nums.into_iter().map(|x|{let result = (index, x); index += 1; result}).collect::<Vec<(usize, i32)>>() {
            assert_eq!(rbt.delete(&n), Some(n));
            assert_eq!(rbt, standards[i]);
        }
    }
}
