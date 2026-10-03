pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        let mut segments: Vec<(usize, usize)> = Vec::from_iter((1..s.len()).map(|i| (i, i)));

        let mut extended_flag = true;
        while extended_flag {
            extended_flag = false;

            let segments_cloned = segments.clone();
            segments.clear();
            for segment in segments_cloned {
                if (segment.0 > 0 && s_bytes[segment.0 - 1] == b'(')
                    && (segment.1 < s.len() && s_bytes[segment.1] == b')')
                {
                    segments.push((segment.0 - 1, segment.1 + 1));
                    extended_flag = true;
                } else {
                    segments.push(segment);
                }
            }

            if segments.is_empty() {
                return 0;
            }
            let segments_cloned = segments.clone();
            segments.clear();
            let mut head_segment = segments_cloned[0];
            for segment in segments_cloned.into_iter().skip(1) {
                if head_segment.1 >= segment.0 {
                    head_segment.1 = head_segment.1.max(segment.1);
                    extended_flag = true;
                } else {
                    segments.push(head_segment);
                    head_segment = segment;
                }
            }
            if segments.last().copied() != Some(head_segment) {
                segments.push(head_segment);
            }
        }

        segments
            .into_iter()
            .map(|segment| segment.1 - segment.0)
            .max()
            .unwrap_or(0) as i32
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
}
