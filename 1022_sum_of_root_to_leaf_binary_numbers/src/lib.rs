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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        Solution::recursion(root, 0, &mut result);
        result
    }

    fn recursion(root: Option<Rc<RefCell<TreeNode>>>, num: i32, result: &mut i32) {
        if root.is_none() {
            return;
        }

        let root = root.unwrap();
        let bit = root.borrow().val;
        let new_num = num << 1 | bit;

        if root.borrow().left.is_none() && root.borrow().right.is_none() {
            *result += new_num;
            return;
        }

        Solution::recursion(root.borrow().left.clone(), new_num, result);
        Solution::recursion(root.borrow().right.clone(), new_num, result);
    }
}
