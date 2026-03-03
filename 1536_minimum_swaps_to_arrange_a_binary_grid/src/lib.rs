pub struct Solution;

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let mut trailing_zeros: Vec<i32> = grid
            .iter()
            .map(|row| Solution::count_trailing_zeros(row))
            .collect();

        if !Solution::check_is_valid(&trailing_zeros) {
            return -1;
        }

        let n = grid.len();
        let mut result = 0;

        for i in 0..n {
            let zeros = trailing_zeros[i];
            let zeros_required = (n - i - 1) as i32;

            if zeros >= zeros_required {
                continue;
            }

            let mut index_to_swap = n - 1;
            for j in (i + 1)..n {
                let zeros = trailing_zeros[j];
                if zeros >= zeros_required {
                    index_to_swap = j;
                    break;
                }
            }

            for j in (i..index_to_swap).rev() {
                trailing_zeros.swap(j, j + 1);
                result += 1;
            }
        }

        result
    }

    fn count_trailing_zeros(bits: &[i32]) -> i32 {
        let mut count = 0;
        for &b in bits.iter().rev() {
            if b == 1 {
                break;
            }
            count += 1;
        }
        count
    }

    fn check_is_valid(trailing_zeros: &[i32]) -> bool {
        let mut trailing_zeros = trailing_zeros.to_vec();
        trailing_zeros.sort();
        for (index, &zeros) in trailing_zeros.iter().enumerate() {
            if zeros < index as i32 {
                return false;
            }
        }
        true
    }
}
