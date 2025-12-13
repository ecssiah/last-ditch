pub struct UnionFind {
    parent_vec: Vec<usize>,
    rank_vec: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parent_vec: (0..size).collect(),
            rank_vec: vec![0; size],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent_vec[x] != x {
            let root = self.find(self.parent_vec[x]);

            self.parent_vec[x] = root;
        }

        self.parent_vec[x]
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return false;
        }

        if self.rank_vec[root_a] < self.rank_vec[root_b] {
            self.parent_vec[root_a] = root_b;
        } else if self.rank_vec[root_a] > self.rank_vec[root_b] {
            self.parent_vec[root_b] = root_a;
        } else {
            self.parent_vec[root_b] = root_a;
            self.rank_vec[root_a] += 1;
        }

        true
    }
}
