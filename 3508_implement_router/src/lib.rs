use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Packet {
    source: i32,
    destination: i32,
    timestamp: i32,
}

pub struct Router {
    limit: usize,
    set: HashSet<Packet>,
    queue: VecDeque<Packet>,
    dest_map: HashMap<i32, VecDeque<Packet>>,
}

impl Router {
    pub fn new(memory_limit: i32) -> Self {
        Self {
            limit: memory_limit as usize,
            set: HashSet::with_capacity(memory_limit as usize),
            queue: VecDeque::with_capacity(memory_limit as usize),
            dest_map: HashMap::new(),
        }
    }

    pub fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let packet = Packet {
            source,
            destination,
            timestamp,
        };

        if self.set.contains(&packet) {
            return false;
        }

        if self.limit <= self.queue.len() {
            let popped_packet = self.queue.pop_front().unwrap();
            self.set.remove(&popped_packet);
            self.dest_map
                .get_mut(&popped_packet.destination)
                .unwrap()
                .pop_front();
        }

        self.set.insert(packet);
        self.queue.push_back(packet);
        self.dest_map
            .entry(packet.destination)
            .or_insert(VecDeque::new())
            .push_back(packet);

        true
    }

    pub fn forward_packet(&mut self) -> Vec<i32> {
        if let Some(packet) = self.queue.pop_front() {
            self.set.remove(&packet);
            self.dest_map
                .get_mut(&packet.destination)
                .unwrap()
                .pop_front();
            vec![packet.source, packet.destination, packet.timestamp]
        } else {
            vec![]
        }
    }

    pub fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if let Some(packets) = self.dest_map.get(&destination) {
            let start_index = packets.partition_point(|packet| packet.timestamp < start_time);
            let end_index = packets.partition_point(|packet| packet.timestamp <= end_time);
            (end_index - start_index) as i32
        } else {
            0
        }
    }
}
