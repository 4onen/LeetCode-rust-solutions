// https://leetcode.com/problems/cousins-in-binary-tree-ii/

use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn replace_value_in_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn summing_dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            level_sums: &mut Vec<i32>,
            depth: usize,
        ) {
            if let Some(n) = node {
                let n = n.borrow();
                if level_sums.len() <= depth {
                    level_sums.push(0);
                }
                level_sums[depth] += n.val;
                summing_dfs(&n.left, level_sums, depth + 1);
                summing_dfs(&n.right, level_sums, depth + 1);
            }
        }
        fn changing_dfs(
            node: &mut TreeNode,
            level_sums: &Vec<i32>,
            depth: usize,
            sibling_val: i32,
        ) {
            node.val = level_sums[depth] - sibling_val - node.val;
            let left_val = if let Some(ref left) = node.left {
                left.borrow().val
            } else {
                0
            };
            let right_val = if let Some(ref right) = node.right {
                right.borrow().val
            } else {
                0
            };
            if let Some(ref mut left) = node.left {
                changing_dfs(&mut left.borrow_mut(), level_sums, depth + 1, right_val);
            }
            if let Some(ref mut right) = node.right {
                changing_dfs(&mut right.borrow_mut(), level_sums, depth + 1, left_val);
            }
        }
        let Some(root) = root else {
            return None;
        };
        let mut level_sums = Vec::new();
        summing_dfs(&Some(root.clone()), &mut level_sums, 0);
        {
            let mut root_ref = root.borrow_mut();
            changing_dfs(&mut root_ref, &level_sums, 0, 0);
        }
        Some(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &[Option<i32>], expected: &[Option<i32>]) {
        let root = TreeNode::from_leetcode_slice(input);
        let expected_tree = TreeNode::from_leetcode_slice(expected);
        assert_eq!(Solution::replace_value_in_tree(root), expected_tree);
    }

    #[test]
    fn ex1() {
        test(
            &[Some(5), Some(4), Some(9), Some(1), Some(10), None, Some(7)],
            &[Some(0), Some(0), Some(0), Some(7), Some(7), None, Some(11)],
        )
    }

    #[test]
    fn ex2() {
        test(&[Some(3), Some(1), Some(2)], &[Some(0), Some(0), Some(0)])
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                Some(436),
                Some(623),
                Some(376),
                Some(117),
                Some(698),
                Some(467),
                Some(818),
                Some(52),
                Some(543),
                Some(880),
                Some(577),
                Some(700),
                Some(568),
                Some(361),
                None,
                Some(616),
                None,
                Some(232),
                Some(656),
                Some(565),
                Some(12),
                None,
                Some(95),
                None,
                None,
                None,
                Some(389),
                Some(830),
                None,
                Some(276),
                None,
                Some(715),
                None,
                Some(144),
                None,
                Some(317),
                None,
                None,
                Some(91),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(129),
                Some(362),
                Some(487),
                Some(272),
                Some(275),
                None,
                None,
                Some(908),
                Some(559),
                None,
                None,
                None,
                None,
                None,
                Some(862),
                None,
                None,
                None,
                None,
                Some(68),
                Some(63),
                None,
                Some(467),
                None,
                Some(274),
                None,
                None,
                None,
                None,
                None,
                Some(920),
                None,
                Some(300),
            ],
            &[
                Some(0),
                Some(0),
                Some(0),
                Some(1285),
                Some(1285),
                Some(815),
                Some(815),
                Some(3086),
                Some(3086),
                Some(2224),
                Some(2224),
                Some(2413),
                Some(2413),
                Some(3320),
                None,
                Some(2779),
                None,
                Some(2507),
                Some(2507),
                Some(2818),
                Some(2818),
                None,
                Some(3300),
                None,
                None,
                None,
                Some(3006),
                Some(2565),
                None,
                Some(1267),
                None,
                Some(828),
                None,
                Some(1399),
                None,
                Some(1226),
                None,
                None,
                Some(1452),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(2501),
                Some(2501),
                Some(2233),
                Some(2233),
                Some(2717),
                None,
                None,
                Some(2084),
                Some(2433),
                None,
                None,
                None,
                None,
                None,
                Some(598),
                None,
                None,
                None,
                None,
                Some(1392),
                Some(1397),
                None,
                Some(993),
                None,
                Some(920),
                None,
                None,
                None,
                None,
                None,
                Some(274),
                None,
                Some(0),
            ],
        )
    }

    #[test]
    fn my_extreme_ex1() {
        let input = &[Some(10_000); 100_000];
        // Don't-crash test
        Solution::replace_value_in_tree(TreeNode::from_leetcode_slice(input));
    }
}
