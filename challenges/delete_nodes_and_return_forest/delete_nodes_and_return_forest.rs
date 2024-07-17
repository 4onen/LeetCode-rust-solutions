// https://leetcode.com/problems/delete-nodes-and-return-forest/

use utils::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let to_delete = to_delete
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        let mut forest = Vec::with_capacity(to_delete.len()+1);
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            to_delete: &std::collections::HashSet<i32>,
            forest: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            let Some(root) = root else {
                return None;
            };
            let mut node = root.borrow_mut();
            let left = dfs(node.left.take(), to_delete, forest);
            let right = dfs(node.right.take(), to_delete, forest);
            if to_delete.contains(&node.val) {
                if left.is_some() {
                    forest.push(left);
                }
                if right.is_some() {
                    forest.push(right);
                }
                None
            } else {
                node.left = left;
                node.right = right;
                std::mem::drop(node);
                Some(root)
            }
        }
        let root = dfs(root, &to_delete, &mut forest);
        if root.is_some() {
            forest.push(root);
        }
        forest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &[Option<i32>], to_delete: &[i32], expected: &[&[Option<i32>]]) {
        assert!(input.iter().filter(|&i| Option::is_some(i)).count() <= 1000);
        assert!(to_delete.len() <= 1000);
        let root = TreeNode::from_leetcode_slice(input);
        let to_delete = to_delete.to_vec();
        let mut output = Solution::del_nodes(root, to_delete);
        let mut expected_forest = expected
            .iter()
            .map(|slice| TreeNode::from_leetcode_slice(slice))
            .collect::<Vec<_>>();
        output.sort_by_key(|node| node.as_ref().map(|node| node.borrow().val));
        expected_forest.sort_by_key(|node| node.as_ref().map(|node| node.borrow().val));
        assert_eq!(output, expected_forest);
    }

    #[test]
    fn ex1() {
        test(
            &[
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
            ],
            &[3, 5],
            &[&[Some(1), Some(2), None, Some(4)], &[Some(6)], &[Some(7)]],
        )
    }

    #[test]
    fn ex2() {
        test(
            &[Some(1), Some(2), Some(4), None, Some(3)],
            &[3],
            &[&[Some(1), Some(2), Some(4)]],
        )
    }
}
