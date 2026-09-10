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
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root).result
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> DfsOutput {
        if root.is_none() {
            return DfsOutput::default();
        }
        let root = root.unwrap();

        let left = Self::dfs(root.borrow().left.clone());
        let right = Self::dfs(root.borrow().right.clone());

        let new_sum = left.sum + right.sum + root.borrow().val;
        let new_nodes_count = left.nodes_count + right.nodes_count + 1;
        let new_result = left.result
            + right.result
            + if root.borrow().val == new_sum / new_nodes_count {
                1
            } else {
                0
            };

        DfsOutput {
            sum: new_sum,
            nodes_count: new_nodes_count,
            result: new_result,
        }
    }
}

#[derive(Default)]
struct DfsOutput {
    sum: i32,
    nodes_count: i32,
    result: i32,
}
