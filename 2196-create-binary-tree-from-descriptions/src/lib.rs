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
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let n = descriptions.len();

        let mut node_map: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::with_capacity(n + 1);
        let mut maybe_root: HashMap<i32, bool> = HashMap::with_capacity(n);

        for desc in descriptions.into_iter() {
            let parent = desc[0];
            let child = desc[1];
            let is_left = desc[2] == 1;

            maybe_root.entry(parent).or_insert(true);
            maybe_root.insert(child, false);

            let parent_node = node_map
                .entry(parent)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(parent))))
                .clone();

            let child_node = node_map
                .entry(child)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(child))))
                .clone();

            if is_left {
                parent_node.borrow_mut().left = Some(child_node);
            } else {
                parent_node.borrow_mut().right = Some(child_node);
            }
        }

        maybe_root
            .iter()
            .filter_map(|(k, v)| v.then(|| node_map.get(k).unwrap()))
            .next()
            .cloned()
    }
}
