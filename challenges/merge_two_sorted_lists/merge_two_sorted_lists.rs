// https://leetcode.com/problems/merge-two-sorted-lists/

use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;

        while let (Some(l1), Some(l2)) = (&list1, &list2) {
            if l1.val < l2.val {
                *tail = list1.take();
                let reference = tail.as_mut().unwrap();
                list1 = reference.next.take();
                tail = &mut reference.next;
            } else {
                *tail = list2.take();
                let reference = tail.as_mut().unwrap();
                list2 = reference.next.take();
                tail = &mut reference.next;
            }
        }

        *tail = list1.or(list2);

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let list1 = ListNode::from_slice(&[1, 2, 4]);
        let list2 = ListNode::from_slice(&[1, 3, 4]);
        let expected = ListNode::from_slice(&[1, 1, 2, 3, 4, 4]);
        assert_eq!(Solution::merge_two_lists(list1, list2), expected,);
    }

    #[test]
    fn ex2() {
        let list1 = ListNode::from_slice(&[]);
        let list2 = ListNode::from_slice(&[]);
        let expected = ListNode::from_slice(&[]);
        assert_eq!(Solution::merge_two_lists(list1, list2), expected,);
    }

    #[test]
    fn ex3() {
        let list1 = ListNode::from_slice(&[]);
        let list2 = ListNode::from_slice(&[0]);
        let expected = ListNode::from_slice(&[0]);
        assert_eq!(Solution::merge_two_lists(list1, list2), expected,);
    }

    #[test]
    fn myex1() {
        let list1 = ListNode::from_slice(&[1, 2, 3]);
        let list2 = ListNode::from_slice(&[4, 5, 6]);
        let expected = ListNode::from_slice(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(Solution::merge_two_lists(list1, list2), expected,);
    }

    #[test]
    fn myex2() {
        let list1 = ListNode::from_slice(&[1, 2, 3]);
        let list2 = ListNode::from_slice(&[1, 2, 3]);
        let expected = ListNode::from_slice(&[1, 1, 2, 2, 3, 3]);
        assert_eq!(Solution::merge_two_lists(list1, list2), expected,);
    }

    #[test]
    fn myex3() {
        let list1 = ListNode::from_slice(&[1, 2, 3]);
        let list2 = ListNode::from_slice(&[1, 2, 3, 4, 5, 6]);
        let expected = ListNode::from_slice(&[1, 1, 2, 2, 3, 3, 4, 5, 6]);
        assert_eq!(Solution::merge_two_lists(list1, list2), expected,);
    }
}
