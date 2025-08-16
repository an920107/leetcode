fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut grid = vec![1; n as usize];
        for _ in 1..m {
            for i in 1..n as usize {
                grid[i] += grid[i - 1];
            }
        }

        *grid.last().unwrap()
    }
}
