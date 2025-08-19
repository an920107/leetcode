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

pub struct Solution;

impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result_head = Some(Box::new(ListNode::new(0)));
        let mut result_current_node = &mut result_head;

        let mut current_node = head.unwrap().next;
        while let Some(node) = current_node.take() {
            if node.val != 0 {
                result_current_node.as_mut().unwrap().val += node.val;
            } else {
                if node.next.as_ref().is_none() {
                    break;
                }

                result_current_node.as_mut().unwrap().next = Some(Box::new(ListNode::new(0)));
                result_current_node = &mut result_current_node.as_mut().unwrap().next;
            }

            current_node = node.next;
        }

        result_head
    }
}
