pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn insert_greatest_common_divisors(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut current = &mut head;

        while let Some(node) = current {
            if let Some(next_node) = &node.next {
                let a = node.val;
                let b = next_node.val;
                let gcd = Solution::gcd(a, b);

                let next = node.next.take();
                node.next = Some(Box::new(ListNode { val: gcd, next }));

                current = &mut node.next.as_mut().unwrap().next;
            } else {
                break;
            }
        }

        head
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        if a > b {
            let tmp = a;
            a = b;
            b = tmp;
        }

        loop {
            let rem = b % a;
            b = a;
            a = rem;

            if rem == 0 {
                break;
            }
        }

        b
    }
}
