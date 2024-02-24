// https://leetcode.com/problems/same-tree/

use utils::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();
                p.val == q.val
                    && Solution::is_same_tree(p.left.clone(), q.left.clone())
                    && Solution::is_same_tree(p.right.clone(), q.right.clone())
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let p = TreeNode::from_slice(&[Some(1), Some(2), Some(3)]);
        let q = TreeNode::from_slice(&[Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::is_same_tree(p, q), true);
    }

    #[test]
    fn ex2() {
        let p = TreeNode::from_slice(&[Some(1), Some(2)]);
        let q = TreeNode::from_slice(&[Some(1), None, Some(2)]);
        assert_eq!(Solution::is_same_tree(p, q), false);
    }

    #[test]
    fn ex3() {
        let p = TreeNode::from_slice(&[Some(1), Some(2), Some(1)]);
        let q = TreeNode::from_slice(&[Some(1), Some(1), Some(2)]);
        assert_eq!(Solution::is_same_tree(p, q), false);
    }
}
