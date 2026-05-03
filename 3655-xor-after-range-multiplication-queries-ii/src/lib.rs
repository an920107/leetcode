pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let n_sqrt = n.isqrt();
        let queries: Vec<Query> = queries.into_iter().map(Query::from).collect();

        let mut groups: Vec<Vec<Query>> = vec![vec![]; n_sqrt];

        for query in queries.into_iter() {
            if query.k >= n_sqrt {
                let mut index = query.l;
                while index <= query.r {
                    nums[index] = ((nums[index] as i64 * query.v as i64) % MOD) as i32;
                    index += query.k;
                }
            } else {
                groups[query.k].push(query);
            }
        }

        for (k, queries) in groups.into_iter().enumerate() {
            if queries.is_empty() {
                continue;
            }

            let mut diff: Vec<i32> = vec![1; n + n_sqrt];
            for query in queries.into_iter() {
                diff[query.l] = (diff[query.l] as i64 * query.v as i64 % MOD) as i32;
                let r_idx = ((query.r - query.l) / k + 1) * k + query.l;
                diff[r_idx] =
                    (diff[r_idx] as i64 * Self::pow_mod(query.v as i64, MOD - 2, MOD) % MOD) as i32;
            }

            for i in k..n {
                diff[i] = (diff[i] as i64 * diff[i - k] as i64 % MOD) as i32;
            }
            for i in 0..n {
                nums[i] = (nums[i] as i64 * diff[i] as i64 % MOD) as i32;
            }
        }

        nums.into_iter().fold(0, |acc, num| acc ^ num)
    }

    fn pow_mod(mut x: i64, mut y: i64, m: i64) -> i64 {
        let mut res = 1;
        while y > 0 {
            if y & 1 == 1 {
                res = res * x % m;
            }
            x = x * x % m;
            y >>= 1;
        }
        res
    }
}

#[derive(Clone, Copy)]
struct Query {
    l: usize,
    r: usize,
    k: usize,
    v: i32,
}

impl From<Vec<i32>> for Query {
    fn from(value: Vec<i32>) -> Self {
        Self {
            l: value[0] as usize,
            r: value[1] as usize,
            k: value[2] as usize,
            v: value[3],
        }
    }
}
