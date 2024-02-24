// https://leetcode.com/problems/binary-tree-level-order-traversal/

use utils::TreeNode;
pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if let Some(root) = root {
            let mut result = Vec::new();
            let mut queue = vec![root];

            while !queue.is_empty() {
                let mut next_queue = Vec::new();

                result.push(
                    queue
                        .into_iter()
                        .map(|node| match Rc::try_unwrap(node).map(RefCell::into_inner) {
                            Ok(node) => {
                                if let Some(left) = node.left {
                                    next_queue.push(left);
                                }
                                if let Some(right) = node.right {
                                    next_queue.push(right);
                                }
                                node.val
                            }
                            Err(node) => {
                                let node = node.borrow();
                                if let Some(left) = node.left.as_ref() {
                                    next_queue.push(left.clone());
                                }
                                if let Some(right) = node.right.as_ref() {
                                    next_queue.push(right.clone());
                                }
                                node.val
                            }
                        })
                        .collect::<Vec<_>>(),
                );
                queue = next_queue;
            }
            result
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::level_order(TreeNode::from_slice(&[
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
    }
}
