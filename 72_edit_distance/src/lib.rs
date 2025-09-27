pub struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];

        let word1 = word1.bytes().collect::<Vec<u8>>();
        let word2 = word2.bytes().collect::<Vec<u8>>();

        for len1 in 0..word1.len() + 1 {
            for len2 in 0..word2.len() + 1 {
                dp[len1][len2] = if len1 == 0 || len2 == 0 {
                    if len1 > 0 { len1 } else { len2 }
                } else if word1[len1 - 1] == word2[len2 - 1] {
                    dp[len1 - 1][len2 - 1]
                } else {
                    1 + dp[len1 - 1][len2]
                        .min(dp[len1][len2 - 1])
                        .min(dp[len1 - 1][len2 - 1])
                };
            }
        }

        dp[word1.len()][word2.len()] as i32
    }
}
