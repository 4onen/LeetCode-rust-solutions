// https://leetcode.com/problems/diameter-of-binary-tree/

use utils::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        struct RecursionStepResult {
            diameter: u16,
            max_depth: u16,
        }
        fn single_pass_recursion(node: &Option<Rc<RefCell<TreeNode>>>) -> RecursionStepResult {
            match node {
                None => RecursionStepResult {
                    diameter: 0,
                    max_depth: 0,
                },
                Some(node) => {
                    let node = node.borrow();
                    let left = single_pass_recursion(&node.left);
                    let right = single_pass_recursion(&node.right);
                    let diameter_through_me = left.max_depth + right.max_depth;
                    let max_diameter = std::cmp::max(
                        diameter_through_me,
                        std::cmp::max(left.diameter, right.diameter),
                    );
                    let max_depth = 1 + std::cmp::max(left.max_depth, right.max_depth);
                    RecursionStepResult {
                        diameter: max_diameter,
                        max_depth,
                    }
                }
            }
        }
        single_pass_recursion(&root).diameter as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let tree = TreeNode::from_leetcode_slice(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(Solution::diameter_of_binary_tree(tree), 3);
    }

    #[test]
    fn ex2() {
        let tree = TreeNode::from_leetcode_slice(&[Some(1), Some(2)]);
        assert_eq!(Solution::diameter_of_binary_tree(tree), 1);
    }

    #[test]
    fn myex1() {
        let tree = TreeNode::from_leetcode_slice(&[Some(1)]);
        assert_eq!(Solution::diameter_of_binary_tree(tree), 0);
    }

    #[test]
    fn myex2() {
        let tree = TreeNode::from_leetcode_slice(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            None,
            Some(6),
        ]);
        assert_eq!(Solution::diameter_of_binary_tree(tree), 4);
    }

    #[test]
    fn discussion_case1() {
        let tree = TreeNode::from_leetcode_slice(&[
            Some(4),
            Some(-7),
            Some(-3),
            None,
            None,
            Some(-9),
            Some(-3),
            Some(9),
            Some(-7),
            Some(-4),
            None,
            Some(6),
            None,
            Some(-6),
            Some(-6),
            None,
            None,
            Some(0),
            Some(6),
            Some(5),
            None,
            Some(9),
            None,
            None,
            Some(-1),
            Some(-4),
            None,
            None,
            None,
            Some(-2),
        ]);
        assert_eq!(Solution::diameter_of_binary_tree(tree), 8);
    }
}
