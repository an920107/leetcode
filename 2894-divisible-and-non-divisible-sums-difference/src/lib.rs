pub struct Solution;

impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let (dividable, undividable): (Vec<i32>, Vec<i32>) =
            (1..(n + 1)).partition(|&val| val % m == 0);

        undividable.iter().sum::<i32>() - dividable.iter().sum::<i32>()
    }
}
