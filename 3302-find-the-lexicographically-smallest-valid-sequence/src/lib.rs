pub struct Solution;

impl Solution {
    pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
        let bytes1: Vec<u8> = word1.bytes().collect();
        let bytes2: Vec<u8> = word2.bytes().collect();

        let mut suffix_match: Vec<usize> = vec![0; bytes1.len() + 1];
        let mut index2 = Some(bytes2.len() - 1);
        for (index1, _) in bytes1.iter().enumerate().rev() {
            let char1 = bytes1[index1];
            let char2 = if let Some(index2) = index2 {
                Some(bytes2[index2])
            } else {
                None
            };
            if let Some(char2) = char2
                && char1 == char2
            {
                suffix_match[index1] = suffix_match[index1 + 1] + 1;
                index2 = if index2 > Some(0) {
                    Some(index2.unwrap() - 1)
                } else {
                    None
                };
            } else {
                suffix_match[index1] = suffix_match[index1 + 1];
            }
        }

        let mut result = Vec::with_capacity(bytes2.len());
        let mut difference_count = 0;
        let mut index2 = 0;
        for (index1, &char1) in bytes1.iter().enumerate() {
            if result.len() == bytes2.len() {
                break;
            }

            let char2 = bytes2[index2];

            if char1 == char2 {
                result.push(index1 as i32);
                index2 += 1;
            } else if difference_count == 0
                && suffix_match[index1 + 1] >= bytes2.len() - (result.len() + 1)
            {
                result.push(index1 as i32);
                index2 += 1;
                difference_count += 1;
            }
        }

        if result.len() == bytes2.len() {
            result
        } else {
            vec![]
        }

        // let mut result = vec![];
        // let mut memo = vec![vec![vec![None; 2]; bytes2.len()]; bytes1.len()];
        // let is_valid = Self::recursion(&mut result, &mut memo, &bytes1, &bytes2, 0, 0, 0);
        // return if is_valid {
        //     result.iter().map(|&index| index as i32).collect()
        // } else {
        //     vec![]
        // };
    }

    // fn recursion(
    //     state: &mut Vec<usize>,
    //     memo: &mut Vec<Vec<Vec<Option<bool>>>>,
    //     bytes1: &[u8],
    //     bytes2: &[u8],
    //     index1: usize,
    //     index2: usize,
    //     diff_count: usize,
    // ) -> bool {
    //     if state.len() == bytes2.len() {
    //         return true;
    //     }
    //     if index1 == bytes1.len() || index2 == bytes2.len() {
    //         return false;
    //     }

    //     if let Some(result) = memo[index1][index2][diff_count] {
    //         return result;
    //     }

    //     let char1 = bytes1[index1];
    //     let char2 = bytes2[index2];

    //     if diff_count == 0 || char1 == char2 {
    //         state.push(index1);
    //         let is_valid = Self::recursion(
    //             state,
    //             memo,
    //             bytes1,
    //             bytes2,
    //             index1 + 1,
    //             index2 + 1,
    //             diff_count + if char1 == char2 { 0 } else { 1 },
    //         );
    //         if is_valid {
    //             return true;
    //         }
    //         state.pop();
    //     }

    //     let result = Self::recursion(state, memo, bytes1, bytes2, index1 + 1, index2, diff_count);
    //     memo[index1][index2][diff_count] = Some(result);
    //     result
    // }
}
