// https://leetcode.com/problems/swap-nodes-in-pairs/

use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.map(|mut headnode| {
            let mut prev = &mut headnode;
            loop {
                // If there is no second node after the one we're looking at,
                // we're done.
                let mut second = if let Some(second) = prev.next.take() {
                    second
                } else {
                    break;
                };
                prev.next = second.next.take();
                let first = std::mem::replace(prev, second);
                prev.next = Some(first);
                prev = if let Some(prev) = prev.next.as_mut().unwrap().next.as_mut() {
                    prev
                } else {
                    break;
                };
            }
            headnode
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = ListNode::from_slice(&[1, 2, 3, 4]);
        let expected = ListNode::from_slice(&[2, 1, 4, 3]);
        assert_eq!(Solution::swap_pairs(input), expected);
    }

    #[test]
    fn ex2() {
        let input = ListNode::from_slice(&[]);
        let expected = ListNode::from_slice(&[]);
        assert_eq!(Solution::swap_pairs(input), expected);
    }

    #[test]
    fn ex3() {
        let input = ListNode::from_slice(&[1]);
        let expected = ListNode::from_slice(&[1]);
        assert_eq!(Solution::swap_pairs(input), expected);
    }

    #[test]
    fn discussion_case1() {
        let input = ListNode::from_slice(&[1, 2, 3]);
        let expected = ListNode::from_slice(&[2, 1, 3]);
        assert_eq!(Solution::swap_pairs(input), expected);
    }
}
