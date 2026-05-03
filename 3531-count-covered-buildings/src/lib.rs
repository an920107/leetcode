pub struct Solution;

#[derive(Clone, Copy)]
struct MinMaxPair {
    min: usize,
    max: usize,
}

impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let mut row_min_max_pairs: Vec<Option<MinMaxPair>> = vec![None; n as usize];
        let mut col_min_max_pairs: Vec<Option<MinMaxPair>> = vec![None; n as usize];

        for building in buildings.iter() {
            let row = building[1] as usize - 1;
            let col = building[0] as usize - 1;

            let row_pair = row_min_max_pairs.get_mut(col).unwrap();
            *row_pair = Solution::count_min_max_pair(*row_pair, row);

            let col_pair = col_min_max_pairs.get_mut(row).unwrap();
            *col_pair = Solution::count_min_max_pair(*col_pair, col);
        }

        let mut result = 0;

        for building in buildings.iter() {
            let row = building[1] as usize - 1;
            let col = building[0] as usize - 1;

            let row_pair = row_min_max_pairs[col].unwrap();
            let col_pair = col_min_max_pairs[row].unwrap();

            if row_pair.min < row && row < row_pair.max && col_pair.min < col && col < col_pair.max
            {
                result += 1;
            }
        }

        result
    }

    fn count_min_max_pair(original: Option<MinMaxPair>, num: usize) -> Option<MinMaxPair> {
        if let Some(pair) = original {
            Some(MinMaxPair {
                min: pair.min.min(num),
                max: pair.max.max(num),
            })
        } else {
            Some(MinMaxPair { min: num, max: num })
        }
    }
}
