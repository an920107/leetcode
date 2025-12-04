use std::{collections::HashMap, ops};

pub struct Solution;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct Fraction {
    numerator: i32,
    denominator: u32,
}

impl Solution {
    const MOD: i64 = 1_000_000_007;

    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let mut slopes: HashMap<Fraction, HashMap<Fraction, i64>> = HashMap::new();
        let mut diagonals: HashMap<(i32, i32), HashMap<Fraction, i64>> = HashMap::new();

        let mut i = 0;
        let mut j = 1;
        while i < points.len() {
            while j < points.len() {
                let p = &points[i];
                let q = &points[j];

                let a = Fraction::new(p[1] - q[1], p[0] - q[0]);
                let b = if a.is_finite() {
                    p[1] - p[0] * a
                } else {
                    Fraction::new(p[0], 1)
                };

                if let Some(slope) = slopes.get_mut(&a) {
                    *slope.entry(b).or_default() += 1;
                } else {
                    let mut slope = HashMap::new();
                    slope.insert(b, 1);
                    slopes.insert(a, slope);
                }

                let mid = (p[0] + q[0], p[1] + q[1]);
                if let Some(slopes) = diagonals.get_mut(&mid) {
                    *slopes.entry(a).or_default() += 1;
                } else {
                    let mut slopes = HashMap::new();
                    slopes.insert(a, 1);
                    diagonals.insert(mid, slopes);
                }

                j += 1;
            }
            i += 1;
            j = i + 1;
        }

        let trapezoid = slopes
            .values()
            .map(Solution::count_combination)
            .sum::<i64>();
        let parallelogram = diagonals
            .values()
            .map(Solution::count_combination)
            .sum::<i64>();
        ((trapezoid - parallelogram) % Solution::MOD) as i32
    }

    fn count_combination(counts: &HashMap<Fraction, i64>) -> i64 {
        let sum_square = counts.values().sum::<i64>().pow(2);
        let square_sum = counts.values().map(|c| c * c).sum::<i64>();

        ((sum_square - square_sum) / 2) as i64
    }
}

impl Fraction {
    fn new(n: i32, d: i32) -> Self {
        let is_neg = n.is_negative() != d.is_negative();
        let n = n.unsigned_abs();
        let d = d.unsigned_abs();
        let g = Fraction::gcd(n, d);
        Self {
            numerator: if is_neg && d != 0 {
                -((n / g) as i32)
            } else {
                (n / g) as i32
            },
            denominator: d / g,
        }
    }

    fn gcd(a: u32, b: u32) -> u32 {
        if a < b {
            Fraction::gcd(b, a)
        } else if b == 0 {
            a
        } else {
            Fraction::gcd(b, a % b)
        }
    }

    fn is_finite(&self) -> bool {
        self.denominator != 0
    }
}

impl ops::Mul<Fraction> for i32 {
    type Output = Fraction;

    fn mul(self, rhs: Fraction) -> Self::Output {
        Fraction::new(rhs.numerator * self, rhs.denominator as i32)
    }
}

impl ops::Sub<Fraction> for i32 {
    type Output = Fraction;

    fn sub(self, rhs: Fraction) -> Self::Output {
        Fraction::new(
            rhs.denominator as i32 * self - rhs.numerator,
            rhs.denominator as i32,
        )
    }
}
