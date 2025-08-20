pub struct Solution;

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let [y, m, d]: [i32; 3] = date.split('-')
            .map(|num_str| num_str.parse().unwrap())
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();
        format!("{:b}-{:b}-{:b}", y, m, d)
    }
}
