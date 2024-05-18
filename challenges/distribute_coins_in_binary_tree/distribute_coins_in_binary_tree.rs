// https://leetcode.com/problems/distribute-coins-in-binary-tree/

use utils::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, moves: &mut i16) -> i16 {
            match node.as_deref() {
                Some(node) => {
                    let node = node.borrow();
                    let left = dfs(&node.left, moves);
                    let right = dfs(&node.right, moves);
                    *moves += left.abs() + right.abs();
                    node.val as i16 + left + right - 1
                }
                None => 0,
            }
        }
        let mut moves = 0;
        dfs(&root, &mut moves);
        moves as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input_slice: &[Option<i32>], expected: i32) {
        let root = TreeNode::from_slice(input_slice);
        assert_eq!(Solution::distribute_coins(root), expected);
    }

    #[test]
    fn ex1() {
        test(&[Some(3), Some(0), Some(0)], 2);
    }

    #[test]
    fn ex2() {
        test(&[Some(0), Some(3), Some(0)], 3);
    }
}
