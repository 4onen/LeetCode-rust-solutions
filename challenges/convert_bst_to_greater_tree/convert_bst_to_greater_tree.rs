// https://leetcode.com/problems/convert-bst-to-greater-tree/

use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn convert_bst(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn right_first_traverse(node: &mut Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
            if let Some(n) = node.as_mut() {
                let mut n = n.borrow_mut();
                right_first_traverse(&mut n.right, sum);
                *sum += n.val;
                n.val = *sum;
                right_first_traverse(&mut n.left, sum);
            }
        }
        let mut sum = 0;
        right_first_traverse(&mut root, &mut sum);
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &[Option<i32>], output: &[Option<i32>]) {
        assert_eq!(
            Solution::convert_bst(TreeNode::from_leetcode_slice(input)),
            TreeNode::from_leetcode_slice(output)
        );
    }

    #[test]
    fn ex1() {
        test(
            &[
                Some(4),
                Some(1),
                Some(6),
                Some(0),
                Some(2),
                Some(5),
                Some(7),
                None,
                None,
                None,
                Some(3),
                None,
                None,
                None,
                Some(8),
            ],
            &[
                Some(30),
                Some(36),
                Some(21),
                Some(36),
                Some(35),
                Some(26),
                Some(15),
                None,
                None,
                None,
                Some(33),
                None,
                None,
                None,
                Some(8),
            ],
        )
    }

    #[test]
    fn ex2() {
        test(&[Some(0), None, Some(1)], &[Some(1), None, Some(1)])
    }
}
