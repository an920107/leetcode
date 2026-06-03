pub struct Solution;

impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        let land_rides = Self::to_rides(land_start_time, land_duration);
        let water_rides = Self::to_rides(water_start_time, water_duration);

        let min_land_end_time = land_rides
            .iter()
            .map(|ride| ride.start_time + ride.duration)
            .min()
            .unwrap();
        let land_result = water_rides
            .iter()
            .map(|ride| {
                min_land_end_time + ride.duration + (ride.start_time - min_land_end_time).max(0)
            })
            .min()
            .unwrap();

        let min_water_end_time = water_rides
            .iter()
            .map(|ride| ride.start_time + ride.duration)
            .min()
            .unwrap();
        let water_result = land_rides
            .iter()
            .map(|ride| {
                min_water_end_time + ride.duration + (ride.start_time - min_water_end_time).max(0)
            })
            .min()
            .unwrap();

        land_result.min(water_result)
    }

    fn to_rides(start_times: Vec<i32>, durations: Vec<i32>) -> Vec<Ride> {
        start_times
            .into_iter()
            .zip(durations.into_iter())
            .map(|(s, d)| Ride {
                start_time: s,
                duration: d,
            })
            .collect()
    }
}

struct Ride {
    start_time: i32,
    duration: i32,
}
