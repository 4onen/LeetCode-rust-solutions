// https://leetcode.com/problems/merge-in-between-linked-lists

use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        assert!(1 <= a);
        assert!(a <= b);
        assert!(b <= 10_000);
        let a = a as u16;
        let b = b as u16;
        let mut output_list = list1;
        let mut current = &mut output_list;
        let mut i = 0;
        while i < a {
            current = &mut current.as_mut().unwrap().next;
            i += 1;
        }
        let mut current_removed = current.take();
        *current = list2;
        while i <= b {
            current_removed = current_removed.unwrap().next.take();
            i += 1;
        }
        while let Some(ref mut node) = current {
            current = &mut node.next;
        }
        *current = current_removed;
        output_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let list1 = ListNode::from_slice(&[10, 1, 13, 6, 9, 5]);
        let list2 = ListNode::from_slice(&[1000000, 1000001, 1000002]);
        let a = 3;
        let b = 4;
        let expected = ListNode::from_slice(&[10, 1, 13, 1000000, 1000001, 1000002, 5]);
        assert_eq!(Solution::merge_in_between(list1, a, b, list2), expected);
    }

    #[test]
    fn ex2() {
        let list1 = ListNode::from_slice(&[0, 1, 2, 3, 4, 5, 6]);
        let list2 = ListNode::from_slice(&[1000000, 1000001, 1000002, 1000003, 1000004]);
        let a = 2;
        let b = 5;
        let expected =
            ListNode::from_slice(&[0, 1, 1000000, 1000001, 1000002, 1000003, 1000004, 6]);
        assert_eq!(Solution::merge_in_between(list1, a, b, list2), expected);
    }

    #[test]
    fn discussion_case1() {
        let list1 = ListNode::from_slice(&[0, 3, 2, 1, 4, 5]);
        let list2 = ListNode::from_slice(&[1000000, 1000001, 1000002]);
        let a = 3;
        let b = 4;
        let expected = ListNode::from_slice(&[0, 3, 2, 1000000, 1000001, 1000002, 5]);
        assert_eq!(Solution::merge_in_between(list1, a, b, list2), expected);
    }

    #[test]
    fn myex0() {
        let list1 = ListNode::from_slice(&[0, 1, 2, 3, 4, 5, 6]);
        let list2 = ListNode::from_slice(&[1000000, 1000001, 1000002]);
        let a = 1;
        let b = 5;
        let expected = ListNode::from_slice(&[0, 1000000, 1000001, 1000002, 6]);
        assert_eq!(Solution::merge_in_between(list1, a, b, list2), expected);
    }

    #[test]
    fn myex1() {
        let list1 = ListNode::from_slice(&[0, 1, 2, 3, 4, 5, 6]);
        let list2 = ListNode::from_slice(&[1000000, 1000001, 1000002]);
        let a = 1;
        let b = 5;
        let expected = ListNode::from_slice(&[0, 1000000, 1000001, 1000002, 6]);
        assert_eq!(Solution::merge_in_between(list1, a, b, list2), expected);
    }
}
