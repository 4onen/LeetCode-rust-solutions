// https://leetcode.com/problems/even-odd-tree/description/

use utils::TreeNode;

pub struct Solution;

// Initial sol'n
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let root = root.borrow();
            if root.val % 2 == 0 {
                return false;
            }
            #[derive(Debug, Clone, Copy)]
            struct NodeInfo<'a> {
                node: &'a TreeNode,
                level: u32,
            }
            let mut queue = std::collections::VecDeque::new();
            if let Some(left) = &root.left {
                queue.push_back(NodeInfo {
                    node: unsafe { &*left.as_ptr() },
                    level: 1,
                });
            }
            if let Some(right) = &root.right {
                queue.push_back(NodeInfo {
                    node: unsafe { &*right.as_ptr() },
                    level: 1,
                });
            }
            let mut prev_level = 0u32;
            let mut prev_value = 0u32; // Will be updated on first iteration
            while let Some(NodeInfo { node, level }) = queue.pop_front() {
                let value = node.val as u32;
                let odd_level = level % 2 == 1;
                let odd_value = value % 2 == 1;
                if odd_level == odd_value {
                    return false;
                }
                if level == prev_level {
                    match prev_value.cmp(&value) {
                        std::cmp::Ordering::Less if !odd_level => {}
                        std::cmp::Ordering::Greater if odd_level => {}
                        _ => return false,
                    }
                } else {
                    prev_level = level;
                }
                prev_value = value;
                if let Some(left) = &node.left {
                    queue.push_back(NodeInfo {
                        node: unsafe { &*left.as_ptr() },
                        level: level + 1,
                    });
                }
                if let Some(right) = &node.right {
                    queue.push_back(NodeInfo {
                        node: unsafe { &*right.as_ptr() },
                        level: level + 1,
                    });
                }
            }
            true
        } else {
            unreachable!("Tree should not be empty!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let tree = TreeNode::from_leetcode_slice(&[
            Some(1),
            Some(10),
            Some(4),
            Some(3),
            None,
            Some(7),
            Some(9),
            Some(12),
            Some(8),
            Some(6),
            None,
            None,
            Some(2),
        ]);
        assert_eq!(Solution::is_even_odd_tree(tree), true);
    }

    #[test]
    fn ex2() {
        let tree =
            TreeNode::from_leetcode_slice(&[Some(5), Some(4), Some(2), Some(3), Some(3), Some(7)]);
        assert_eq!(Solution::is_even_odd_tree(tree), false);
    }

    #[test]
    fn ex3() {
        let tree =
            TreeNode::from_leetcode_slice(&[Some(5), Some(9), Some(1), Some(3), Some(5), Some(7)]);
        assert_eq!(Solution::is_even_odd_tree(tree), false);
    }

    #[test]
    fn discussion_case1() {
        // tc 99/105
        let tree = TreeNode::from_leetcode_slice(&[
            Some(2),
            Some(12),
            Some(8),
            Some(5),
            Some(9),
            None,
            None,
            Some(18),
            Some(16),
        ]);
        assert_eq!(Solution::is_even_odd_tree(tree), false);
    }

    #[test]
    fn discussion_case2() {
        let tree = TreeNode::from_leetcode_slice(&[
            Some(11),
            Some(18),
            Some(14),
            Some(3),
            Some(7),
            None,
            None,
            None,
            None,
            Some(18),
            None,
            Some(6),
        ]);
        assert_eq!(Solution::is_even_odd_tree(tree), false);
    }
}
