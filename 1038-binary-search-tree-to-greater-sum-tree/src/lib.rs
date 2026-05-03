pub struct Solution;

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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::recursion(root.clone(), &mut 0);
        root
    }

    fn recursion(root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if root.is_none() {
            return;
        }
        let root = root.unwrap();
        let mut root_mut_ref = root.borrow_mut();

        Solution::recursion(root_mut_ref.right.clone(), sum);
        let root_val = &mut root_mut_ref.val;
        *root_val += *sum;
        *sum = *root_val;
        Solution::recursion(root_mut_ref.left.clone(), sum);
    }
}
