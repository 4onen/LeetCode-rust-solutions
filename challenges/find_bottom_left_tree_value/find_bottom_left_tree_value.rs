// https://leetcode.com/problems/find-bottom-left-tree-value/

use utils::TreeNode;

pub struct Solution;

// Initial sol'n
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::num::NonZeroU16;
        const ONE: NonZeroU16 = unsafe { NonZeroU16::new_unchecked(1) };
        struct DFSResult {
            value: i32,
            depth: NonZeroU16,
        }
        impl DFSResult {
            fn step_depth(self) -> Self {
                Self {
                    value: self.value,
                    depth: self.depth.checked_add(1).unwrap(),
                }
            }
        }
        fn dfs(node: &TreeNode) -> DFSResult {
            match (&node.left, &node.right) {
                (None, None) => DFSResult {
                    value: node.val,
                    depth: ONE,
                },
                (Some(n), None) | (None, Some(n)) => dfs(&n.borrow()).step_depth(),
                (Some(left), Some(right)) => {
                    let left = dfs(&left.borrow());
                    let right = dfs(&right.borrow());
                    if right.depth > left.depth {
                        right
                    } else {
                        left
                    }
                    .step_depth()
                }
            }
        }
        dfs(&root.unwrap().borrow()).value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let tree = TreeNode::from_leetcode_slice(&[Some(2), Some(1), Some(3)]);
        assert_eq!(Solution::find_bottom_left_value(tree), 1);
    }

    #[test]
    fn ex2() {
        let tree = TreeNode::from_leetcode_slice(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            Some(5),
            Some(6),
            None,
            None,
            Some(7),
        ]);
        assert_eq!(Solution::find_bottom_left_value(tree), 7);
    }

    #[test]
    fn discussion_case1() {
        let tree = TreeNode::from_leetcode_slice(&[Some(0), None, Some(-1)]);
        assert_eq!(Solution::find_bottom_left_value(tree), -1);
    }

    #[test]
    fn discussion_case2() {
        let tree = TreeNode::from_leetcode_slice(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            Some(5),
            Some(6),
            None,
            None,
            None,
            None,
            Some(7),
            Some(9),
            Some(8),
            None,
            None,
            Some(10),
            None,
            None,
            None,
            Some(11),
            None,
            Some(12),
        ]);
        assert_eq!(Solution::find_bottom_left_value(tree), 12);
    }

    #[test]
    fn discussion_case3() {
        // tc 75/77
        let tree = TreeNode::from_leetcode_slice(&[
            Some(50),
            Some(25),
            Some(75),
            Some(2),
            None,
            Some(55),
            None,
            None,
            Some(5),
            None,
            Some(59),
            Some(4),
            Some(6),
            Some(58),
            None,
            None,
            None,
            None,
            Some(7),
            Some(57),
            None,
        ]);
        assert_eq!(Solution::find_bottom_left_value(tree), 7);
    }
}
