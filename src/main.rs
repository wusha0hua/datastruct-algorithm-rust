use datastruct_alogritm_rust::datastruct::graph::{adjacency_linklist::GraphAdjacencyLinkList, adjacency_matrix::GraphAdjacencyMatrix};


fn main() {
    use datastruct_alogritm_rust::datastruct::tree::binary_search_tree::BinarySearchTree;
    use datastruct_alogritm_rust::datastruct::tree::TreeTrait;
    use std::rc::Rc;
    use std::cell::RefCell;
    use datastruct_alogritm_rust::node::TreeNode;
    use datastruct_alogritm_rust::algorithm::sort::*;
    use datastruct_alogritm_rust::algorithm::string::*;
    use rand::{Rng, thread_rng};
    use datastruct_alogritm_rust::datastruct::tire::tire::Trie;
    use datastruct_alogritm_rust::datastruct::graph::*;
    use datastruct_alogritm_rust::datastruct::graph::union_find_set::UnionFindSet;
    use datastruct_alogritm_rust::datastruct::link_list::doubly_link_list::*;
    use datastruct_alogritm_rust::datastruct::tree::balanced_binary_tree::BalancedBinaryTree;
    use datastruct_alogritm_rust::datastruct::tree::b_tree::BTree;
    use datastruct_alogritm_rust::datastruct::tree::red_black_tree::RedBlackTree;
    use datastruct_alogritm_rust::datastruct::number::Number;
    let mut number1 = Number::from_dec("789", 1000).unwrap();
    let number2 = Number::from_dec("789", 1000).unwrap();
    //number1 -= number2;
    //println!("{}", number1.dec());
    println!("{:?}", (number1 * number2));
    /*
        let mut bt = BTree::new(4);
        bt.append(10);
        bt.append(20);
        bt.append(30);
        bt.append(40);
        bt.append(15);
        bt.append(18);
        bt.append(50);
        println!("{:?}", bt);
        println!("{:?}", bt.delete(&15));
        println!("{:?}", bt.delete(&50));
    */
    /*
    let mut avl = BalancedBinaryTree::new();
    avl.append(1);
    avl.append(2);
    avl.delete(&1);
    println!("{:#?}", avl);
    */
    /*
const MATRIX1: [[isize; 6]; 6] = [
    [0, 5, 0, 0, 6, 0],
    [0, 0, 0, 7, 0, 0],
    [0, 0, 0, 0, 0, 0],
    [0, 0, 9, 0, 0, 0],
    [0, 0, 0, 0, 0, 8],
    [0, 0, 0, 3, 0, 0],
];
const MATRIX2: [[isize; 7]; 7] = [
    //A,B,C,D,E,F,G
    [0, 7, 0, 5, 0, 0, 0],     // A
    [7, 0, 8, 9, 7, 0, 0],     // B
    [0, 8, 0, 0, 5, 0, 0],     // C
    [5, 9, 0, 0, 15, 6, 0],     // D
    [0, 7, 5, 15, 0, 8, 9],     // E
    [0, 0, 0, 6, 8, 0, 11],     // F
    [0, 0, 0, 0, 9, 11, 0],     // G
];

const MINSPANTREEFORMATRIX2: [[isize; 7]; 7] = [
    //A,B,C,D,E,F,G
    [0, 7, 0, 5, 0, 0, 0],     // A
    [7, 0, 0, 0, 7, 0, 0],     // B
    [0, 0, 0, 0, 5, 0, 0],     // C
    [5, 0, 0, 0, 0, 6, 0],     // D
    [0, 7, 5, 0, 0, 0, 9],     // E
    [0, 0, 0, 6, 0, 0, 0],     // F
    [0, 0, 0, 0, 9, 0, 0],     // G
];

    fn gen_matrix_with_zero_present_nan(m: Vec<Vec<isize>>) -> Vec<Vec<Option<isize>>> {
    let len = m.len();
    let mut matrix = vec![vec![None; len]; len];
    for i in 0..len {
        for j in 0..len {
            if m[i][j] != 0 {
                matrix[i][j] = Some(m[i][j]);
            }
        }
    } 
    matrix
}

fn gen_matrix_with_zero_present_zero(m: Vec<Vec<isize>>) -> Vec<Vec<Option<isize>>> {
    let len = m.len();
    let mut matrix = vec![vec![None; len]; len];
    for i in 0..len {
        for j in 0..len {
            matrix[i][j] = Some(m[i][j]);
        }
    } 
    matrix
}
        let mut matrix2 = Vec::new();
        for r in MATRIX2 {
            matrix2.push(r.to_vec());
        }
        let graph = GraphAdjacencyLinkList::from(gen_matrix_with_zero_present_nan(matrix2), GraphType::NDG);
        graph.print();
        println!("{:?}", graph.convert_to_vector());
        */
/*
        let graph = GraphAdjacencyMatrix::from(gen_matrix_with_zero_present_nan(matrix2), GraphType::NDG); 
        graph.print();
        println!("{:?}", graph.convert_to_vector());
        let mg = graph.minimum_spanning_tree_with_prim();
        mg.print();
        println!("{:?}", MINSPANTREEFORMATRIX2);
        println!("{:?}", mg.convert_to_vector());
*/
    /*
        let t = "asdasd";
        let p = "asd";
        let v = vec![0, 3];
        println!("{:?}", rk(t, p));
        let mut hashs = Vec::new();
        const MERSENNE_PRIME: u64 = 2147483647;
        const ASCII_COUNTS: u64 = 256;
    let quick_power_modular = |base: u64, exp: u64, modulu: u64| -> u64 {
        let mut result = 1;
        let mut base = base % modulu;
        let mut exp = exp % modulu;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base % modulu;
            }
            base = base * base % modulu;
            exp >>= 1;
        }
        result % modulu
    };

        for i in 0..4 {
            let mut patten_hash = 0;
            for (index, value) in t[i..i+3].as_bytes().iter().enumerate() {
                patten_hash = (*value as u64 * ((patten_hash + quick_power_modular(ASCII_COUNTS, (3 - index) as u64 - 1, MERSENNE_PRIME)) % MERSENNE_PRIME)) % MERSENNE_PRIME; 
    }
            hashs.push(patten_hash);
        }
        println!("{:?}", hashs);
    println!("just cargo test");
    */
}
