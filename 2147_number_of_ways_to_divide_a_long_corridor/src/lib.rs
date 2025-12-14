pub struct Solution;

const MOD: usize = 1_000_000_007;

const SEAT: u8 = 'S' as u8;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let mut result = 1;

        let mut current_seat_count = 0;
        let mut last_available_divider: Option<usize> = None;

        for (index, item) in corridor.bytes().enumerate() {
            if let Some(last_divder_index) = last_available_divider {
                if item == SEAT {
                    result = result * (index - last_divder_index) % MOD;

                    current_seat_count = 1;
                    last_available_divider = None;
                }
            } else {
                if item == SEAT {
                    current_seat_count += 1;
                }

                if current_seat_count == 2 {
                    current_seat_count = 0;
                    last_available_divider = Some(index);
                }
            }
        }

        if last_available_divider.is_none() {
            0
        } else {
            result as i32
        }
    }
}
