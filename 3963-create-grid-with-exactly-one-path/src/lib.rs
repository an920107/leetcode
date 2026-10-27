pub struct Solution;

impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let (m, n) = (m as usize, n as usize);

        let mut result = Vec::with_capacity(m);

        let first_row = ".".repeat(n);
        let others_row = "#".repeat(n - 1) + ".";

        result.push(first_row);
        for _ in 1..m {
            result.push(others_row.clone());
        }

        result
    }
}
