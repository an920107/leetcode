pub struct Solution;

const MAX: i64 = 1_000_000_000_000;

impl Solution {
    pub fn minimum_total_distance(mut robots: Vec<i32>, mut factories: Vec<Vec<i32>>) -> i64 {
        robots.sort_unstable();
        factories.sort_unstable_by_key(|f| f[0]);
        let limits = factories.iter().map(|f| f[1]).collect::<Vec<_>>();
        let factories = factories.into_iter().map(|f| f[0]).collect::<Vec<_>>();

        let mut memo = vec![vec![None; factories.len()]; robots.len()];

        Self::min_distance(&robots, &factories, &limits, &mut memo, 0, 0)
    }

    fn min_distance(
        robots: &[i32],
        factories: &[i32],
        limits: &[i32],
        memo: &mut Vec<Vec<Option<i64>>>,
        i: usize,
        j: usize,
    ) -> i64 {
        if i >= robots.len() {
            return 0;
        }
        if j >= factories.len() {
            return MAX;
        }

        if let Some(result) = memo[i][j] {
            return result;
        }

        let mut result = MAX;

        for k in 0..=(limits[j] as usize) {
            if i + k as usize > robots.len() {
                break;
            }

            let distance = (i..(i + k))
                .map(|index| robots[index].abs_diff(factories[j]) as i64)
                .sum::<i64>();
            let next_distance = Self::min_distance(robots, factories, limits, memo, i + k, j + 1);

            result = result.min(distance + next_distance);
        }

        memo[i][j] = Some(result);
        result
    }
}
