pub struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();

        let mut i = 0;
        let mut j = 0;

        let mut result = 0;
        let mut state: Vec<i32> = vec![0; 3];

        while j < n {
            state[(s[j] - b'a') as usize] += 1;
            while state.iter().all(|&count| count > 0) {
                result += n - j;
                state[(s[i] - b'a') as usize] -= 1;
                i += 1;
            }
            j += 1;
        }

        result as i32
    }
}

// impl Solution {
//     pub fn number_of_substrings(s: String) -> i32 {
//         let chars = s.as_bytes();

//         let mut seen: Vec<Option<usize>> = vec![None; 3];
//         let mut cnt = 0;

//         for (i, &val) in chars.iter().enumerate() {
//             seen[val as usize - 'a' as usize] = Some(i);

//             if seen.iter().all(|&index| index.is_some()) {
//                 cnt += seen.iter().min().copied().unwrap().unwrap();
//             }
//         }

//         cnt as i32
//     }
// }
