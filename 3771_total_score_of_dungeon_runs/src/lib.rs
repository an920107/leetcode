pub struct Solution;

impl Solution {
    pub fn total_score(hp: i32, damages: Vec<i32>, requirements: Vec<i32>) -> i64 {
        let mut prefixs = vec![0];
        for damage in &damages {
            prefixs.push(prefixs.last().unwrap() + damage);
        }

        let score_0: Vec<i32> = damages
            .iter()
            .scan(hp, |state, damage| {
                *state -= damage;
                Some(*state)
            })
            .collect();

        let mut result: i64 = 0;
        let mut r = 0;
        let n = damages.len();
        while r < n {
            let i = Solution::binary_search(r, &score_0, &prefixs, &requirements);
            result += (r + 1 - i) as i64;
            r += 1;
        }

        result
    }

    fn binary_search(r: usize, score_0: &[i32], prefixs: &[i32], requirements: &[i32]) -> usize {
        let mut lower = 0;
        let mut upper = r + 1;
        while lower < upper {
            let mid = (lower + upper) / 2;
            if score_0[r] + prefixs[mid] >= requirements[r] {
                upper = mid;
            } else {
                lower = mid + 1;
            }
        }
        lower
    }
}
