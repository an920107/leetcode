use std::hash::{DefaultHasher, Hash, Hasher};

pub struct MyHashMap {
    capacity: usize,
    buckets: Vec<Vec<(i32, i32)>>,
}

impl MyHashMap {
    pub fn new() -> Self {
        const CAPACITY: usize = 4096;
        Self {
            capacity: CAPACITY,
            buckets: vec![Vec::new(); CAPACITY],
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let index = self.get_bucket_index(key);
        let list = &mut self.buckets[index];

        let node = list.iter_mut().find(|node| node.0 == key);
        match node {
            Some(node) => {
                node.1 = value;
            }
            None => {
                list.push((key, value));
            }
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let index = self.get_bucket_index(key);
        let list = &self.buckets[index];

        let node = list.iter().find(|node| node.0 == key);
        match node {
            Some(node) => node.1,
            None => -1,
        }
    }

    pub fn remove(&mut self, key: i32) {
        let index = self.get_bucket_index(key);
        let list = &mut self.buckets[index];

        if let Some(index_to_remove) = list.iter().position(|node| node.0 == key) {
            list.remove(index_to_remove);
        }
    }

    fn get_bucket_index(&mut self, key: i32) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize & (self.capacity - 1)
    }
}
