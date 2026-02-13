pub struct Solution;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let mut segment_tree = SegmentTree::new(nums.len() + 1);
        let mut last_seen: HashMap<i32, usize> = HashMap::new();

        let mut result = 0;
        let mut current_sum = 0;
        for (index, num) in nums.iter().enumerate() {
            let delta = if *num % 2 == 0 { 1 } else { -1 };
            if let Some(last_seen_index) = last_seen.get(num) {
                segment_tree.range_add(*last_seen_index + 1, index + 1, -delta);
            } else {
                segment_tree.range_add(index + 1, nums.len() + 2, delta);
                current_sum += delta;
            }
            last_seen.insert(*num, index);

            if let Some(fisrt_index) = segment_tree.find_first(0, index, current_sum) {
                result = result.max(index - fisrt_index + 1);
            }
        }

        result as i32
    }
}

struct TreeNode {
    val: (i32, i32),
    left: usize,
    right: usize,
    left_node: Option<Rc<RefCell<TreeNode>>>,
    right_node: Option<Rc<RefCell<TreeNode>>>,
    lazy_delta: Option<i32>,
}

struct SegmentTree {
    root: Option<Rc<RefCell<TreeNode>>>,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        let left = 0;
        let right = size;
        let root = SegmentTree::build(left, right);
        SegmentTree { root }
    }

    fn build(left: usize, right: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if left + 1 == right {
            return Some(Rc::new(RefCell::new(TreeNode {
                val: (0, 0),
                left,
                right,
                left_node: None,
                right_node: None,
                lazy_delta: None,
            })));
        }

        let mid = (left + right) / 2;

        let left_node = SegmentTree::build(left, mid);
        let right_node = SegmentTree::build(mid, right);
        let root = TreeNode {
            val: (0, 0),
            left,
            right,
            left_node,
            right_node,
            lazy_delta: None,
        };

        Some(Rc::new(RefCell::new(root)))
    }

    fn range_add(&mut self, left: usize, right: usize, delta: i32) {
        self.root
            .clone()
            .map(|node| node.borrow_mut().update(left, right, delta));
    }

    fn find_first(&mut self, left: usize, right: usize, target: i32) -> Option<usize> {
        self.root
            .clone()
            .map(|node| node.borrow_mut().find_first(left, right, target))
            .flatten()
    }
}

impl TreeNode {
    fn update(&mut self, updated_left: usize, updated_right: usize, delta: i32) {
        if updated_right <= self.left || self.right <= updated_left {
            return;
        }

        if updated_left <= self.left && self.right <= updated_right {
            self.apply(delta);
            return;
        }

        self.push_down();

        let mid = (self.left + self.right) / 2;
        if updated_left < mid {
            self.left_node
                .clone()
                .map(|node| node.borrow_mut().update(updated_left, updated_right, delta));
        }
        if updated_right > mid {
            self.right_node
                .clone()
                .map(|node| node.borrow_mut().update(updated_left, updated_right, delta));
        }
        self.pull_up();
    }

    fn apply(&mut self, delta: i32) {
        self.val.0 += delta;
        self.val.1 += delta;
        match self.lazy_delta {
            None => self.lazy_delta = Some(delta),
            Some(original_delta) => self.lazy_delta = Some(original_delta + delta),
        };
    }

    fn push_down(&mut self) {
        if let Some(delta) = self.lazy_delta {
            self.left_node
                .clone()
                .map(|node| node.borrow_mut().apply(delta));
            self.right_node
                .clone()
                .map(|node| node.borrow_mut().apply(delta));
        }

        self.lazy_delta = None;
    }

    fn pull_up(&mut self) {
        let left_val = self
            .left_node
            .clone()
            .map_or((i32::MAX, i32::MIN), |node| node.borrow().val);
        let right_val = self
            .right_node
            .clone()
            .map_or((i32::MAX, i32::MIN), |node| node.borrow().val);

        let merged_val = (left_val.0.min(right_val.0), left_val.1.max(right_val.1));
        self.val = merged_val;
    }

    fn find_first(
        &mut self,
        queried_left: usize,
        queried_right: usize,
        target: i32,
    ) -> Option<usize> {
        if queried_right < self.left || self.right <= queried_left {
            return None;
        }

        if target < self.val.0 || self.val.1 < target {
            return None;
        }

        if self.left + 1 == self.right {
            return Some(self.left);
        }

        self.push_down();

        let mut result = self
            .left_node
            .clone()
            .map(|node| {
                node.borrow_mut()
                    .find_first(queried_left, queried_right, target)
            })
            .flatten();

        if result.is_none() {
            result = self
                .right_node
                .clone()
                .map(|node| {
                    node.borrow_mut()
                        .find_first(queried_left, queried_right, target)
                })
                .flatten();
        }

        result
    }
}
