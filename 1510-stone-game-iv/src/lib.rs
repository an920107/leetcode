pub struct Solution;

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let n = n as usize;

        let mut dp: Vec<bool> = vec![false; n + 1];
        dp[0] = false;

        for i in 1..=n {
            let mut k = 1;
            while k * k <= i {
                if !dp[i - k * k] {
                    dp[i] = true;
                    break;
                }
                k += 1;
            }
        }

        dp[n]

        // let mut memo = vec![None; n as usize + 1];
        // Self::is_able_to_win(&mut memo, n as usize)
    }

    // fn is_able_to_win(memo: &mut Vec<Option<bool>>, n: usize) -> bool {
    //     if let Some(result) = memo[n] {
    //         return result;
    //     }

    //     if n == 0 {
    //         return false;
    //     }

    //     let mut k = 1;
    //     while k * k <= n {
    //         if !Self::is_able_to_win(memo, n - k * k) {
    //             memo[n] = Some(true);
    //             return true;
    //         }
    //         k += 1;
    //     }
    //     memo[n] = Some(false);
    //     false
    // }
}
