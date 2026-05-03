pub struct Solution;

impl Solution {
    pub fn can_be_typed_words(mut text: String, broken_letters: String) -> i32 {
        text.push(' ');

        let broken_set = std::collections::HashSet::<char>::from_iter(broken_letters.chars());
        let mut broken_flag = false;

        let mut count = 0;
        for c in text.chars() {
            if c == ' ' {
                if !broken_flag {
                    count += 1;
                }
                broken_flag = false;
            }

            if broken_flag {
                continue;
            }

            if broken_set.contains(&c) {
                broken_flag = true;
            }
        }

        count
    }
}
