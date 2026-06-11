pub struct Solution;

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len() + 1;
        let mut tree: Vec<Vec<usize>> = vec![vec![]; n];

        for edge in edges.into_iter() {
            let u = (edge[0] - 1) as usize;
            let v = (edge[1] - 1) as usize;
            tree[u].push(v);
            tree[v].push(u);
        }

        let mut max_depth = 0;
        let mut dfs_stack: Vec<(usize, i32)> = vec![(0, 0)];
        let mut visited: Vec<bool> = vec![false; n];
        while let Some((parent, depth)) = dfs_stack.pop() {
            if visited[parent] {
                continue;
            }
            visited[parent] = true;
            max_depth = max_depth.max(depth);
            let children = &tree[parent];
            for &child in children.iter() {
                dfs_stack.push((child, depth + 1));
            }
        }

        let mut result = 1i64;
        for _ in 1..max_depth {
            result = result * 2 % 1_000_000_007;
        }

        result as i32
    }
}
