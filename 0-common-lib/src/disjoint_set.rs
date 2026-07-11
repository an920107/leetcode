pub struct DisjointSet {
    nodes: Vec<usize>,
    ranks: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        let nodes = (0..n).collect();
        let ranks = vec![1; n];
        Self { nodes, ranks }
    }

    pub fn find(&mut self, node: usize) -> usize {
        if node == self.nodes[node] {
            return node;
        }
        let root = self.find(self.nodes[node]);
        self.nodes[node] = root;
        root
    }

    pub fn union(&mut self, u: usize, v: usize) {
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
