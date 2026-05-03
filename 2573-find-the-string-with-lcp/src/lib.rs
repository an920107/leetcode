pub struct Solution;

impl Solution {
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();

        let mut word = vec![0u8; n];
        let mut current_c = b'a';

        // generate word
        for i in 0..n {
            if word[i] == 0 {
                if current_c > b'z' {
                    return String::new();
                }

                word[i] = current_c;

                for j in (i + 1)..n {
                    if lcp[i][j] > 0 {
                        word[j] = current_c;
                    }
                }

                current_c += 1;
            }
        }

        // validate lcp matrix
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                let valid = if word[i] != word[j] {
                    lcp[i][j] == 0
                } else {
                    // lcp[i][j] should > 0 here

                    if i == n - 1 || j == n - 1 {
                        lcp[i][j] == 1
                    } else {
                        lcp[i][j] == lcp[i + 1][j + 1] + 1
                    }
                };

                if !valid {
                    return String::new();
                }
            }
        }

        String::from_utf8(word).unwrap()
    }
}
