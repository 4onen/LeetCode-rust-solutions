// https://leetcode.com/problems/add-one-row-to-tree/

use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn add_one_row(
//         mut root: Option<Rc<RefCell<TreeNode>>>,
//         val: i32,
//         depth: i32,
//     ) -> Option<Rc<RefCell<TreeNode>>> {
//         if depth == 1 {
//             Some(Rc::new(RefCell::new(TreeNode {
//                 val,
//                 left: root,
//                 right: None,
//             })))
//         } else {
//             match root.as_mut() {
//                 None => {}
//                 Some(node) => {
//                     let mut node_inner = node.borrow_mut();
//                     if depth == 2 {
//                         let left = node_inner.left.take();
//                         let right = node_inner.right.take();
//                         node_inner.left = Some(Rc::new(RefCell::new(TreeNode {
//                             val,
//                             left,
//                             right: None,
//                         })));
//                         node_inner.right = Some(Rc::new(RefCell::new(TreeNode {
//                             val,
//                             left: None,
//                             right,
//                         })));
//                     } else {
//                         node_inner.left = Self::add_one_row(node_inner.left.take(), val, depth - 1);
//                         node_inner.right =
//                             Self::add_one_row(node_inner.right.take(), val, depth - 1);
//                     }
//                 }
//             }
//             root
//         }
//     }
// }

// Improved sol'n
impl Solution {
    pub fn add_one_row(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: root,
                right: None,
            })))
        } else {
            Self::add_row_below_root(&mut root, val, depth - 2);
            root
        }
    }
    pub fn add_row_below_root(root: &mut Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) {
        if let Some(inner) = root {
            if depth > 0 {
                let mut node = inner.borrow_mut();
                Self::add_row_below_root(&mut node.left, val, depth - 1);
                Self::add_row_below_root(&mut node.right, val, depth - 1);
            } else {
                let mut node = inner.borrow_mut();
                node.left = Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: node.left.take(),
                    right: None,
                })));
                node.right = Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: None,
                    right: node.right.take(),
                })));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input_slice: &[Option<i32>], val: i32, depth: i32, expected_slice: &[Option<i32>]) {
        let root = TreeNode::from_leetcode_slice(input_slice);
        let expected = TreeNode::from_leetcode_slice(expected_slice);
        assert_eq!(Solution::add_one_row(root, val, depth), expected);
    }

    #[test]
    fn ex1() {
        test(
            &[Some(4), Some(2), Some(6), Some(3), Some(1), Some(5)],
            1,
            2,
            &[
                Some(4),
                Some(1),
                Some(1),
                Some(2),
                None,
                None,
                Some(6),
                Some(3),
                Some(1),
                Some(5),
            ],
        );
    }

    #[test]
    fn ex2() {
        test(
            &[Some(4), Some(2), None, Some(3), Some(1)],
            1,
            3,
            &[
                Some(4),
                Some(2),
                None,
                Some(1),
                Some(1),
                Some(3),
                None,
                None,
                Some(1),
            ],
        );
    }

    #[test]
    fn discussion_case1() {
        test(
            &[Some(4), Some(2), Some(6), Some(3), Some(1), Some(5)],
            1,
            1,
            &[
                Some(1),
                Some(4),
                None,
                Some(2),
                Some(6),
                Some(3),
                Some(1),
                Some(5),
            ],
        );
    }
}
