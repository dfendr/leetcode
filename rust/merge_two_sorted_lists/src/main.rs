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

fn merge_two_lists(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let (mut list1, mut list2) = (v1, v2);

    while !list1.is_empty() && !list2.is_empty() {
        match list1.last().unwrap() >= list2.last().unwrap() {
            true => result.push(list1.pop().unwrap()),
            false => result.push(list2.pop().unwrap()),
        }
    }
    // Likely that there are left over items in one of the lists
    match list1.len() > list2.len() {
        true => {
            while !list1.is_empty() {
                result.push(list1.pop().unwrap())
            }
        }
        false => {
            while !list2.is_empty() {
                result.push(list2.pop().unwrap())
            }
        }
    }

    // Reverse sorting order in place
    result.reverse();
    result
}

struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // start list, sentinel node
        let mut pre_head = Some(Box::new(ListNode::new(0)));
        let mut current = &mut pre_head;

        while let (Some(ref mut node1), Some(ref mut node2)) = (&mut list1, &mut list2) {
            if node1.val < node2.val {
                let next = node1.next.take(); // detaches the node while keeping original linked list intact.
                current.as_mut()?.next = list1;
                list1 = next;
            } else {
                let next = node2.next.take();
                current.as_mut()?.next = list2;
                list2 = next;
            }
            current = &mut current.as_mut()?.next;
        }
        current.as_mut()?.next = if list1.is_some() { list1 } else { list2 };

        pre_head?.next
    }
}

#[test]
fn test_merge_two_lists() {
    let l1 = vec![1, 2, 4, 4];
    let l2 = vec![1, 3, 4];
    let expected = vec![1, 1, 2, 3, 4, 4, 4];
    assert_eq!(merge_two_lists(l1, l2), expected);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &v in vec.iter().rev() {
            let mut node = ListNode::new(v);
            node.next = current;
            current = Some(Box::new(node));
        }
        current
    }

    #[test]
    fn test_empty_lists() {
        assert_eq!(Solution::merge_two_lists(None, None), None);
    }

    #[test]
    fn test_single_element_lists() {
        assert_eq!(
            Solution::merge_two_lists(to_list(vec![1]), to_list(vec![2])),
            to_list(vec![1, 2])
        );
    }

    #[test]
    fn test_multiple_elements() {
        assert_eq!(
            Solution::merge_two_lists(to_list(vec![1, 3, 5]), to_list(vec![2, 4, 6])),
            to_list(vec![1, 2, 3, 4, 5, 6])
        );
    }

    #[test]
    fn test_mixed_empty_and_non_empty() {
        assert_eq!(
            Solution::merge_two_lists(None, to_list(vec![1, 3, 5])),
            to_list(vec![1, 3, 5])
        );
        assert_eq!(
            Solution::merge_two_lists(to_list(vec![2, 4, 6]), None),
            to_list(vec![2, 4, 6])
        );
    }
}
