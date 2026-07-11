pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        let mut disjoint_set = DisjointSet::new(n);
        for edge in edges.into_iter() {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
            disjoint_set.union(u, v);
        }

        let mut complete_connected_graph: Vec<Option<bool>> = vec![None; n];
        'node_for_loop: for node in 0..n {
            let root = disjoint_set.find(node);
            if complete_connected_graph[root].is_some() {
                continue;
            }
            let neighbors = &graph[node];
            let mut nodes_in_graph: HashSet<usize> = HashSet::from_iter(neighbors.iter().copied());
            nodes_in_graph.insert(node);
            for &neighbor in neighbors.iter() {
                let neighbors_of_neighbor = &graph[neighbor];
                let mut nodes_in_neighbor_graph: HashSet<usize> =
                    HashSet::from_iter(neighbors_of_neighbor.iter().copied());
                nodes_in_neighbor_graph.insert(neighbor);
                if nodes_in_graph != nodes_in_neighbor_graph {
                    complete_connected_graph[root] = Some(false);
                    continue 'node_for_loop;
                }
            }
            complete_connected_graph[root] = Some(true);
        }

        complete_connected_graph
            .into_iter()
            .filter(|&flag| flag == Some(true))
            .count() as i32
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
