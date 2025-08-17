use std::ops::Deref;

fn main() {
    println!("Hello, world!");
}

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

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut arr: Vec<i32> = vec![];
        let mut current_node = head;

        while let Some(node) = current_node.take() {
            arr.push(node.val);
            current_node = node.next;
        }

        if arr.is_empty() {
            return None;
        }

        let mut head = Some(Box::new(ListNode::new(*arr.last().unwrap())));
        let mut current_node = &mut head;

        for &num in arr[..arr.len() - 1].iter().rev() {
            current_node.as_mut().unwrap().next = Some(Box::new(ListNode::new(num)));
            current_node = &mut current_node.as_mut().unwrap().next;
        }

        head
    }

    pub fn reverse_list_2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut p: Option<Box<ListNode>> = None;

        while let Some(mut node) = head.take() {
            let next = node.next;
            node.next = p;
            p = Some(node);
            head = next;
        }

        p
    }
}
