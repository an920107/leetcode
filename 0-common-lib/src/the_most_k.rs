use std::{collections::BinaryHeap, ops::Neg};

pub fn the_most_k_of<T>(iter: impl Iterator<Item = T>, k: usize) -> Option<T>
where
    T: Ord + Copy + Neg<Output = T>,
{
    if k == 0 {
        return None;
    }

    let mut heap: BinaryHeap<T> = BinaryHeap::new();
    for item in iter {
        if heap.len() < k {
            heap.push(-item);
        } else if let Some(&top) = heap.peek() {
            if -top < item {
                heap.pop();
                heap.push(-item);
            }
        }
    }

    if heap.len() == k {
        heap.peek().copied().map(|val| -val)
    } else {
        None
    }
}

#[test]
fn test_the_most_k_of() {
    let nums = vec![1, 3, 5, 2, 4];
    assert_eq!(the_most_k_of(nums.iter().copied(), 1), Some(5));
    assert_eq!(the_most_k_of(nums.iter().copied(), 2), Some(4));
    assert_eq!(the_most_k_of(nums.iter().copied(), 5), Some(1));
    assert_eq!(the_most_k_of(nums.iter().copied(), 6), None);
    assert_eq!(the_most_k_of(nums.iter().copied(), 0), None);
}
