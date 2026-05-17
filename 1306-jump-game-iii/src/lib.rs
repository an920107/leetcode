pub struct Solution;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut dfs_stack = vec![start as usize];
        let mut visited = vec![false; arr.len()];
        while let Some(current_index) = dfs_stack.pop() {
            if visited[current_index] {
                continue;
            }
            visited[current_index] = true;

            let current_value = arr[current_index] as usize;
            if current_index >= current_value {
                dfs_stack.push(current_index - current_value);
            }
            if current_index + current_value < arr.len() {
                dfs_stack.push(current_index + current_value);
            }
        }

        visited
            .into_iter()
            .enumerate()
            .filter_map(|(k, v)| (arr[k] == 0).then(|| v))
            .any(|v| v)
    }
}
