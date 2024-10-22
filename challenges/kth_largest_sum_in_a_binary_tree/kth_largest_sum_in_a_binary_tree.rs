// https://leetcode.com/problems/kth-largest-sum-in-a-binary-tree/

use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, level: usize, sums: &mut Vec<i64>) {
            if let Some(node) = node {
                if sums.len() <= level {
                    sums.push(0);
                }
                sums[level] += node.borrow().val as i64;
                dfs(&node.borrow().left, level + 1, sums);
                dfs(&node.borrow().right, level + 1, sums);
            }
        }
        let mut sums = Vec::new();
        dfs(&root, 0, &mut sums);
        if sums.len() < k as usize {
            return -1;
        }
        *(sums.select_nth_unstable_by(k as usize - 1, |a, b| b.cmp(a)).1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &[Option<i32>], k: i32, expected: i64) {
        assert_eq!(
            Solution::kth_largest_level_sum(TreeNode::from_leetcode_slice(input), k),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(
            &[
                Some(5),
                Some(8),
                Some(9),
                Some(2),
                Some(1),
                Some(3),
                Some(7),
                Some(4),
                Some(6),
            ],
            2,
            13,
        )
    }

    #[test]
    fn ex2() {
        test(
            &[Some(1),Some(2),None,Some(3)],
            1,
            3,
        )
    }

    #[test]
    fn ex2_1() {
        test(
            &[Some(1),Some(2),None,Some(3)],
            3,
            1,
        )
    }

    #[test]
    fn ex2_2() {
        test(
            &[Some(1),Some(2),None,Some(3)],
            4,
            -1,
        )
    }

    #[test]
    fn failing_case1() {
        // Only failed because LeetCode wouldn't let me use ex2_2.
        test(
            &[Some(5),Some(8),Some(9),Some(2),Some(1),Some(3),Some(7)],
            4,
            -1
        )
    }
}
