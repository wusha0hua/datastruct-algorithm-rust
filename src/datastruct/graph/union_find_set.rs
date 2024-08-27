
pub struct UnionFindSet {
    pub parent: Vec<usize>,
}

impl UnionFindSet {
    pub fn new(size: usize) -> Self {
        let mut unf = Self {
            parent: Vec::new(),
        };
        for i in 0..size {
            unf.parent.push(i);
        }
        unf
    }
    pub fn union(&mut self, a: usize, b: usize) {
        let pa = self.find(a);
        let pb = self.find(b);
        self.parent[pa] = pb;
    }
    pub fn find(&mut self, a: usize) -> usize {
        if self.parent[a] != a {
            let pa = self.parent[a];
            self.parent[a] = self.find(pa);
        } 
        self.parent[a]
    }
    pub fn is_union(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}
