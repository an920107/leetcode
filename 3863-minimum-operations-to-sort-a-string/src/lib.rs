pub struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        if s.as_bytes().is_sorted() {
            return 0;
        }

        if s.len() <= 2 {
            return -1;
        }

        let min_c = *s.as_bytes().iter().min().unwrap();
        let max_c = *s.as_bytes().iter().max().unwrap();

        let min_c_indices: Vec<usize> = s
            .as_bytes()
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if *c == min_c { Some(i) } else { None })
            .collect();
        let max_c_indices: Vec<usize> = s
            .as_bytes()
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if *c == max_c { Some(i) } else { None })
            .collect();

        if min_c_indices[0] == 0 || max_c_indices[max_c_indices.len() - 1] == s.len() - 1 {
            return 1;
        }

        if s.as_bytes()[1..].contains(&max_c) {
            let mut sorted_right: Vec<char> = s[1..].chars().collect();
            sorted_right.sort();
            1 + Self::min_operations(s[..1].to_owned() + &sorted_right.iter().collect::<String>())
        } else {
            let mut sorted_left: Vec<char> = s[..(s.len() - 1)].chars().collect();
            sorted_left.sort();
            1 + Self::min_operations(sorted_left.iter().collect::<String>() + &s[(s.len() - 1)..])
        }
    }
}
