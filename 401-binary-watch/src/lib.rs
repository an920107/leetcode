pub struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        (0..720)
            .map(Time::from)
            .filter(|t| t.count_ones() == turned_on as u32)
            .map(|t| t.to_string())
            .collect()
    }
}

struct Time {
    hour: u8,
    minute: u8,
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{:02}", self.hour, self.minute)
    }
}

impl From<u32> for Time {
    fn from(minutes: u32) -> Self {
        Self {
            hour: (minutes / 60) as u8,
            minute: (minutes % 60) as u8,
        }
    }
}

impl Time {
    fn count_ones(&self) -> u32 {
        self.hour.count_ones() + self.minute.count_ones()
    }
}
