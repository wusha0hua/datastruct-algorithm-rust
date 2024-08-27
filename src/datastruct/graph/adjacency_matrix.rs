use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Reverse;

use crate::datastruct::graph::{GraphType, GraphTrait};
use crate::datastruct::graph::union_find_set::UnionFindSet;

#[derive(Debug, Clone)]
pub struct GraphAdjacencyMatrix {
    pub vertex: HashMap<usize, usize>,
    pub ids: HashMap<usize, usize>,
    pub matrix: Vec<Vec<Option<isize>>>,
    pub graph_type: GraphType,
}


impl GraphTrait for GraphAdjacencyMatrix {
    fn print(&self) {
        let mut ids: Vec<(&usize, &usize)> = self.vertex.iter().collect();
        ids.sort_by(|(id1, _), (id2, _)| id1.cmp(id2));
        print!("\t");
        for (id, _) in ids.iter() {
            print!("{}\t", id);
        }
        println!("");
        for (&id_from, &index_from) in ids.iter() {
            print!("{}\t", id_from);
            for (_, &index_to) in ids.iter() {
                match self.matrix[index_from][index_to] {
                    Some(weight) => print!("{}\t", weight),
                    None => print!("nan\t"),
                } 
            }
            println!("");
        }
    }
    fn convert_to_vector(&self) -> Vec<Vec<Option<isize>>> {
        let mut result = vec![vec![None; self.vertex.len()]; self.vertex.len()];
        let mut ids: Vec<(&usize, &usize)> = self.vertex.iter().collect();
        ids.sort_by(|(id1, _), (id2, _)| id1.cmp(id2));
        let mut count = 0;
        for (_, &index_from) in ids.iter() {
            for (_, &index_to) in ids.iter() {
                if let Some(wight) = self.matrix[index_from][index_to] {
                    let row = count / ids.len();
                    let column = count % ids.len();
                    result[row][column] = Some(wight);
                }
                count += 1;
            }
        }
        result
    }

    fn add_vertex(&mut self, id: usize) -> bool {
        if self.vertex.contains_key(&id) {
            return false;
        }
        let count = self.vertex.len();
        self.vertex.insert(id, count);
        self.ids.insert(count, id);
        self.matrix.push(vec![None; count]);
        for i in 0..count + 1 {
            self.matrix[i].push(None);
        }
        true
    }
    fn delete_vertex(&mut self, id: usize) -> bool {
        match self.vertex.remove(&id) {
            Some(index) => {
                self.ids.remove(&index);
                let len = self.vertex.len(); 
                for i in 0..len + 1 {
                    if i == index {continue;}
                    self.matrix[index][i] = self.matrix[len][i];
                    self.matrix[i][index] = self.matrix[i][len];
                }
                self.matrix.pop();
                for row in self.matrix.iter_mut() {
                    row.pop();
                }
                for (_, v) in self.vertex.iter_mut() {
                    if *v == len {
                        *v = index;
                        break;
                    }
                }
                true
            }
            None => false,
        }
    }
    fn add_edge(&mut self, (from, to, weight): (usize, usize, isize)) -> bool{
        self.add_vertex(from);
        self.add_vertex(to);
        let from_index = self.vertex[&from];
        let to_index = self.vertex[&to];
        match self.graph_type {
            GraphType::DG => self.matrix[from_index][to_index] = Some(weight),
            GraphType::NDG => {
                self.matrix[from_index][to_index] = Some(weight);
                self.matrix[to_index][from_index] = Some(weight);
            }
        }
        true
    } 
    fn delete_edge(&mut self, (from, to): (usize, usize)) -> bool {
        match self.vertex.get(&from) {
            Some(&from_index) => match self.vertex.get(&to) {
                Some(&to_index) => match self.graph_type {
                    GraphType::DG => match self.matrix[from_index][to_index].take() {
                        Some(_) => true,
                        None => false,
                    }
                    GraphType::NDG => match (self.matrix[from_index][to_index].take(), self.matrix[from_index][to_index].take()) {
                        (Some(_), Some(_)) => true,
                        (Some(w), None) => {
                            self.matrix[from_index][to_index] = Some(w);
                            false
                        }
                        (None, Some(w)) => {
                            self.matrix[to_index][from_index] = Some(w);
                            false
                        }
                        (None, None) => false,
                    }
                },
                None => false,
            },
            None => false,
        }
    }
    fn update_edge(&mut self, (from, to, weight): (usize, usize, isize)) -> bool{
        match self.vertex.get(&from) {
            Some(&from_index) => match self.vertex.get(&to) {
                Some(&to_index) => match self.matrix[from_index][to_index].as_mut() {
                    Some(w) => {
                        *w = weight;
                        true
                    }
                    None => false,
                },
                None => false,
            },
            None => false,
        }
    }
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut stack = Vec::new();
        let mut path = Vec::new();
        let count = self.vertex.len();
        let mut vistied = vec![false; count];
        match self.vertex.get(&start) {
            Some(start_index) => stack.push(*start_index),
            None => return path,
        } 
        while let Some(index) = stack.pop() {
            if vistied[index] == true {
                continue;
            } else {
                vistied[index] = true;
            }
            let id = self.ids[&index]; 
            path.push(id);
            for i in 0..count {
                match self.matrix[index][i].as_ref() {
                    Some(_) => {
                        stack.push(i); 
                    }
                    None => (),
                }
            }
        }
        path
    }
    fn bfs(&self, start: usize) -> Vec<usize> {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut path = Vec::new();
        let count = self.vertex.len();
        let mut vistied = vec![false; count];
        match self.vertex.get(&start) {
            Some(index) => queue.push_back(*index),
            None => return path,
        }
        while let Some(index) = queue.pop_front() {
            if vistied[index] == true {
                continue;
            } else {
                vistied[index] = true;
            }
            path.push(self.ids[&index]);
            for i in 0..count {
                if let Some(_) = self.matrix[index][i] {
                    queue.push_back(i);
                }
            }
        }
        path
    }
    fn topo_sort(&self) -> Option<Vec<usize>> {
        let count = self.vertex.len();
        let mut in_dgreee = vec![0; count];
        let mut path = Vec::new();
        let mut zero_in = Vec::new();
        for i in 0..count {
            for j in 0..count {
                if let Some(_) = self.matrix[j][i] {
                    in_dgreee[i] += 1;
                }
            }
        }
        for i in 0..count {
            if in_dgreee[i] == 0 {
                in_dgreee[i] = -1;
                zero_in.push(i);
            }
        }
        while let Some(index) = zero_in.pop() {
            path.push(self.ids[&index]);
            for i in 0..count {
                if let Some(_) = self.matrix[index][i] {
                    in_dgreee[i] -= 1;
                    if in_dgreee[i] == 0 {
                        zero_in.push(i);
                    }
                }
            }
        }
        if path.len() == count {
            Some(path)
        } else {
            None
        }
    }
    fn minimum_spanning_tree_with_kruskal(&self) -> Self {
        let graph_type = self.graph_type.clone();
        let mut graph = GraphAdjacencyMatrix::new(graph_type.clone());
        let mut edges = Vec::new();
        let count = self.vertex.len();
        let mut vertex_set = HashSet::new();
        match graph_type {
            GraphType::DG => {
                for from in 0..count {
                    for to in 0..count {
                        if let Some(weight) = self.matrix[from][to] {
                            edges.push((self.ids[&from], self.ids[&to], weight));
                        }
                    }
                }
            }
            GraphType::NDG => {
                for from in 0..count {
                    for to in from..count {
                        if let Some(weight) = self.matrix[from][to] {
                            edges.push((self.ids[&from], self.ids[&to], weight));
                        }
                    }
                }
            }
        }
        edges.sort_by(|(_, _, weight1), (_, _, weight2)| weight2.cmp(weight1));
        let mut unf = UnionFindSet::new(count); 
        while vertex_set.len() != count {
            let (from, to, wight) = edges.pop().unwrap();
            let from_index = self.vertex[&from];
            let to_index = self.vertex[&to];
            if unf.is_union(from_index, to_index) {
                continue;
            } else {
                unf.union(from_index, to_index);
            }
            graph.add_edge((from, to, wight)); 
            vertex_set.insert(from);
            vertex_set.insert(to);
        }
        graph
    }
    fn minimum_spanning_tree_with_prim(&self) -> Self {
        let mut graph = GraphAdjacencyMatrix::new(self.graph_type.clone());
        let count = self.vertex.len();
        let mut vistied = vec![false; count];
        let mut heap = BinaryHeap::new();
        let id = self.ids[&0];
        graph.add_vertex(id);
        vistied[0] = true;
        for i in 0..count {
            if let Some(weight) = self.matrix[0][i] {
                heap.push(Reverse((weight, 0, i)));
            } 
        }
        while let Some(Reverse((weight, from_index, to_index))) = heap.pop() {
            if vistied[to_index] {
                continue;
            } else {
                vistied[to_index] = true;
            }
            let to_id = self.ids[&to_index];
            let from_id = self.ids[&from_index];
            graph.add_edge((from_id, to_id, weight));
            for i in 0..count {
                if vistied[i] {continue;}
                if let Some(weight) = self.matrix[to_index][i] {
                    heap.push(Reverse((weight, to_index, i)));
                }
            }
        }
        graph
    }
    fn shortest_path_with_dijkstra(&self, start: usize) -> HashMap<usize, Option<isize>> {
        let count = self.vertex.len();
        let mut vistied = vec![false; count];
        let mut distance = HashMap::new();
        for i in 0..count {
            let id = self.ids[&i];
            distance.insert(id, None);
        }
        let mut heap = BinaryHeap::new();
        let index = self.vertex[&start];
        heap.push(Reverse((0, index)));
        distance.insert(start, Some(0));
        while let Some(Reverse((wight, index))) = heap.pop() {
            if vistied[index] {
                continue;
            } 
            vistied[index] = true;
            for i in 0..count {
                if let Some(w) = self.matrix[index][i] {
                    let id = self.ids[&i];
                    match distance[&id] {
                        Some(old_weight) => if old_weight > wight + w {
                            distance.insert(id, Some(wight + w));
                            heap.push(Reverse((wight + w, i)));
                        }
                        None => {
                            distance.insert(id, Some(wight + w));
                            heap.push(Reverse((wight + w, i)));
                        }
                    } 
                }
            }
        }
        distance
    }
    fn shortest_path_with_floyd(&self) -> HashMap<usize, HashMap<usize, isize>> {
        fn get_weight_with_transfer(path: &HashMap<usize, HashMap<usize, isize>>, from: usize, to: usize, transfer: usize) -> (Option<isize>, Option<isize>) {
            match (path.get(&from), path.get(&transfer)) {
                (Some(from_map), Some(trans_map)) => match (from_map.get(&to), from_map.get(&transfer), trans_map.get(&to)) {
                    (Some(w_f_t), Some(w_f_tans), Some(w_trans_t)) => (Some(*w_f_t), Some(*w_f_tans + *w_trans_t)),
                    (None, Some(w_f_tans), Some(w_trans_t)) => (None, Some(*w_f_tans + *w_trans_t)),
                    (Some(w_f_t), None, _) | (Some(w_f_t), _, None) => (Some(*w_f_t), None),
                    _ => (None, None),
                }
                _ => (None, None),
            }  
        }
        let count = self.vertex.len();
        let mut path = HashMap::new();
        for (id, _) in self.vertex.iter() {
            let mut map = HashMap::new();
            map.insert(*id, 0);
            path.insert(*id, map);
        }
        for from_index in 0..count {
            for to_index in 0..count {
                if let Some(w) = self.matrix[from_index][to_index] {
                    let from_id = self.ids[&from_index];
                    let to_id = self.ids[&to_index];
                    if let Some(map) = path.get_mut(&from_id) {
                        map.insert(to_id, w);
                    }
                }
            }
        }
        for transfer in 0..count {
            for from_index in 0..count {
                for to_index in 0..count {
                    let trans_id = self.ids[&transfer];
                    let from_id = self.ids[&from_index];
                    let to_id = self.ids[&to_index];
                    if from_index == to_index {
                        continue;
                    }
                    match get_weight_with_transfer(&path, from_id, to_id, trans_id) {
                        (Some(w_f_t), Some(w_trans)) => if w_trans < w_f_t {
                            if let Some(map) = path.get_mut(&from_id) {
                                map.insert(to_id, w_trans);
                            }
                        } 
                        (Some(w_f_t), None) => if let Some(map) = path.get_mut(&from_id) {
                            map.insert(to_id, w_f_t);
                        }
                        (None, Some(w_trans)) => if let Some(map) = path.get_mut(&from_id) {
                            map.insert(to_id, w_trans);
                        }
                        (None, None) => (),
                    }
                }
            }
        }
        path
    }
}


impl GraphAdjacencyMatrix {
    pub fn new(graph_type: GraphType) -> Self {
        Self { 
            vertex: HashMap::new(),
            ids: HashMap::new(),
            matrix: Vec::new(), 
            graph_type,
        }
    }
    pub fn from(matrix: Vec<Vec<Option<isize>>>, graph_type: GraphType) -> Self {
        let len = matrix.len();
        let mut graph = GraphAdjacencyMatrix::new(graph_type);
        for i in 0..len {
            for j in 0..len {
                match matrix[i][j] {
                    Some(weight) => graph.add_edge((i, j, weight)),
                    None => false,
                };
            }
        }
        graph
    }
}
