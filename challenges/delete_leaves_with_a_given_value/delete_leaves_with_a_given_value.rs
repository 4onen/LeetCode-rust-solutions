// https://leetcode.com/problems/delete-leaves-with-a-given-value/

use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn remove_leaf_nodes(
//         root: Option<Rc<RefCell<TreeNode>>>,
//         target: i32,
//     ) -> Option<Rc<RefCell<TreeNode>>> {
//         let Some(node) = root else {
//             return None;
//         };
//         let mut node_ref = node.borrow_mut();
//         node_ref.left = Self::remove_leaf_nodes(node_ref.left.take(), target);
//         node_ref.right = Self::remove_leaf_nodes(node_ref.right.take(), target);
//         if node_ref.left.is_none() && node_ref.right.is_none() && node_ref.val == target {
//             return None;
//         }
//         std::mem::drop(node_ref);
//         Some(node)
//     }
// }

// Reference dfs (same speed less of leetcode's memory usage)
impl Solution {
    pub fn remove_leaf_nodes(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: &mut Option<Rc<RefCell<TreeNode>>>, target: i32) -> bool {
            let Some(node) = root.as_mut() else {
                return true;
            };
            let mut node_ref = node.borrow_mut();
            let left = dfs(&mut node_ref.left, target);
            let right = dfs(&mut node_ref.right, target);
            if left && right && node_ref.val == target {
                std::mem::drop(node_ref);
                *root = None;
                return true;
            }
            false
        }
        dfs(&mut root, target);
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(tree_slice: &[Option<i32>], target: i32, expected_slice: &[Option<i32>]) {
        let root = TreeNode::from_leetcode_slice(tree_slice);
        let expected = TreeNode::from_leetcode_slice(expected_slice);
        assert_eq!(Solution::remove_leaf_nodes(root, target), expected);
    }

    #[test]
    fn ex1() {
        // Input: root = [1,2,3,2,null,2,4], target = 2
        // Output: [1,null,3,null,4]
        test(
            &[Some(1), Some(2), Some(3), Some(2), None, Some(2), Some(4)],
            2,
            &[Some(1), None, Some(3), None, Some(4)],
        );
    }

    #[test]
    fn ex2() {
        // Input: root = [1,3,3,3,2], target = 3
        // Output: [1,3,null,null,2]
        test(
            &[Some(1), Some(3), Some(3), Some(3), Some(2)],
            3,
            &[Some(1), Some(3), None, None, Some(2)],
        );
    }

    #[test]
    fn ex3() {
        // Input: root = [1,2,null,2,null,2], target = 2
        // Output: [1]
        test(
            &[Some(1), Some(2), None, Some(2), None, Some(2)],
            2,
            &[Some(1)],
        );
    }
}
