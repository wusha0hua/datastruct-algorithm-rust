use std::collections::{HashMap, HashSet};
use crate::datastruct::{graph::{GraphTrait, GraphType}, heap};

#[derive(Debug, Clone)]
pub struct GraphLinkList {
    pub to: usize,
    pub weight: isize,
    pub next: Option<Box<GraphLinkList>>,
}

#[derive(Debug, Clone)]
pub struct GraphAdjacencyLinkList {
    pub vertex: HashMap<usize, Option<Box<GraphLinkList>>>,
    pub graph_type: GraphType,
}

impl GraphTrait for GraphAdjacencyLinkList {
    fn print(&self) {
        let mut edge_map = HashMap::new();
        let mut vertex = Vec::new();
        for (from, head) in self.vertex.iter() {
            let mut edge = HashMap::new();
            Self::set_edge_map(head, &mut edge);
            edge_map.insert(from, edge);
            vertex.push(from);
        }        
        vertex.sort();
        print!("\t");
        for v in vertex.iter() {
            print!("{}\t", v);
        }
        println!("");
        for from in vertex.iter() {
            print!("{}\t", from);
            for to in vertex.iter() {
                match edge_map.get(from) {
                    Some(edge) => match edge.get(&to) {
                        Some(weight) => print!("{}\t", weight),
                        None => print!("nan\t"),
                    }
                    None => panic!("error"),
                }
            }
            println!("");
        }
    }
    fn convert_to_vector(&self) -> Vec<Vec<Option<isize>>> {
        let mut edge_map = HashMap::new();
        let mut vertex = Vec::new();
        for (from, head) in self.vertex.iter() {
            let mut edge = HashMap::new();
            Self::set_edge_map(head, &mut edge);
            edge_map.insert(from, edge);
            vertex.push(from);
        }        
        vertex.sort();
        let mut matrix = Vec::new();
        for from in vertex.iter() {
            let mut row = Vec::new();
            for to in vertex.iter() {
                match edge_map.get(from) {
                    Some(map) => match map.get(to) {
                        Some(weight) => row.push(Some(**weight)),
                        None => row.push(None),
                    }
                    None => panic!("error"),
                }
            } 
            matrix.push(row)
        }
        matrix
    }
    fn add_vertex(&mut self, id: usize) -> bool {
        if self.vertex.contains_key(&id) {
            false
        } else {
            self.vertex.insert(id, None); 
            true
        }
    }
    fn delete_vertex(&mut self, id: usize) -> bool {
        if !self.vertex.contains_key(&id) {
            return false;
        }
        self.vertex.remove(&id);
        for head in self.vertex.values_mut() {
            let mut flag = false;
            *head = Self::delete_by_sort(head.take(), id, &mut flag);
        }
        true
    }
    fn add_edge(&mut self, (from, to, weight): (usize, usize, isize)) -> bool {
        self.add_vertex(from);
        self.add_vertex(to);
        match self.vertex.get_mut(&from) {
            Some(head) => {
                let mut flag = true;
                *head = Self::insert_by_sort(head.take(), to, weight, &mut flag);
            }
            None => panic!("add edge error"),
        }
        if let GraphType::NDG = &self.graph_type {
            match self.vertex.get_mut(&to) {
                Some(head) => {
                    let mut flag = true;
                    *head = Self::insert_by_sort(head.take(), from, weight, &mut flag);
                }
                None => panic!("add edge error"),
            }
        }
        true 
    } 
    fn delete_edge(&mut self, (from, to): (usize, usize)) -> bool {
        let mut flag = match self.vertex.get_mut(&from) {
            Some(head) => {
                let mut flag = false;
                *head = Self::delete_by_sort(head.take(), to, &mut flag);
                flag
            }
            None => false,
        }; 
        flag = match &self.graph_type {
            GraphType::NDG if flag => match self.vertex.get_mut(&to) {
                Some(head) => {
                    let mut flag = false;
                    *head = Self::delete_by_sort(head.take(), from, &mut flag);
                    flag
                }
                None => false
            }
            _ => false,
        };
        flag
    }
    fn update_edge(&mut self, (from, to, weight): (usize, usize, isize)) -> bool {
        let mut flag = match self.vertex.get_mut(&from) {
            Some(head) => {
                let mut flag = false;
                *head = Self::update_by_sort(head.take(), to, weight, &mut flag);
                flag
            }
            None => false,
        };
        flag = match &self.graph_type {
            GraphType::NDG if flag => match self.vertex.get_mut(&to) {
                Some(head) => {
                    let mut flag = false;
                    *head = Self::update_by_sort(head.take(), from, weight, &mut flag);
                    flag
                }
                None => false,
            }
            _ => false,
        };
        flag
    }
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut stack = Vec::new();
        let mut path = Vec::new();
        let mut visited = HashSet::new();
        match self.vertex.get(&start) {
            Some(_) => stack.push(start),
            None => return path,
        }
        while let Some(id) = stack.pop() {
            if visited.contains(&id) == true {
                continue;
            } else {
                visited.insert(id);
            }
            path.push(id); 
            if let Some(map) = self.vertex.get(&id) {
                let mut edge = HashMap::new();
                Self::set_edge_map(map, &mut edge);
                for (to, _) in edge.iter() {
                    stack.push(*to);
                }
            }
        }
        path
    }
    fn bfs(&self, start: usize) -> Vec<usize> {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut path = Vec::new();
        let mut visited = HashSet::new();
        match self.vertex.get(&start) {
            Some(_) => queue.push_back(start),
            None => return path,
        }
        while let Some(id) = queue.pop_front() {
            if visited.contains(&id) {
                continue;
            } else {
                visited.insert(id);
            }
            path.push(id);
            if let Some(map) = self.vertex.get(&id) {
                let mut edge = HashMap::new();
                Self::set_edge_map(map, &mut edge);
                for (to, _) in edge.iter() {
                    queue.push_back(*to);
                }
            }
        }
        path
    }
    fn topo_sort(&self) -> Option<Vec<usize>> {
        let mut path = Vec::new();
        
        Some(path)
    }
    fn minimum_spanning_tree_with_kruskal(&self) -> Self {todo!()}
    fn minimum_spanning_tree_with_prim(&self) -> Self {todo!()}
    fn shortest_path_with_dijkstra(&self, start: usize) -> HashMap<usize, Option<isize>> {todo!()}
    fn shortest_path_with_floyd(&self) -> HashMap<usize, HashMap<usize, isize>> {todo!()}
}

impl GraphAdjacencyLinkList {
    pub fn new(graph_type: GraphType) -> Self {
        Self { 
            vertex: HashMap::new(), 
            graph_type,
        }
    }
    pub fn from(matrix: Vec<Vec<Option<isize>>>, graph_type: GraphType) -> Self {
        let len = matrix.len();
        let mut graph = Self::new(graph_type);
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
    fn insert_by_sort(head: Option<Box<GraphLinkList>>, to: usize, weight: isize, flag: &mut bool) -> Option<Box<GraphLinkList>> {
        match head {
            Some(mut head) => {
                if head.to > to {
                    head = Box::new(GraphLinkList::from(to, weight, Some(head)));
                    *flag = true;
                } else if head.to < to {
                    head.next = Self::insert_by_sort(head.next.take(), to, weight, flag);
                }
                Some(head)
            }
            None => Some(Box::new(GraphLinkList::from(to, weight, None))),
        }
    }
    fn delete_by_sort(head: Option<Box<GraphLinkList>>, to: usize, flag: &mut bool) -> Option<Box<GraphLinkList>> {
        match head {
            Some(mut head) => {
                if head.to == to {
                    *flag = true;
                    head.next
                } else if head.to < to {
                    head.next = Self::delete_by_sort(head.next.take(), to, flag);
                    Some(head)
                } else {
                    Some(head)
                }
            }
            None => None,
        }
    }
    fn update_by_sort(head: Option<Box<GraphLinkList>>, to: usize, weight: isize, flag: &mut bool) -> Option<Box<GraphLinkList>> {
        match head {
            Some(mut head) => {
                if head.to == to {
                    head.weight = weight;
                    *flag = true;
                } else if head.to < to {
                    head.next = Self::update_by_sort(head.next.take(), to, weight, flag);
                }
                Some(head)
            }
            None => None,
        }
    }
    fn set_edge_map<'a>(head: &'a Option<Box<GraphLinkList>>, map: &mut HashMap<usize, &'a isize>) {
        match head {
            Some(head) => {
                map.insert(head.to, &head.weight);
                Self::set_edge_map(&head.next, map);
            }
            None => return,
        } 
    }
}

impl GraphLinkList {
    pub fn from(to: usize, weight: isize, next: Option<Box<GraphLinkList>>) -> Self {
        Self { 
            to, 
            weight, 
            next,
        }
    }
}

