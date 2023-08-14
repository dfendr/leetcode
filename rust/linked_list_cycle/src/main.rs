use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

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
    pub fn has_cycle(mut list: Option<Box<ListNode>>) -> bool {
        let mut visited: HashSet<usize> = HashSet::new();
        while let Some(ref mut node) = &mut list {
            let address = node as *const _ as usize; // get raw address
            if !visited.insert(address) {
                return true;
            }
            list = node.next.take();
        }
        false
    }
}

