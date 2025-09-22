use std::hash::{DefaultHasher, Hash, Hasher};

pub struct MyHashSet {
    capacity: usize,
    buckets: Vec<Vec<i32>>,
}

impl MyHashSet {
    pub fn new() -> Self {
        const CAPACITY: usize = 4096;
        Self {
            capacity: CAPACITY,
            buckets: vec![Vec::new(); CAPACITY],
        }
    }

    pub fn add(&mut self, key: i32) {
        let index = self.get_bucket_index(key);
        let list = self.buckets.get_mut(index).unwrap();

        if list.iter().find(|num| **num == key).is_none() {
            list.push(key);
        }
    }

    pub fn remove(&mut self, key: i32) {
        let index = self.get_bucket_index(key);
        let list = self.buckets.get_mut(index).unwrap();

        if let Some(index_to_remove) = list.iter().position(|num| *num == key) {
            list.remove(index_to_remove);
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        let index = self.get_bucket_index(key);
        let list = self.buckets.get(index).unwrap();

        list.iter().find(|num| **num == key).is_some()
    }

    fn get_bucket_index(&self, key: impl Hash) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) & (self.capacity - 1)
    }
}
