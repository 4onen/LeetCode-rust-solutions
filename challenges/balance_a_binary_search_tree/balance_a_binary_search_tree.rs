// https://leetcode.com/problems/balance-a-binary-search-tree/

use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

// Failed impl of https://ftp.gnu.org/gnu/avl/avl-2.0.2.pdf.gz
// impl Solution {
//     pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
//         fn rotate_left(
//             root: Rc<RefCell<TreeNode>>,
//             right_subtree: Rc<RefCell<TreeNode>>,
//         ) -> Rc<RefCell<TreeNode>> {
//             root.borrow_mut().right = right_subtree.borrow_mut().left.take();
//             right_subtree.borrow_mut().left = Some(root);
//             right_subtree
//         }
//         fn rotate_right(
//             left_subtree: Rc<RefCell<TreeNode>>,
//             root: Rc<RefCell<TreeNode>>,
//         ) -> Rc<RefCell<TreeNode>> {
//             root.borrow_mut().left = left_subtree.borrow_mut().right.take();
//             left_subtree.borrow_mut().right = Some(root);
//             left_subtree
//         }
//         fn tree_to_left_vine(mut root: Rc<RefCell<TreeNode>>) -> (usize, Rc<RefCell<TreeNode>>) {
//             let mut node_count = 0;
//             let mut curr = root.clone();
//             loop {
//                 let right = curr.borrow_mut().right.take();
//                 if let Some(right) = right {
//                     curr = rotate_left(curr, right);
//                     if node_count == 0 {
//                         root = curr.clone();
//                     }
//                     continue;
//                 }
//                 let left = curr.borrow().left.clone();
//                 if let Some(left) = left {
//                     node_count += 1;
//                     curr = left;
//                     continue;
//                 }
//                 node_count += 1;
//                 break;
//             }
//             (node_count, root)
//         }
//         fn compress(mut root: Rc<RefCell<TreeNode>>, count: usize) -> Rc<RefCell<TreeNode>> {
//             let mut curr = root.clone();
//             let mut i = 0;
//             while i < count {
//                 let Some(red) = curr.borrow_mut().left.take() else {
//                     break;
//                 };
//                 let Some(black) = red.borrow_mut().left.take() else {
//                     rotate_right(red, curr);
//                     break;
//                 };
//                 root.borrow_mut().left = Some(black.clone());
//                 red.borrow_mut().left = black.borrow_mut().right.take();
//                 black.borrow_mut().right = Some(red);
//                 curr = black;
//                 if i == 0 {
//                     root = curr.clone();
//                 }
//                 i += 1;
//             }
//             root
//         }
//         fn compress_left_vine_to_tree(
//             mut root: Rc<RefCell<TreeNode>>,
//             node_count: usize,
//         ) -> Rc<RefCell<TreeNode>> {
//             let leaves = node_count - (node_count.next_power_of_two() - 1) >> 1;
//             root = compress(root, leaves);
//             let mut vine = node_count - leaves;
//             while vine > 1 {
//                 root = compress(root, vine >> 1);
//                 vine >>= 1;
//             }
//             root
//         }
//         let (node_count, root) = tree_to_left_vine(root?);
//         let root = compress_left_vine_to_tree(root, node_count);
//         Some(root)
//     }
// }

impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn inorder_consume(root: Rc<RefCell<TreeNode>>, result: &mut Vec<i32>) {
            let node: TreeNode = match Rc::try_unwrap(root) {
                Ok(node) => node.into_inner(),
                Err(root) => {
                    let borrow = root.borrow();
                    TreeNode {
                        val: borrow.val,
                        left: borrow.left.clone(),
                        right: borrow.right.clone(),
                    }
                },
            };
            if let Some(left) = node.left {
                inorder_consume(left, result);
            }
            result.push(node.val);
            if let Some(right) = node.right {
                inorder_consume(right, result);
            }
        }
        fn build_balanced_tree(
            inorder: &[i32],
            start: usize,
            end: usize,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if start >= end {
                return None;
            }
            let mid = start + (end - start) / 2;
            let mut node = TreeNode::new(inorder[mid]);
            node.left = build_balanced_tree(inorder, start, mid);
            node.right = build_balanced_tree(inorder, mid + 1, end);
            Some(Rc::new(RefCell::new(node)))
        }
        let mut inorder = vec![];
        inorder_consume(root?, &mut inorder);
        build_balanced_tree(&inorder, 0, inorder.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_balanced(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                let left_depth = assert_balanced(&node.left);
                let right_depth = assert_balanced(&node.right);
                assert!(
                    left_depth <= right_depth + 1,
                    "Left subtree is too deep: {} >> {}",
                    left_depth,
                    right_depth,
                );
                assert!(
                    right_depth <= left_depth + 1,
                    "Right subtree is too deep: {} << {}",
                    left_depth,
                    right_depth,
                );
                1 + std::cmp::max(left_depth, right_depth)
            }
        }
    }

    fn inorder_traversal_and_assert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut result = vec![];
        let mut stack = vec![];
        let mut current = root;
        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                current = node.borrow_mut().left.take();
                stack.push(node);
            }
            let node = stack.pop().unwrap();
            let val = node.borrow().val;
            assert!(result.last().map_or(true, |x| x <= &Some(val)), "Not a BST");
            result.push(Some(node.borrow().val));
            current = node.borrow().right.clone();
        }
        result
    }

    fn test(input: &[Option<i32>]) {
        let node_count = input.iter().filter(|x| x.is_some()).count();
        assert!(node_count > 0);
        assert!(node_count <= 10_000);
        for maybe_node in input {
            if let Some(node) = maybe_node {
                assert!(*node >= 1);
                assert!(*node <= 100_000);
            }
        }
        let original_inorder =
            inorder_traversal_and_assert_bst(TreeNode::from_leetcode_slice(input));
        // A BST is balanced if the depth of the two subtrees of every node never differ by more than 1.
        // Two BSTs represent the same data if both are BSTs and the inorder traversal of both are the same.
        let root = TreeNode::from_leetcode_slice(input);
        let result = Solution::balance_bst(root);
        assert_balanced(&result);
        let result_inorder = inorder_traversal_and_assert_bst(result);
        assert_eq!(original_inorder, result_inorder);
    }

    #[test]
    fn ex1() {
        test(&[
            Some(1),
            None,
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            None,
        ])
    }

    #[test]
    fn ex2() {
        test(&[Some(2), Some(1), Some(3)])
    }

    #[test]
    fn discussion_case1() {
        // [1000,500,1500,250,750,1250,1750]
        test(&[
            Some(1000),
            Some(500),
            Some(1500),
            Some(250),
            Some(750),
            Some(1250),
            Some(1750),
        ])
    }

    #[test]
    fn discussion_case2() {
        test(&[Some(1)])
    }

    #[test]
    fn discussion_case3() {
        // [10,5,null,2,null,1]
        test(&[Some(10), Some(5), None, Some(2), None, Some(1)])
    }

    #[test]
    fn discussion_case4() {
        // [4,2,6,1,3,null,7]
        test(&[Some(4), Some(2), Some(6), Some(1), Some(3), None, Some(7)])
    }
}
