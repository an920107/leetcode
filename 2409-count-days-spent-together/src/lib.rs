pub struct Solution;

use std::str::FromStr;

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let a_start = Date::from_str(&arrive_alice).unwrap();
        let a_end = Date::from_str(&leave_alice).unwrap();
        let b_start = Date::from_str(&arrive_bob).unwrap();
        let b_end = Date::from_str(&leave_bob).unwrap();

        if b_start > a_end || a_start > b_end {
            return 0;
        }

        if a_start >= b_start && a_end <= b_end {
            return a_end.days_of_year() - a_start.days_of_year() + 1;
        }

        if b_start >= a_start && b_end <= a_end {
            return b_end.days_of_year() - b_start.days_of_year() + 1;
        }

        if a_start < b_start {
            a_end.days_of_year() - b_start.days_of_year() + 1
        } else {
            b_end.days_of_year() - a_start.days_of_year() + 1
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Date {
    month: i32,
    day: i32,
}

impl std::str::FromStr for Date {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Result<Vec<i32>, _> = s.split('-').map(|part| part.parse::<i32>()).collect();

        parts
            .map(|parts| Self {
                month: parts[0],
                day: parts[1],
            })
            .map_err(|_| ())
    }
}

impl Date {
    const DAYS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    fn days_of_year(&self) -> i32 {
        Self::DAYS[..(self.month as usize - 1)].iter().sum::<i32>() + self.day
    }
}
