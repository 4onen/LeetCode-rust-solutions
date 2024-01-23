// https://leetcode.com/problems/maximum-depth-of-binary-tree/

use crate::utils::TreeNode;

pub struct Solution;

// Recursive sol'n
// use std::cell::RefCell;
// use std::rc::Rc;
// impl Solution {
//     pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         match root {
//             Some(node) => {
//                 let node = node.borrow();
//                 let left = Self::max_depth(node.left.clone());
//                 let right = Self::max_depth(node.right.clone());
//                 1 + left.max(right)
//             }
//             None => 0,
//         }
//     }
// }

// Stack sol'n
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let mut stack = vec![(node, 1)];
                let mut max_depth = 0;
                while let Some((node, depth)) = stack.pop() {
                    let mut node = node.borrow_mut();
                    max_depth = max_depth.max(depth);
                    if let Some(left) = node.left.take() {
                        stack.push((left, depth + 1));
                    }
                    if let Some(right) = node.right.take() {
                        stack.push((right, depth + 1));
                    }
                }
                max_depth
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let root =
            TreeNode::from_slice(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(Solution::max_depth(root), 3);
    }

    #[test]
    fn ex2() {
        let root = TreeNode::from_slice(&[Some(1), None, Some(2)]);
        assert_eq!(Solution::max_depth(root), 2);
    }
}
