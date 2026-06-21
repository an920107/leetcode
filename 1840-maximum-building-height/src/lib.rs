pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
        let mut restrictions: Vec<Restriction> =
            restrictions.iter().map(Restriction::from).collect();
        restrictions.push(Restriction { id: 1, height: 0 });
        restrictions.sort_unstable_by_key(|restriction| restriction.id);

        if let Some(last_restriction) = restrictions.last_mut()
            && last_restriction.id == n
        {
            last_restriction.height = last_restriction.height.min(n - 1);
        } else {
            restrictions.push(Restriction {
                id: n,
                height: n - 1,
            });
        }

        let mut height_ranges: HashMap<i32, i32> = HashMap::new();
        height_ranges.insert(1, 0);

        for window in restrictions.windows(2) {
            let (r1, r2) = (&window[0], &window[1]);
            let l1_height = height_ranges.get(&r1.id).unwrap();
            height_ranges.insert(r2.id, r2.height.min(l1_height + r2.id - r1.id));
        }

        let mut result = 0;

        restrictions.reverse();
        for window in restrictions.windows(2) {
            let (r1, r2) = (&window[0], &window[1]);
            let l1_height = *height_ranges.get(&r1.id).unwrap();
            let l2_height = *height_ranges.get(&r2.id).unwrap();
            height_ranges.insert(
                r2.id,
                l2_height.min(r2.height.min(l1_height + r1.id - r2.id)),
            );

            {
                let l1 = *height_ranges.get(&r1.id).unwrap();
                let l2 = *height_ranges.get(&r2.id).unwrap();
                let d = r1.id - r2.id;
                let max_height = l1.max(l2) + 0.max((d - l1.abs_diff(l2) as i32) / 2);
                result = result.max(max_height);
            }
        }

        result
    }
}

#[derive(Clone, Copy)]
struct Restriction {
    id: i32,
    height: i32,
}

impl From<&Vec<i32>> for Restriction {
    fn from(value: &Vec<i32>) -> Self {
        Restriction {
            id: value[0],
            height: value[1],
        }
    }
}
