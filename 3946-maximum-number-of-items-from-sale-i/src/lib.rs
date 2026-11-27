pub struct Solution;

impl Solution {
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        let budget = budget as usize;

        let mut multiples: Vec<i32> = Vec::with_capacity(items.len());
        for (i, item_i) in items.iter().enumerate() {
            let factor_i = item_i[0];
            multiples.push(
                items
                    .iter()
                    .enumerate()
                    .filter(|(j, item_j)| {
                        let factor_j = item_j[0];
                        i != *j && factor_j % factor_i == 0
                    })
                    .count() as i32,
            );
        }

        let mut memo = vec![vec![vec![None; 2]; budget + 1]; items.len()];
        Self::recursion(&mut memo, &items, &multiples, 0, budget, false)
    }

    fn recursion(
        memo: &mut Vec<Vec<Vec<Option<i32>>>>,
        items: &[Vec<i32>],
        multiples: &[i32],
        index: usize,
        budget: usize,
        bought: bool,
    ) -> i32 {
        if index >= items.len() {
            return 0;
        }

        if let Some(result) = memo[index][budget][usize::from(bought)] {
            return result;
        }

        let skipped = Self::recursion(memo, items, multiples, index + 1, budget, false);

        let mut purchased = 0;
        let price = items[index][1] as usize;
        if budget >= price {
            let gain = if bought { 1 } else { 1 + multiples[index] };
            purchased = Self::recursion(memo, items, multiples, index, budget - price, true) + gain;
        }

        let result = skipped.max(purchased);
        memo[index][budget][usize::from(bought)] = Some(result);
        result
    }
}
