#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => {
                let left_depth = Solution::dfs(node.borrow().left.clone());
                let right_depth = Solution::dfs(node.borrow().right.clone());

                if let Some(left_depth) = left_depth
                    && let Some(right_depth) = right_depth
                {
                    if left_depth.abs_diff(right_depth) > 1 {
                        false
                    } else {
                        true
                    }
                } else {
                    false
                }
            }
            None => true,
        }
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        match root {
            Some(node) => {
                let left_depth = Solution::dfs(node.borrow().left.clone());
                let right_depth = Solution::dfs(node.borrow().right.clone());

                if let Some(left_depth) = left_depth
                    && let Some(right_depth) = right_depth
                {
                    if left_depth.abs_diff(right_depth) > 1 {
                        None
                    } else {
                        Some(left_depth.max(right_depth) + 1)
                    }
                } else {
                    None
                }
            }
            None => Some(0),
        }
    }
}
