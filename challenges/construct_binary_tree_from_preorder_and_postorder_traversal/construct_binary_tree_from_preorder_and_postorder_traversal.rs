// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/

use utils::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        assert!(preorder.len() <= 30);
        assert_eq!(preorder.len(), postorder.len());
        // We know the root is the first element in preorder and the last element in postorder.
        // We also know that the right subtree must root at the second to last element in postorder,
        // and the left subtree must root at the second element in preorder.
        // Let's make this a recursive solution working our way down on slices of the given arrays.
        fn build_tree(preorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() {
                return None;
            }
            match preorder.len() {
                1 => Some(Rc::new(RefCell::new(TreeNode {
                    val: preorder[0],
                    left: None,
                    right: None,
                }))),
                2 => {
                    // It's impossible to tell whether the spare node is the left or right child,
                    // so we'll just assume it's the left child.
                    Some(Rc::new(RefCell::new(TreeNode {
                        val: preorder[0],
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: preorder[1],
                            left: None,
                            right: None,
                        }))),
                        right: None,
                    })))
                }
                _ => {
                    // We can capture both subtree roots from the preorder and postorder arrays,
                    // so recurse the problem.
                    let root = preorder[0];
                    let right_root = postorder[postorder.len() - 2];
                    // The left subtree ends where the right subtree begins.
                    let left_preorder_end = preorder.iter().position(|&x| x == right_root).unwrap();
                    // Any elements of the preorder not the original root and not in the
                    // left subtree must be in the right subtree.
                    let left_preorder = &preorder[1..left_preorder_end];
                    let right_preorder = &preorder[left_preorder_end..];
                    // The length of each subtree in the postorder must be the same,
                    // and they must come in the same order.
                    // (Remember the first element of preorder was the root, which is moved in postorder.)
                    let left_postorder = &postorder[..left_preorder_end - 1];
                    let right_postorder = &postorder[left_preorder_end - 1..postorder.len() - 1];
                    let left_subtree = build_tree(left_preorder, left_postorder);
                    let right_subtree = build_tree(right_preorder, right_postorder);
                    Some(Rc::new(RefCell::new(TreeNode {
                        val: root,
                        left: left_subtree,
                        right: right_subtree,
                    })))
                }
            }
        }
        build_tree(&preorder, &postorder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn preorder_traversal(root: &Option<Rc<RefCell<TreeNode>>>, size_hint: u8) -> Vec<i32> {
        let mut result = std::vec::Vec::with_capacity(size_hint as usize);
        let mut stack = std::vec::Vec::with_capacity(2 * size_hint as usize);
        stack.push(root.clone());
        while let Some(node) = stack.pop() {
            match node {
                None => continue,
                Some(node) => {
                    result.push(node.borrow().val);
                    stack.push(node.borrow().right.clone());
                    stack.push(node.borrow().left.clone());
                }
            }
        }
        result
    }

    fn postorder_traversal(root: &Option<Rc<RefCell<TreeNode>>>, size_hint: u8) -> Vec<i32> {
        let mut result = std::vec::Vec::with_capacity(size_hint as usize);
        let mut stack = std::vec::Vec::with_capacity(2 * size_hint as usize);
        let mut visited = std::vec::Vec::with_capacity(size_hint as usize);
        stack.push(root.clone());
        while let Some(node) = stack.last() {
            match node.clone() {
                None => continue,
                Some(node_in) => {
                    if node_in
                        .borrow()
                        .left
                        .as_deref()
                        .map(|n| !visited.contains(&n.borrow().val))
                        .unwrap_or(false)
                    {
                        stack.push(node_in.borrow().left.clone());
                    } else if node_in
                        .borrow()
                        .right
                        .as_deref()
                        .map(|n| !visited.contains(&n.borrow().val))
                        .unwrap_or(false)
                    {
                        stack.push(node_in.borrow().right.clone());
                    } else {
                        result.push(node_in.borrow().val);
                        visited.push(node_in.borrow().val);
                        stack.pop();
                    }
                }
            }
        }
        result
    }

    fn test(preorder: &[i32], postorder: &[i32]) {
        assert_eq!(preorder.len(), postorder.len());
        assert!(preorder.len() >= 1);
        assert!(preorder.len() <= 30);
        assert_eq!(preorder.first(), postorder.last());
        let mut seen = std::collections::HashSet::new();
        for &node in preorder {
            assert!(node >= 1);
            assert!(node <= preorder.len() as i32);
            assert!(seen.insert(node));
        }
        seen.clear();
        for &node in postorder {
            assert!(node >= 1);
            assert!(node <= postorder.len() as i32);
            assert!(seen.insert(node));
        }
        // If our tree has the same preorder and postorder as those given, it must be correct.
        // (Saves me having to implement all possible trees when multiple are possible.)
        let result = Solution::construct_from_pre_post(preorder.to_owned(), postorder.to_owned());
        let result_preorder = preorder_traversal(&result, preorder.len() as u8);
        let result_postorder = postorder_traversal(&result, postorder.len() as u8);
        assert_eq!(result_preorder, preorder);
        assert_eq!(result_postorder, postorder);
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 4, 5, 3, 6, 7], &[4, 5, 2, 6, 7, 3, 1])
    }

    #[test]
    fn ex2() {
        test(&[1], &[1])
    }

    #[test]
    fn discussion_case1() {
        test(&[1, 2, 3, 4, 5], &[3, 5, 4, 2, 1])
    }

    #[test]
    fn discussion_case2() {
        test(&[1, 2, 3], &[3, 2, 1])
    }
}
