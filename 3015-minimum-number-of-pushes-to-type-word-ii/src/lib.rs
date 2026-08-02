pub struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut frequency: [usize; 26] = [0; 26];
        for c in word.bytes() {
            frequency[(c - b'a') as usize] += 1;
        }
        let mut ordered_frequency: Vec<(usize, usize)> =
            frequency.iter().copied().enumerate().collect();
        ordered_frequency.sort_by_key(|(_, count)| *count);
        ordered_frequency.reverse();

        let mut orders: [usize; 26] = [0; 26];
        for (index, order) in ordered_frequency
            .iter()
            .enumerate()
            .map(|(order, (index, _))| (index, order))
        {
            orders[*index] = order;
        }
        let mut result = 0;
        for (c, count) in frequency.iter().enumerate() {
            result += *count * (orders[c] / 8 + 1);
        }
        result as i32
    }
}
