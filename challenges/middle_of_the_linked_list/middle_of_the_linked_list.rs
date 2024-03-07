// https://leetcode.com/problems/middle-of-the-linked-list/

use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut walker = head;
        let mut runner = unsafe { &*(&walker as *const Option<Box<ListNode>>) };
        while runner.is_some() && runner.as_ref().unwrap().next.is_some() {
            runner = &runner.as_ref().unwrap().next.as_ref().unwrap().next;
            walker = walker.unwrap().next;
        }
        walker
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let expected = ListNode::from_slice(&[3, 4, 5]);
        assert_eq!(Solution::middle_node(head), expected);
    }

    #[test]
    fn ex2() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5, 6]);
        let expected = ListNode::from_slice(&[4, 5, 6]);
        assert_eq!(Solution::middle_node(head), expected);
    }

    #[test]
    fn myex1() {
        let head = ListNode::from_slice(&[1]);
        let expected = ListNode::from_slice(&[1]);
        assert_eq!(Solution::middle_node(head), expected);
    }

    #[test]
    fn myex2() {
        let head = ListNode::from_slice(&[1, 2]);
        let expected = ListNode::from_slice(&[2]);
        assert_eq!(Solution::middle_node(head), expected);
    }

    #[test]
    fn myex3() {
        let head = ListNode::from_slice(&[1, 2, 3]);
        let expected = ListNode::from_slice(&[2, 3]);
        assert_eq!(Solution::middle_node(head), expected);
    }

    #[test]
    fn myex4() {
        let head = ListNode::from_slice(&[1, 2, 3, 4]);
        let expected = ListNode::from_slice(&[3, 4]);
        assert_eq!(Solution::middle_node(head), expected);
    }

    #[test]
    fn myex5() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let expected = ListNode::from_slice(&[3, 4, 5]);
        assert_eq!(Solution::middle_node(head), expected);
    }
}
