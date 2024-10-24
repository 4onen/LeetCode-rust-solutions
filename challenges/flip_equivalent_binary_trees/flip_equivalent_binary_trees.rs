// https://leetcode.com/problems/flip-equivalent-binary-trees/

use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn flip_equiv_help(
            root1: &Option<Rc<RefCell<TreeNode>>>,
            root2: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (root1, root2) {
                (Some(node1), Some(node2)) => {
                    let node1 = node1.borrow();
                    let node2 = node2.borrow();
                    node1.val == node2.val
                        && ((flip_equiv_help(&node1.left, &node2.left)
                            && flip_equiv_help(&node1.right, &node2.right))
                            || (flip_equiv_help(&node1.left, &node2.right)
                                && flip_equiv_help(&node1.right, &node2.left)))
                }
                (None, None) => true,
                _ => false,
            }
        }
        return flip_equiv_help(&root1, &root2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input1: &[Option<i32>], input2: &[Option<i32>], expected: bool) {
        let tree1 = TreeNode::from_leetcode_slice(input1);
        let tree2 = TreeNode::from_leetcode_slice(input2);
        assert_eq!(Solution::flip_equiv(tree1, tree2), expected);
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
                None,
                None,
                None,
                Some(7),
                Some(8),
            ],
            &[
                Some(1),
                Some(3),
                Some(2),
                None,
                Some(6),
                Some(4),
                Some(5),
                None,
                None,
                None,
                None,
                Some(8),
                Some(7),
            ],
            true,
        );
    }

    #[test]
    fn ex1_1() {
        test(
            &[
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                None,
                None,
                None,
            ],
            &[
                Some(1),
                Some(3),
                Some(2),
                None,
                Some(6),
                Some(4),
                Some(5),
                None,
                None,
                None,
            ],
            true,
        );
    }

    #[test]
    fn ex1_2() {
        test(
            &[
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                None,
                None,
                None,
                Some(7),
                Some(8),
            ],
            &[
                Some(1),
                Some(3),
                Some(2),
                None,
                Some(6),
                Some(4),
                Some(5),
                None,
                None,
                None,
                Some(8),
                Some(7),
            ],
            false,
        );
    }

    #[test]
    fn ex2() {
        test(&[], &[], true);
    }

    #[test]
    fn ex3() {
        test(&[], &[Some(1)], false);
    }
}
