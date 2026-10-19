pub struct Solution;

use std::{cell::RefCell, rc::Rc, sync::OnceLock};

const MOD: i64 = 1_000_000_007;
const SIZE: usize = 1000;
static PASCAL: OnceLock<Vec<Vec<i32>>> = OnceLock::new();

impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let pascal = PASCAL.get_or_init(Self::init_pascal);

        let mut bst = None;
        for num in nums.iter() {
            Self::build_bst(&mut bst, *num);
        }

        Self::count_comb(pascal, &bst) - 1
    }

    fn init_pascal() -> Vec<Vec<i32>> {
        let mut pascal = vec![vec![0; SIZE]; SIZE];
        pascal[0][0] = 1;
        for layer in 1..pascal.len() {
            pascal[layer][0] = 1;
            for i in 1..pascal.len() {
                pascal[layer][i] =
                    ((pascal[layer - 1][i - 1] as i64 + pascal[layer - 1][i] as i64) % MOD) as i32;
            }
        }
        pascal
    }

    fn build_bst(root: &mut Option<Rc<RefCell<TreeNode>>>, num: i32) {
        if let Some(node) = root {
            node.borrow_mut().len += 1;
            if num > node.borrow().value {
                Self::build_bst(&mut node.borrow_mut().right, num);
            } else {
                Self::build_bst(&mut node.borrow_mut().left, num);
            }
        } else {
            *root = Some(Rc::new(RefCell::new(TreeNode {
                value: num,
                len: 1,
                left: None,
                right: None,
            })));
        }
    }

    fn count_comb(pascal: &[Vec<i32>], root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let l = Self::count_comb(pascal, &node.borrow().left) as i64;
            let r = Self::count_comb(pascal, &node.borrow().right) as i64;

            let len = node.borrow().len;
            let left_len = node
                .borrow()
                .left
                .clone()
                .map(|left_node| left_node.borrow().len)
                .unwrap_or(0);

            (pascal[len - 1][left_len] as i64 * l % MOD * r % MOD) as i32
        } else {
            1
        }
    }
}

#[derive(Clone)]
struct TreeNode {
    value: i32,
    len: usize,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

#[test]
fn test_solution() {
    assert_eq!(Solution::num_of_ways(vec![1, 2, 3]), 0);
    assert_eq!(
        Solution::num_of_ways(vec![6, 9, 11, 15, 1, 12, 3, 2, 7, 8, 14, 4, 5, 13, 10]),
        840839
    );
}
