pub struct Solution;

impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let mut stack: Vec<Vec<f64>> = vec![cards.iter().map(|&v| v as f64).collect()];

        while let Some(cards) = stack.pop() {
            if cards.len() == 1 {
                if (cards.first().unwrap() - 24f64).abs() < 1e-6 {
                    return true;
                } else {
                    continue;
                }
            }

            for i in 0..(cards.len() - 1) {
                for j in (i + 1)..cards.len() {
                    let a = cards[i];
                    let b = cards[j];

                    let mut left_cards = cards.clone();
                    left_cards.remove(j);
                    left_cards.remove(i);

                    for v in [a + b, a * b, a - b, b - a, a / b, b / a] {
                        let mut new_cards = left_cards.clone();
                        new_cards.push(v);
                        stack.push(new_cards);
                    }
                }
            }
        }

        false
    }
}
