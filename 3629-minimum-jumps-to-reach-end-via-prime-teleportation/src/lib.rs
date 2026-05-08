pub struct Solution;

use std::{
    collections::{HashMap, VecDeque},
    sync::OnceLock,
};

static FACTORS: OnceLock<Vec<Vec<i32>>> = OnceLock::new();

impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        let factors = FACTORS.get_or_init(|| {
            let mut factors = vec![vec![]; 1_000_001];
            for i in 2..factors.len() {
                if factors[i].is_empty() {
                    for j in (i..factors.len()).step_by(i) {
                        factors[j].push(i as i32);
                    }
                }
            }
            factors
        });

        let mut primes_to_num_indices: HashMap<i32, Vec<usize>> = HashMap::new();
        for (j, &num) in nums.iter().enumerate() {
            for &prime in factors[num as usize].iter() {
                primes_to_num_indices.entry(prime).or_default().push(j);
            }
        }

        let mut visited: Vec<bool> = vec![false; nums.len()];
        visited[0] = true;

        let mut bfs_queue: VecDeque<BfsParams> =
            VecDeque::from_iter([BfsParams { index: 0, layer: 0 }]);
        let mut result = 0;

        while let Some(BfsParams { index, layer }) = bfs_queue.pop_front() {
            if index == nums.len() - 1 {
                result = layer;
                break;
            }

            if index < nums.len() - 1 && !visited[index + 1] {
                visited[index + 1] = true;
                bfs_queue.push_back(BfsParams {
                    index: index + 1,
                    layer: layer + 1,
                });
            }
            if index > 0 && !visited[index - 1] {
                visited[index - 1] = true;
                bfs_queue.push_back(BfsParams {
                    index: index - 1,
                    layer: layer + 1,
                });
            }
            let num = nums[index];
            if factors[num as usize].len() == 1 && factors[num as usize][0] == num {
                if let Some(targets) = primes_to_num_indices.remove(&num) {
                    for j in targets.into_iter() {
                        if !visited[j] {
                            visited[j] = true;
                            bfs_queue.push_back(BfsParams {
                                index: j,
                                layer: layer + 1,
                            });
                        }
                    }
                }
            }
        }

        result
    }
}

struct BfsParams {
    index: usize,
    layer: i32,
}
