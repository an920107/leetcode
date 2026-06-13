pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut tree = vec![vec![]; n];

        for edge in edges.into_iter() {
            let (u, v) = (edge[0] as usize - 1, edge[1] as usize - 1);
            tree[u].push(v);
            tree[v].push(u);
        }

        let tree_lca = TreeLca::new(n, &tree);
        let mut result = Vec::with_capacity(queries.len());

        for query in queries.into_iter() {
            let (u, v) = (query[0] as usize - 1, query[1] as usize - 1);

            if u == v {
                result.push(0);
                continue;
            }

            let lca = tree_lca.get_lca(u, v);
            let length =
                tree_lca.get_depth(u) + tree_lca.get_depth(v) - 2 * tree_lca.get_depth(lca);

            let mut base = 2i64;
            let mut combination = 1i64;
            let mut k = length - 1;

            while k > 0 {
                if k % 2 == 1 {
                    combination = combination * base % MOD;
                    k -= 1;
                }
                base = base * base % MOD;
                k /= 2;
            }

            result.push(combination as i32);
        }

        result
    }
}

struct TreeLca {
    depths: Vec<usize>,
    up: Vec<Vec<usize>>,
    max_pow: usize,
}

impl TreeLca {
    pub fn new(n: usize, tree: &Vec<Vec<usize>>) -> TreeLca {
        let max_pow = (n as f32).log2().ceil() as usize + 1;
        let depths = vec![0; n];
        let up = vec![vec![0; max_pow]; n];

        let mut tree_lca = Self {
            depths,
            up,
            max_pow,
        };
        tree_lca.dfs(tree, 0, 0, 0);
        tree_lca
    }

    pub fn get_depth(&self, node: usize) -> usize {
        self.depths[node]
    }

    pub fn get_lca(&self, mut u: usize, mut v: usize) -> usize {
        if self.depths[u] < self.depths[v] {
            return self.get_lca(v, u);
        }

        for i in (0..self.max_pow).rev() {
            if self.depths[self.up[u][i]] >= self.depths[v] {
                u = self.up[u][i];
            }
        }

        if u == v {
            return u;
        }

        for i in (0..self.max_pow).rev() {
            if self.up[u][i] != self.up[v][i] {
                u = self.up[u][i];
                v = self.up[v][i];
            }
        }

        self.up[u][0]
    }

    fn dfs(&mut self, tree: &Vec<Vec<usize>>, node: usize, parent: usize, depth: usize) {
        self.depths[node] = depth;
        self.up[node][0] = parent;

        for i in 1..self.max_pow {
            self.up[node][i] = self.up[self.up[node][i - 1]][i - 1];
        }

        for &child in tree[node].iter() {
            if child == parent {
                continue;
            }
            self.dfs(tree, child, node, depth + 1);
        }
    }
}
