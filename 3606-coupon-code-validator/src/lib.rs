pub struct Solution;

#[derive(PartialEq, Eq)]
struct Coupon {
    code: String,
    business_line: String,
    is_active: bool,
}

impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        let coupons: Vec<Coupon> = code
            .into_iter()
            .zip(business_line)
            .zip(is_active)
            .map(|((code, business_line), is_active)| Coupon::new(code, business_line, is_active))
            .collect();

        let mut valid_coupons: Vec<Coupon> = coupons
            .into_iter()
            .filter(|coupon| coupon.is_valid())
            .collect();

        valid_coupons.sort();

        valid_coupons
            .into_iter()
            .map(|coupon| coupon.code)
            .collect()
    }
}

impl Coupon {
    fn new(code: String, business_line: String, is_active: bool) -> Self {
        Self {
            code,
            business_line,
            is_active,
        }
    }

    fn is_valid(&self) -> bool {
        let is_code_valid = !self.code.is_empty()
            && self.code.bytes().all(|c| {
                let c = c as char;
                c == '_'
                    || (c >= 'a' && c <= 'z')
                    || (c >= 'A' && c <= 'Z')
                    || (c >= '0' && c <= '9')
            });

        is_code_valid
            && (self.business_line == "electronics"
                || self.business_line == "grocery"
                || self.business_line == "pharmacy"
                || self.business_line == "restaurant")
            && self.is_active
    }
}

impl Ord for Coupon {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.business_line
            .cmp(&other.business_line)
            .then(self.code.cmp(&other.code))
            .then(self.is_active.cmp(&other.is_active))
    }
}

impl PartialOrd for Coupon {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
