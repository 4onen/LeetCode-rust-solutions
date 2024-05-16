// https://leetcode.com/problems/evaluate-boolean-binary-tree/

use utils::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => unreachable!("Empty tree"),
            Some(root_ref) => {
                let mut root = root_ref.borrow_mut();
                match root.val {
                    0 => false,
                    1 => true,
                    2 | 3 => {
                        let left = Self::evaluate_tree(root.left.take());
                        let right = Self::evaluate_tree(root.right.take());
                        match root.val {
                            2 => left || right,
                            3 => left && right,
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!("Invalid value in tree node"),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(tree_slice: &[Option<i32>], expected: bool) {
        assert!(tree_slice.len() > 0);
        for el in tree_slice {
            assert!(el.is_none() || [Some(0), Some(1), Some(2), Some(3)].contains(el));
        }
        let tree = TreeNode::from_leetcode_slice(tree_slice);
        assert_eq!(Solution::evaluate_tree(tree), expected);
    }

    #[test]
    fn ex1() {
        test(
            &[Some(2), Some(1), Some(3), None, None, Some(0), Some(1)],
            true,
        )
    }

    #[test]
    fn ex2() {
        test(&[Some(0)], false)
    }

    #[test]
    fn failing_case1() {
        test(
            &[
                Some(3),
                Some(3),
                Some(2),
                Some(0),
                Some(0),
                Some(3),
                Some(2),
                None,
                None,
                None,
                None,
                Some(1),
                Some(3),
                Some(1),
                Some(1),
                None,
                None,
                Some(2),
                Some(1),
                None,
                None,
                None,
                None,
                Some(1),
                Some(1),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            false,
        )
    }
}
