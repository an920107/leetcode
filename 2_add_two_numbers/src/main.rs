#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result_head = None;
        let mut result_current = &mut result_head;
        let mut current1 = l1.as_deref();
        let mut current2 = l2.as_deref();

        let mut carry = 0;

        loop {
            if current1 == None && current2 == None {
                if carry > 0 {
                    *result_current = Some(Box::new(ListNode::new(carry)));
                }
                break;
            }

            let n1 = current1.map_or(0, |v| {
                current1 = v.next.as_deref();
                v.val
            });
            let n2 = current2.map_or(0, |v| {
                current2 = v.next.as_deref();
                v.val
            });

            let mut sum = n1 + n2 + carry;
            carry = sum / 10;
            sum %= 10;

            *result_current = Some(Box::new(ListNode::new(sum)));
            result_current = &mut result_current.as_mut().unwrap().next;
        }
        result_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl ListNode {
        pub fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
            let mut head = None;
            let mut current = &mut head;
            for i in v {
                *current = Some(Box::new(ListNode::new(i)));
                current = &mut current.as_mut().unwrap().next;
            }
            head
        }

        pub fn to_vec(&self) -> Vec<i32> {
            let mut result = vec![];
            let mut current = Some(self);
            while let Some(node) = current {
                result.push(node.val);
                current = node.next.as_deref();
            }
            result
        }
    }

    #[test]
    fn testcase_1() {
        let l1 = ListNode::from_vec(vec![2, 4, 3]);
        let l2 = ListNode::from_vec(vec![5, 6, 4]);
        let expected = vec![7, 0, 8];
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(result.unwrap().to_vec(), expected);
    }

    #[test]
    fn testcase_2() {
        let l1 = ListNode::from_vec(vec![0]);
        let l2 = ListNode::from_vec(vec![0]);
        let expected = vec![0];
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(result.unwrap().to_vec(), expected);
    }

    #[test]
    fn testcase_3() {
        let l1 = ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from_vec(vec![9, 9, 9, 9]);
        let expected = vec![8, 9, 9, 9, 0, 0, 0, 1];
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(result.unwrap().to_vec(), expected);
    }
}

fn main() {}
