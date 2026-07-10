pub struct Solution;

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;

        let mut nums_with_index: Vec<(i32, usize)> =
            nums.iter().enumerate().map(|(k, v)| (*v, k)).collect();
        nums_with_index.sort();
        let mut nums_index_map: Vec<usize> = vec![0; nums.len()];
        for (original_index, new_index) in nums_with_index
            .iter()
            .enumerate()
            .map(|(k, (_, v))| (*v, k))
        {
            nums_index_map[original_index] = new_index;
        }

        let sorted_nums: Vec<i32> = nums_with_index.iter().map(|(k, _)| *k).collect();

        let mut sparse_table: Vec<Vec<usize>> = vec![vec![0; (n.ilog2() + 2) as usize]; n];

        let mut left = 0;
        let mut right = 0;
        while left < n {
            while right + 1 < n && sorted_nums[right + 1] - sorted_nums[left] <= max_diff {
                right += 1;
            }
            sparse_table[left][0] = right;
            left += 1;
        }

        for j in 1..((n.ilog2() + 2) as usize) {
            for i in 0..n {
                let first = sparse_table[i][j - 1];
                sparse_table[i][j] = sparse_table[first][j - 1];
            }
        }

        let mut result: Vec<i32> = Vec::with_capacity(queries.len());
        for query in queries.into_iter() {
            let (mut u, mut v) = (
                nums_index_map[query[0] as usize],
                nums_index_map[query[1] as usize],
            );
            if u == v {
                result.push(0);
                continue;
            }
            if u > v {
                (u, v) = (v, u);
            }
            let mut current = u;
            let mut step = 0;
            for j in (0..((n.ilog2() + 2) as usize)).rev() {
                if sparse_table[current][j] < v {
                    current = sparse_table[current][j];
                    step += 1 << j;
                }
            }
            result.push(if sparse_table[current][0] >= v {
                step + 1
            } else {
                -1
            });
        }

        result
    }
}
