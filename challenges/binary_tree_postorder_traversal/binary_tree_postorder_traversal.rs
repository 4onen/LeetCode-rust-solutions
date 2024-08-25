// https://leetcode.com/problems/binary-tree-postorder-traversal/

use utils::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn recurse(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(node) = node {
                let node = node.borrow();
                recurse(&node.left, result);
                recurse(&node.right, result);
                result.push(node.val);
            }
        }
        let mut result = Vec::new();
        recurse(&root, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(slice: &[Option<i32>], expected: &[i32]) {
        let mut non_none = 0;
        for &n in slice {
            if let Some(n) = n {
                assert!(n >= -100);
                assert!(n <= 100);
                non_none += 1;
            }
        }
        assert!(non_none <= 100);
        let root = TreeNode::from_leetcode_slice(slice);
        assert_eq!(Solution::postorder_traversal(root), expected);
    }

    #[test]
    fn ex1() {
        test(&[Some(1), None, Some(2), Some(3)], &[3, 2, 1]);
    }

    #[test]
    fn ex2() {
        test(&[], &[]);
    }

    #[test]
    fn ex3() {
        test(&[Some(1)], &[1]);
    }
}
