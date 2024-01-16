// https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/

use crate::utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recurse(
            node: std::cell::Ref<'_, TreeNode>,
            max_ancestor: i32,
            min_ancestor: i32,
        ) -> i32 {
            let val = node.val;
            let diff = std::cmp::max(
                (max_ancestor - node.val).abs(),
                (min_ancestor - node.val).abs(),
            );
            let max_ancestor = max_ancestor.max(val);
            let min_ancestor = min_ancestor.min(val);
            let left = match node.left.as_ref() {
                Some(left) => recurse(left.borrow(), max_ancestor, min_ancestor),
                None => 0,
            };
            let right = match node.right.as_ref() {
                Some(right) => recurse(right.borrow(), max_ancestor, min_ancestor),
                None => 0,
            };
            std::cmp::max(diff, std::cmp::max(left, right))
        }
        match root {
            Some(root) => {
                let val = root.borrow().val;
                recurse(root.borrow(), val, val)
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let root = TreeNode::from_leetcode_slice(&[
            Some(8),
            Some(3),
            Some(10),
            Some(1),
            Some(6),
            None,
            Some(14),
            None,
            None,
            Some(4),
            Some(7),
            Some(13),
        ]);
        assert_eq!(Solution::max_ancestor_diff(root), 7);
    }

    #[test]
    fn ex2() {
        let root = TreeNode::from_leetcode_slice(&[Some(1), None, Some(2), None, Some(0), Some(3)]);
        assert_eq!(Solution::max_ancestor_diff(root), 3);
    }
}
