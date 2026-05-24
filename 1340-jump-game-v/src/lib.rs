pub struct Solution;

impl Solution {
    pub fn max_jumps(arr: Vec<i32>, limit: i32) -> i32 {
        let mut memo = vec![None; arr.len()];

        (0..arr.len())
            .map(|start_index| {
                Self::max_jumps_from_index(&arr, &mut memo, limit as usize, start_index)
            })
            .max()
            .unwrap()
    }

    fn max_jumps_from_index(
        arr: &[i32],
        memo: &mut [Option<i32>],
        limit: usize,
        index: usize,
    ) -> i32 {
        if let Some(result) = memo[index] {
            return result;
        }

        if (index == 0 || arr[index - 1] >= arr[index])
            && (index == arr.len() - 1 || arr[index + 1] >= arr[index])
        {
            memo[index] = Some(1);
            return 1;
        }

        let right_targets = ((index + 1)..=(index + limit).min(arr.len() - 1))
            .take_while(|&target| arr[target] < arr[index]);
        let left_targets = (((index as i32 - limit as i32).max(0) as usize)..index)
            .rev()
            .take_while(|&target| arr[target] < arr[index]);
        let result = 1 + right_targets
            .chain(left_targets)
            .map(|target| Self::max_jumps_from_index(arr, memo, limit, target))
            .max()
            .unwrap_or(0) as i32;
        memo[index] = Some(result);
        result
    }
}
