pub struct Solution;

impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let mut result = 0;

        let mut stack: Vec<char> = vec![];
        let mut directions: Vec<char> = directions
            .as_bytes()
            .iter()
            .map(|&c| c as char)
            .rev()
            .collect();

        while let Some(current) = directions.pop() {
            let current = current as char;
            if let Some(&last) = stack.last() {
                if (last == 'L' && current == 'R')
                    || (last == 'S' && current == 'R')
                    || (last == 'L' && current == 'S')
                    || last == current
                {
                    stack.push(current);
                    continue;
                }

                if last == 'R' && current == 'L' {
                    result += 2;
                } else if (last == 'S' && current == 'L') || (last == 'R' && current == 'S') {
                    result += 1;
                }
                stack.pop();
                directions.push('S');
            } else {
                stack.push(current);
            }
        }

        result
    }
}
