pub struct Solution;

impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut intervals = intervals
            .into_iter()
            .enumerate()
            .map(Interval::from)
            .collect::<Vec<_>>();
        intervals.sort();

        let next: Vec<usize> = intervals
            .iter()
            .map(|current| intervals.partition_point(|other| other.left <= current.right))
            .collect();

        let mut memo = vec![vec![None; 5]; intervals.len() + 1];

        (1..=4)
            .map(|required_count| {
                Self::maximum_weight_from_index(&intervals, &next, &mut memo, 0, required_count)
            })
            .max()
            .map(|r| r.indices)
            .unwrap()
    }

    fn maximum_weight_from_index(
        intervals: &[Interval],
        next: &[usize],
        memo: &mut Vec<Vec<Option<BestResult>>>,
        from_index: usize,
        required_count: usize,
    ) -> BestResult {
        let n = intervals.len();

        if from_index >= n || required_count == 0 {
            return BestResult::default();
        }

        if let Some(result) = &memo[from_index][required_count] {
            return result.clone();
        }

        let not_selected =
            Self::maximum_weight_from_index(intervals, next, memo, from_index + 1, required_count);

        let selected = Self::maximum_weight_from_index(
            intervals,
            next,
            memo,
            next[from_index],
            required_count - 1,
        ) + &intervals[from_index];

        let result = not_selected.max(selected);
        memo[from_index][required_count] = Some(result.clone());
        result
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Interval {
    left: usize,
    right: usize,
    weight: i32,
    original_index: usize,
}

impl From<(usize, Vec<i32>)> for Interval {
    fn from(value: (usize, Vec<i32>)) -> Self {
        Self {
            left: value.1[0] as usize,
            right: value.1[1] as usize,
            weight: value.1[2],
            original_index: value.0,
        }
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
struct BestResult {
    sum: i64,
    indices: Vec<i32>,
}

impl std::ops::Add<&Interval> for BestResult {
    type Output = Self;

    fn add(mut self, rhs: &Interval) -> Self::Output {
        self.sum += rhs.weight as i64;
        self.indices.push(rhs.original_index as i32);
        self.indices.sort();
        self
    }
}

impl Ord for BestResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sum
            .cmp(&other.sum)
            .then(other.indices.cmp(&self.indices))
    }
}

impl PartialOrd for BestResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn test_sol() {
    assert_eq!(
        Solution::maximum_weight(vec![
            vec![1, 3, 2],
            vec![4, 5, 2],
            vec![1, 5, 5],
            vec![6, 9, 3],
            vec![6, 7, 1],
            vec![8, 9, 1],
        ]),
        vec![2, 3]
    );
}
