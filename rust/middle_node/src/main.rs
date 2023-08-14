fn main() {}
/*
Given the head of a singly linked list, return the middle node of the linked list.

If there are two middle nodes, return the second middle node.
*/

// Definition for singly-linked list.
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

struct Solution {}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut count = 0;
        let mut current = head.as_ref();
        while let Some(node) = current {
            count += 1;
            current = node.next.as_ref();
        }
        let middle = count / 2;
        current = head.as_ref();
        for _ in 0..middle {
            current = current
                .expect("Node should be present -- Middle calculted incorrectly")
                .next
                .as_ref();
        }
        current.cloned()
    }
}
