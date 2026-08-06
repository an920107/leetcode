pub struct Solution;

impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let k = k as usize;

        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for edge in invocations {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            graph[u].push(v);
        }

        let mut valid_vertices = vec![true; n];
        let mut visited = vec![false; n];
        let mut dfs_stack = vec![k];
        while let Some(node) = dfs_stack.pop() {
            valid_vertices[node] = false;
            if visited[node] {
                continue;
            }
            visited[node] = true;
            for &next_node in graph[node].iter() {
                dfs_stack.push(next_node);
            }
        }

        let mut visited = vec![false; n];
        let mut dfs_stack: Vec<usize> = valid_vertices
            .iter()
            .enumerate()
            .filter(|(_, v)| **v)
            .map(|(k, _)| k)
            .collect();
        while let Some(node) = dfs_stack.pop() {
            if visited[node] {
                continue;
            }
            if !valid_vertices[node] {
                return (0..(n as i32)).collect();
            }
            visited[node] = true;
            for &next_node in graph[node].iter() {
                dfs_stack.push(next_node);
            }
        }

        valid_vertices
            .into_iter()
            .enumerate()
            .filter(|(_, v)| *v)
            .map(|(k, _)| k as i32)
            .collect()
    }
}
