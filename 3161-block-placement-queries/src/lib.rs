pub struct Solution;

use std::collections::BTreeSet;

struct SegmentTree {
    tree: Vec<i32>,
    n: i32,
}

impl SegmentTree {
    pub fn new(n: i32) -> Self {
        Self {
            tree: vec![0; (n as usize) << 2],
            n,
        }
    }

    pub fn update(&mut self, idx: i32, val: i32) {
        self._update(idx, val, 1, 0, self.n);
    }

    fn _update(&mut self, idx: i32, val: i32, p: usize, l: i32, r: i32) {
        if l == r {
            self.tree[p] = val;
            return;
        }
        let mid = (l + r) >> 1;
        if idx <= mid {
            self._update(idx, val, p << 1, l, mid);
        } else {
            self._update(idx, val, p << 1 | 1, mid + 1, r);
        }
        self.tree[p] = self.tree[p << 1].max(self.tree[p << 1 | 1]);
    }

    pub fn query(&self, left: i32, right: i32) -> i32 {
        self._query(left, right, 1, 0, self.n)
    }

    fn _query(&self, query_l: i32, query_r: i32, p: usize, l: i32, r: i32) -> i32 {
        if query_l <= l && r <= query_r {
            return self.tree[p];
        }
        let mid = (l + r) >> 1;
        let mut res = 0;
        if query_l <= mid {
            res = res.max(self._query(query_l, query_r, p << 1, l, mid));
        }
        if query_r > mid {
            res = res.max(self._query(query_l, query_r, p << 1 | 1, mid + 1, r));
        }
        res
    }
}

enum Query {
    InsertObstacle { x: i32 },
    CheckSize { x: i32, sz: i32 },
}

impl From<Vec<i32>> for Query {
    fn from(value: Vec<i32>) -> Self {
        match value[0] {
            1 => Query::InsertObstacle { x: value[1] },
            2 => Query::CheckSize {
                x: value[1],
                sz: value[2],
            },
            _ => unreachable!(),
        }
    }
}

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mx = 50000;
        let mut segment_tree = SegmentTree::new(mx);

        let mut obstacle_indices = BTreeSet::new();
        obstacle_indices.insert(0);
        obstacle_indices.insert(mx);

        segment_tree.update(mx, mx);

        let mut results = Vec::new();

        for query in queries.into_iter().map(Query::from) {
            match query {
                Query::InsertObstacle { x } => {
                    let r = obstacle_indices
                        .range((x + 1)..)
                        .next()
                        .copied()
                        .unwrap_or(mx);
                    let l = obstacle_indices
                        .range(..x)
                        .next_back()
                        .copied()
                        .unwrap_or(0);

                    segment_tree.update(x, x - l);
                    segment_tree.update(r, r - x);
                    obstacle_indices.insert(x);
                }

                Query::CheckSize { x, sz } => {
                    let pre = obstacle_indices
                        .range(..=x)
                        .next_back()
                        .copied()
                        .unwrap_or(0);

                    let max_space = segment_tree.query(0, pre).max(x - pre);
                    results.push(max_space >= sz);
                }
            }
        }

        results
    }
}
