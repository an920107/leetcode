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

#[derive(Eq)]
struct List {
    head: Box<ListNode>,
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.head.val == other.head.val
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.head.val.cmp(&self.head.val)
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut result_head = None;
        let mut result = &mut result_head;

        let mut heap = std::collections::BinaryHeap::<List>::new();
        for l in lists {
            if let Some(head) = l {
                heap.push(List { head: head });
            }
        }

        while let Some(top) = heap.pop() {
            *result = Some(Box::new(ListNode::new(top.head.val)));
            result = &mut result.as_mut().unwrap().next;

            if let Some(new_top_head) = top.head.next {
                heap.push(List { head: new_top_head });
            }
        }

        result_head
    }
}
