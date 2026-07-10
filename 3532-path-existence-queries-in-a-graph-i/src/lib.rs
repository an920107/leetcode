pub struct Solution;

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = n as usize;
        let mut disjoint_set = DisjointSet::new(n);

        for j in 1..n {
            let i = j - 1;
            if nums[j] - nums[i] <= max_diff {
                disjoint_set.union(i, j);
            }
        }

        let mut result: Vec<bool> = Vec::with_capacity(queries.len());
        for query in queries.into_iter() {
            let (u, v) = (query[0] as usize, query[1] as usize);
            result.push(disjoint_set.find(u) == disjoint_set.find(v));
        }

        result
    }
}

struct DisjointSet {
    nodes: Vec<usize>,
    ranks: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        let nodes = (0..n).collect();
        let ranks = vec![1; n];
        Self { nodes, ranks }
    }

    fn find(&mut self, node: usize) -> usize {
        if node == self.nodes[node] {
            return node;
        }
        let root = self.find(self.nodes[node]);
        self.nodes[node] = root;
        root
    }

    fn union(&mut self, u: usize, v: usize) {
        let u_root = self.find(u);
        let v_root = self.find(v);
        if u_root == v_root {
            return;
        }
        if self.ranks[u_root] > self.ranks[v_root] {
            self.nodes[v_root] = u_root;
        } else if self.ranks[u_root] < self.ranks[v_root] {
            self.nodes[u_root] = v_root;
        } else {
            self.nodes[v_root] = u_root;
            self.ranks[u_root] += 1;
        }
    }
}
