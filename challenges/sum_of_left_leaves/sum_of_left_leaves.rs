// https://leetcode.com/problems/sum-of-left-leaves/

use utils::TreeNode;

pub struct Solution;

// Initial sol'n
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        let mut stack = vec![(root, false)];
        while !stack.is_empty() {
            if let (Some(node), is_left) = stack.pop().unwrap() {
                let mut node = Rc::try_unwrap(node).unwrap().into_inner();
                if node.left.is_none() && node.right.is_none() && is_left {
                    sum += node.val;
                }
                stack.push((node.right.take(), false));
                stack.push((node.left.take(), true));
            }
        }
        sum
    }
}

// Non-mutating sol'n (bar reference counting)
// use std::cell::RefCell;
// use std::rc::Rc;
// impl Solution {
//     pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         let mut sum = 0;
//         let mut stack = vec![(root, false)];
//         while !stack.is_empty() {
//             if let (Some(node), is_left) = stack.pop().unwrap() {
//                 let node = node.borrow();
//                 if node.left.is_none() && node.right.is_none() && is_left {
//                     sum += node.val;
//                 }
//                 stack.push((node.right.clone(), false));
//                 stack.push((node.left.clone(), true));
//             }
//         }
//         sum
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(tree: &[Option<i32>], expected: i32) {
        let root = TreeNode::from_leetcode_slice(tree);
        assert_eq!(Solution::sum_of_left_leaves(root), expected);
    }

    #[test]
    fn ex1() {
        test(
            &[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
            24,
        );
    }

    #[test]
    fn ex2() {
        test(&[Some(1)], 0);
    }

    #[test]
    fn myex1() {
        test(&[Some(1), Some(2), Some(3)], 2);
    }
}
