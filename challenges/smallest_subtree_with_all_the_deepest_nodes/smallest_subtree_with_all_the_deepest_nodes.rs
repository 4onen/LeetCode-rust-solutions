// https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/

use utils::TreeNode;

pub struct Solution;
use std::cell::RefCell;
use std::rc::Rc;

// Copied from solution for
// https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/
impl Solution {
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // Find the LCA's value, we'll pull it out later.
        // Returns (value, depth) -- Value of 0 indicates not found yet
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, usize) {
            match node {
                None => (0, 0),
                Some(node) => {
                    let left = dfs(&node.borrow().left);
                    let right = dfs(&node.borrow().right);
                    if left.1 == right.1 {
                        (node.borrow().val, left.1 + 1)
                    } else if left.1 > right.1 {
                        (left.0, left.1 + 1)
                    } else {
                        (right.0, right.1 + 1)
                    }
                }
            }
        }
        let (value, _depth) = dfs(&root);
        // Now tear the tree apart until we find the value
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if node.is_none() {
                continue;
            }
            let node = node.unwrap();
            if node.borrow().val == value {
                return Some(node);
            }
            stack.push(node.borrow_mut().left.take());
            stack.push(node.borrow_mut().right.take());
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(tree: &[Option<i32>], expected: &[Option<i32>]) {
        let input = TreeNode::from_leetcode_slice(tree);
        let result = Solution::subtree_with_all_deepest(input);
        let expected_tree = TreeNode::from_leetcode_slice(expected);
        assert_eq!(result, expected_tree);
    }

    #[test]
    fn ex1() {
        test(
            // [3,5,1,6,2,0,8,null,null,7,4]
            &[
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(2),
                Some(0),
                Some(8),
                None,
                None,
                Some(7),
                Some(4),
            ],
            // [2,7,4]
            &[Some(2), Some(7), Some(4)],
        )
    }

    #[test]
    fn ex2() {
        test(&[Some(1)], &[Some(1)])
    }

    #[test]
    fn ex3() {
        test(
            // [0,1,3,null,2]
            &[Some(0), Some(1), Some(3), None, Some(2)],
            // [2]
            &[Some(2)],
        )
    }

    #[test]
    fn my_extreme_ex1() {
        // Tree that's just all left children for all 500 nodes.
        let mut input = vec![Some(1)];
        input.extend((2..=500).map(|i| [Some(i), None]).flatten());
        let expected = [Some(500)];
        test(&input, &expected);
    }

    #[test]
    fn my_extreme_ex2() {
        // Tree that's just all right children for all 500 nodes.
        let mut input = vec![Some(1)];
        input.extend((1..=500).map(|i| [None, Some(i)]).flatten());
        let expected = [Some(500)];
        test(&input, &expected);
    }

    #[test]
    fn my_extreme_ex3() {
        // 124 left children
        // 124 right children
        let mut input = vec![Some(1), Some(2), Some(3)];
        input.extend(
            (4..=500)
                .step_by(2)
                .map(|i| [Some(i), None, None, Some(i + 1)])
                .flatten(),
        );
        test(&input, &input);
    }

    #[test]
    fn discussion_case1() {
        test(
            // [1,2,null,3,4,null,6,null,5]
            &[
                Some(1),
                Some(2),
                None,
                Some(3),
                Some(4),
                None,
                Some(6),
                None,
                Some(5),
            ],
            // [2,3,4,null,6,null,5]
            &[Some(2), Some(3), Some(4), None, Some(6), None, Some(5)],
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            // [1, 2, 3, 4, 5, null, 6, null, null, 7]
            &[
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                None,
                Some(6),
                None,
                None,
                Some(7),
            ],
            // [7]
            &[Some(7)],
        )
    }
}
