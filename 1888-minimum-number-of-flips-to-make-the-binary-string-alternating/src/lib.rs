pub struct Solution;

impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut mismatch_count = [0; 2];

        for (index, &c) in bytes.iter().enumerate() {
            // (c & 1) ^ (index & 1) -> (c ^ index) & 1
            // if (c as usize ^ index) & 1 == 0 {
            //     mismatch_count[0] += 1;
            // } else {
            //     mismatch_count[1] += 1;
            // }
            mismatch_count[(c as usize ^ index) & 1] += 1;
        }

        let mut result = mismatch_count[0].min(mismatch_count[1]);

        for index in 0..s.len() {
            let c = bytes[index];
            mismatch_count[(c as usize ^ index) & 1] -= 1;
            mismatch_count[(c as usize ^ (s.len() + index)) & 1] += 1;
            result = result.min(mismatch_count[0]).min(mismatch_count[1]);
        }

        result
    }
}
