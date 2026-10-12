pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        Self::recursion(&mut result, n as usize, "".to_string(), 0, n as usize);
        result
    }

    fn recursion(
        result: &mut Vec<String>,
        n: usize,
        current: String,
        depth: usize,
        left_required: usize,
    ) {
        if current.len() == n * 2 {
            result.push(current);
            return;
        }

        if left_required > 0 {
            let mut next = current.clone();
            next.push('(');
            Self::recursion(result, n, next, depth + 1, left_required - 1);
        }

        if depth > 0 {
            let mut next = current.clone();
            next.push(')');
            Self::recursion(result, n, next, depth - 1, left_required);
        }
    }
}
