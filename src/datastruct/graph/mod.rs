use std::collections::HashMap;

pub mod union_find_set;
pub mod adjacency_matrix;
pub mod adjacency_linklist;

pub trait GraphTrait {
    fn print(&self);
    fn convert_to_vector(&self) -> Vec<Vec<Option<isize>>>;
    fn add_vertex(&mut self, id: usize) -> bool;
    fn delete_vertex(&mut self, id: usize) -> bool;
    fn add_edge(&mut self, info: (usize, usize, isize)) -> bool; 
    fn delete_edge(&mut self, info: (usize, usize)) -> bool;
    fn update_edge(&mut self, info: (usize, usize, isize)) -> bool;
    fn dfs(&self, start: usize) -> Vec<usize>;
    fn bfs(&self, start: usize) -> Vec<usize>;
    fn topo_sort(&self) -> Option<Vec<usize>>;
    fn minimum_spanning_tree_with_kruskal(&self) -> Self;
    fn minimum_spanning_tree_with_prim(&self) -> Self;
    fn shortest_path_with_dijkstra(&self, start: usize) -> HashMap<usize, Option<isize>>;
    fn shortest_path_with_floyd(&self) -> HashMap<usize, HashMap<usize, isize>>;
}

#[derive(Debug, Clone)]
pub enum GraphType {
    DG,
    NDG,
}

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

const MATRIX3: [[isize; 6]; 6] = [
    // A B C D E F 
    [0, 10, 0, 4, 0, 0],     // A
    [10, 0, 8, 2, 6, 0],     // B
    [0, 8, 0, 15, 1, 5],     // C
    [4, 2, 15, 0, 6, 0],     // D
    [0, 6, 1, 6, 0, 12],     // E
    [0, 0, 5, 0, 12, 0],     // F
];

const MATRIX4: [[isize; 7]; 7] = [
    [0, 12, 0, 0, 0, 16, 14],
    [12, 0, 10, 0, 0, 7, 0],
    [0, 10, 0, 3, 5, 6, 0],
    [0, 0, 3, 0, 4, 0, 0],
    [0, 0, 5, 4, 0, 2, 8],
    [16, 7, 6, 0, 2, 0, 9],
    [14, 0, 0, 0, 8, 9, 0],
];

#[cfg(test)]
mod test_graph_adjacency_matrix {
    use crate::datastruct::graph::*;
    use self::adjacency_matrix::GraphAdjacencyMatrix;
    #[test]
    fn test_dfs() {
        let mut matrix1 = Vec::new();
        for r in MATRIX1 {
            matrix1.push(r.to_vec());
        }
        let graph = GraphAdjacencyMatrix::from(gen_matrix_with_zero_present_nan(matrix1), GraphType::DG); 
        let path = graph.dfs(0);
        assert!(path == vec![0, 1, 3, 2, 4, 5] || path == vec![0, 4, 5, 3, 2, 1]);
    }
    #[test]
    fn test_bfs() {
        let mut matrix1 = Vec::new();
        for r in MATRIX1 {
            matrix1.push(r.to_vec());
        }
        let graph = GraphAdjacencyMatrix::from(gen_matrix_with_zero_present_nan(matrix1), GraphType::DG); 
        let path = graph.bfs(0);
        assert!(path == vec![0, 1, 4, 3, 5, 2] || path == vec![0, 4, 1, 5, 3, 2]);
    }
    #[test]
    fn test_mst_kruskal() {
        let mut matrix = Vec::new();
        for r in MATRIX2 {
            matrix.push(r.to_vec());
        }
        let graph = GraphAdjacencyMatrix::from(gen_matrix_with_zero_present_nan(matrix), GraphType::NDG); 
        let mst = graph.minimum_spanning_tree_with_kruskal();
        let mut mmst = Vec::new();
        for r in MINSPANTREEFORMATRIX2 {
            mmst.push(r.to_vec());
        }
        assert_eq!(mst.convert_to_vector(), gen_matrix_with_zero_present_nan(mmst));
    }
    #[test]
    fn test_mst_prime() {
        let mut matrix = Vec::new();
        for r in MATRIX2 {
            matrix.push(r.to_vec());
        }
        let graph = GraphAdjacencyMatrix::from(gen_matrix_with_zero_present_nan(matrix), GraphType::NDG); 
        let mst = graph.minimum_spanning_tree_with_prim();
        let mut mmst = Vec::new();
        for r in MINSPANTREEFORMATRIX2 {
            mmst.push(r.to_vec());
        }
        assert_eq!(mst.convert_to_vector(), gen_matrix_with_zero_present_nan(mmst));
    }
    #[test]
    fn test_sp_dijkstra() {
        let mut matrix = Vec::new();
        for r in MATRIX3 {
            matrix.push(r.to_vec());
        }
        let graph = GraphAdjacencyMatrix::from(gen_matrix_with_zero_present_nan(matrix), GraphType::NDG); 
        let path_test = graph.shortest_path_with_dijkstra(0);
        let path: HashMap<usize, Option<isize>> = vec![(0, Some(0)), (1, Some(6)), (2, Some(11)), (3, Some(4)), (4, Some(10)), (5, Some(16))].into_iter().collect();
        assert_eq!(path_test, path);
    }
    #[test]
    fn test_sp_floyd() {
        let mut matrix = Vec::new();
        for r in MATRIX4 {
            matrix.push(r.to_vec());
        }
        let graph = GraphAdjacencyMatrix::from(gen_matrix_with_zero_present_nan(matrix), GraphType::NDG); 
        let path_test = graph.shortest_path_with_floyd();
        let path = HashMap::from([
            (0, HashMap::from([(0, 0), (1, 12), (2, 22), (3, 22), (4, 18), (5, 16), (6, 14)])),
            (1, HashMap::from([(0, 12), (1, 0), (2, 10), (3, 13), (4, 9), (5, 7), (6, 16)])),
            (2, HashMap::from([(0, 22), (1, 10), (2, 0), (3, 3), (4, 5), (5, 6), (6, 13)])),
            (3, HashMap::from([(0, 22), (1, 13), (2, 3), (3, 0), (4, 4), (5, 6), (6, 12)])),
            (4, HashMap::from([(0, 18), (1, 9), (2, 5), (3, 4), (4, 0), (5, 2), (6, 8)])),
            (5, HashMap::from([(0, 16), (1, 7), (2, 6), (3, 6), (4, 2), (5, 0), (6, 9)])),
            (6, HashMap::from([(0, 14), (1, 16), (2, 13), (3, 12), (4, 8), (5, 9), (6, 0)])),
        ]);
        assert_eq!(path_test, path);
    }
}

#[cfg(test)]
mod test_graph_adjacency_linklist {
    use crate::datastruct::graph::*;
    use self::adjacency_linklist::GraphAdjacencyLinkList;
    #[test]
    fn test_dfs() {
        let mut matrix1 = Vec::new();
        for r in MATRIX1 {
            matrix1.push(r.to_vec());
        }
        let graph = GraphAdjacencyLinkList::from(gen_matrix_with_zero_present_nan(matrix1), GraphType::DG); 
        let path = graph.dfs(0);
        assert!(path == vec![0, 1, 3, 2, 4, 5] || path == vec![0, 4, 5, 3, 2, 1]);
    }
    fn test_bfs() {
        let mut matrix1 = Vec::new();
        for r in MATRIX1 {
            matrix1.push(r.to_vec());
        }
        let graph = GraphAdjacencyLinkList::from(gen_matrix_with_zero_present_nan(matrix1), GraphType::DG); 
        let path = graph.bfs(0);
        assert!(path == vec![0, 1, 4, 3, 5, 2] || path == vec![0, 4, 1, 5, 3, 2]);
    }
}
