pub struct Solution;

impl Solution {
    pub fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let queries: Vec<Query> = queries.into_iter().map(Query::from).collect();

        for query in queries.into_iter() {
            let mut index = query.l;
            while index <= query.r {
                nums[index] = ((nums[index] as i64 * query.v as i64) % 1_000_000_007) as i32;
                index += query.k;
            }
        }

        nums.into_iter().fold(0, |acc, num| acc ^ num)
    }
}

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
