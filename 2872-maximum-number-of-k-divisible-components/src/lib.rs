pub struct Solution;

impl Solution {
    pub fn max_k_divisible_components(
        _: i32,
        edges: Vec<Vec<i32>>,
        values: Vec<i32>,
        k: i32,
    ) -> i32 {
        let mut tree: Vec<Vec<usize>> = vec![vec![]; values.len()];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            tree[u].push(v);
            tree[v].push(u);
        }

        let mut dfs = DFS::new(k, values, tree);
        dfs.run(0, 0);

        dfs.result
    }
}

struct DFS {
    result: i32,
    visited: Vec<bool>,
    tree: Vec<Vec<usize>>,
    values: Vec<i32>,
    k: i32,
}

impl DFS {
    fn new(k: i32, values: Vec<i32>, tree: Vec<Vec<usize>>) -> Self {
        Self {
            result: 0,
            visited: vec![false; values.len()],
            tree: tree,
            values: values,
            k: k,
        }
    }

    fn run(&mut self, r: i32, root: usize) -> i32 {
        if self.visited[root] {
            return r;
        }
        self.visited[root] = true;

        let val = self.values[root];
        let mut current_r = r;
        let children = self.tree[root].clone();
        for &node in children.iter() {
            current_r = (current_r + self.run(r, node)) % self.k;
        }

        current_r = (current_r + val) % self.k;
        if current_r == 0 {
            self.result += 1;
        }

        current_r
    }
}
