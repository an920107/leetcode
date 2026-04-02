use std::collections::{BTreeMap, VecDeque, btree_map::Entry};

#[derive(Debug)]
pub struct MKAverage {
    left_map: BTreeMap<i32, usize>,
    mid_map: BTreeMap<i32, usize>,
    right_map: BTreeMap<i32, usize>,
    queue: VecDeque<i32>,
    left_len: usize,
    right_len: usize,
    m: usize,
    k: usize,
}

impl MKAverage {
    pub fn new(m: i32, k: i32) -> Self {
        Self {
            left_map: BTreeMap::new(),
            mid_map: BTreeMap::new(),
            right_map: BTreeMap::new(),
            queue: VecDeque::new(),
            left_len: 0,
            right_len: 0,
            m: m as usize,
            k: k as usize,
        }
    }

    pub fn add_element(&mut self, num: i32) {
        self.queue.push_back(num);

        let num_to_rm = if self.queue.len() > self.m {
            Some(self.queue.pop_front().unwrap())
        } else {
            None
        };

        Self::add_num_to_map(&mut self.mid_map, num);

        if let Some(num) = num_to_rm {
            let mut removed = Self::remove_num_from_map(&mut self.mid_map, num);
            if !removed {
                removed = Self::remove_num_from_map(&mut self.left_map, num);
                if removed {
                    self.left_len -= 1;
                }
            }
            if !removed {
                Self::remove_num_from_map(&mut self.right_map, num);
                self.right_len -= 1;
            }
        }

        self.rebalance();
    }

    pub fn calculate_mk_average(&self) -> i32 {
        if self.queue.len() < self.m {
            return -1;
        }

        let mut sum = 0;
        let mut len = 0;
        for (&num, &cnt) in self.mid_map.iter() {
            sum += num * cnt as i32;
            len += cnt;
        }
        sum / len as i32
    }

    fn rebalance(&mut self) {
        if self.queue.len() >= self.m {
            while self.left_len < self.k {
                let num = *self.mid_map.keys().next().unwrap();
                Self::remove_num_from_map(&mut self.mid_map, num);
                Self::add_num_to_map(&mut self.left_map, num);
                self.left_len += 1;
            }
            while self.right_len < self.k {
                let num = *self.mid_map.keys().last().unwrap();
                Self::remove_num_from_map(&mut self.mid_map, num);
                Self::add_num_to_map(&mut self.right_map, num);
                self.right_len += 1;
            }
        }

        let left_max = *self.left_map.keys().last().unwrap_or(&i32::MIN);
        let mid_min = *self.mid_map.keys().next().unwrap_or(&i32::MAX);
        let mid_max = *self.mid_map.keys().last().unwrap_or(&i32::MIN);
        let right_min = *self.right_map.keys().next().unwrap_or(&i32::MAX);

        if left_max > mid_min {
            Self::remove_num_from_map(&mut self.mid_map, mid_min);
            Self::add_num_to_map(&mut self.left_map, mid_min);
            self.left_len += 1;

            if self.left_len > self.k {
                Self::remove_num_from_map(&mut self.left_map, left_max);
                Self::add_num_to_map(&mut self.mid_map, left_max);
                self.left_len -= 1;
            }
        }

        if right_min < mid_max {
            Self::remove_num_from_map(&mut self.mid_map, mid_max);
            Self::add_num_to_map(&mut self.right_map, mid_max);
            self.right_len += 1;

            if self.right_len > self.k {
                Self::remove_num_from_map(&mut self.right_map, right_min);
                Self::add_num_to_map(&mut self.mid_map, right_min);
                self.right_len -= 1;
            }
        }
    }

    fn add_num_to_map(map: &mut BTreeMap<i32, usize>, num: i32) {
        *map.entry(num).or_default() += 1;
    }

    fn remove_num_from_map(map: &mut BTreeMap<i32, usize>, num: i32) -> bool {
        if let Entry::Occupied(mut entry) = map.entry(num) {
            if *entry.get() <= 1 {
                entry.remove();
            } else {
                *entry.get_mut() -= 1;
            }
            true
        } else {
            false
        }
    }
}
