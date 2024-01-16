// https://leetcode.com/problems/merge-k-sorted-lists/

use super::utils::ListNode;

pub struct Solution;

use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Debug, Clone)]
struct OrderedNodeHolder {
    node: Box<ListNode>,
}

impl PartialOrd for OrderedNodeHolder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.node.val.partial_cmp(&other.node.val)
    }
}

impl Ord for OrderedNodeHolder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.node.val.cmp(&other.node.val)
    }
}

struct ListIntoIterator {
    node: Option<Box<ListNode>>,
}

impl ListIntoIterator {
    fn new(node: Option<Box<ListNode>>) -> Self {
        Self { node }
    }
}

impl Iterator for ListIntoIterator {
    type Item = Box<ListNode>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut node = self.node.take()?;
        self.node = node.next.take();
        Some(node)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = lists
            .into_iter()
            .flat_map(ListIntoIterator::new)
            .map(|x| OrderedNodeHolder { node: x })
            .collect::<BinaryHeap<_>>();

        let mut new_head = None;
        while let Some(OrderedNodeHolder { node }) = heap.pop() {
            let mut node = node;
            let old_next = std::mem::replace(&mut node.next, new_head);
            new_head = Some(node);
            if let Some(old_next) = old_next {
                heap.push(OrderedNodeHolder { node: old_next });
            }
        }

        new_head
    }
}

#[cfg(test)]
mod test {
    use super::{ListNode, Solution};

    #[test]
    fn ex1() {
        let input = vec![
            ListNode::from_slice(&[1, 4, 5]),
            ListNode::from_slice(&[1, 3, 4]),
            ListNode::from_slice(&[2, 6]),
        ];
        let output = Solution::merge_k_lists(input);
        let expected = ListNode::from_slice(&[1, 1, 2, 3, 4, 4, 5, 6]);
        assert_eq!(output, expected);
    }

    #[test]
    fn ex1_simpler() {
        let input = vec![
            ListNode::from_slice(&[1, 4, 5]),
            ListNode::from_slice(&[1, 3, 4]),
        ];
        let output = Solution::merge_k_lists(input);
        let expected = ListNode::from_slice(&[1, 1, 3, 4, 4, 5]);
        assert_eq!(output, expected);
    }

    #[test]
    fn empty() {
        let input = vec![];
        let output = Solution::merge_k_lists(input);
        let expected = None;
        assert_eq!(output, expected);
    }

    #[test]
    fn empty2() {
        let input = vec![None];
        let output = Solution::merge_k_lists(input);
        let expected = None;
        assert_eq!(output, expected);
    }

    #[test]
    fn empty3() {
        let input = vec![None, None];
        let output = Solution::merge_k_lists(input);
        let expected = None;
        assert_eq!(output, expected);
    }
}
