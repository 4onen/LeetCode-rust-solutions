// https://leetcode.com/problems/two-sum-iv-input-is-a-bst/

use std::rc::Rc;
use std::cell::RefCell;
use utils::TreeNode;

pub struct Solution;

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        fn inorder(root: Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
            if let Some(node) = root {
                match Rc::<RefCell<TreeNode>>::try_unwrap(node) {
                    Ok(node) => {
                        let node = node.into_inner();
                        inorder(node.left, nums);
                        nums.push(node.val);
                        inorder(node.right, nums);
                    }
                    Err(node) => {
                        let borrow = node.borrow();
                        inorder(borrow.left.clone(), nums);
                        nums.push(node.borrow().val);
                        inorder(borrow.right.clone(), nums);
                    }
                }
            }
        }
        let mut nums = Vec::new();
        inorder(root, &mut nums);
        let mut i = 0;
        let mut j = nums.len() as i32 - 1;
        while i < j {
            let sum = nums[i as usize] + nums[j as usize];
            if sum == k {
                return true;
            } else if sum < k {
                i += 1;
            } else {
                j -= 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &[Option<i32>], k: i32, expected: bool) {
        let root = TreeNode::from_leetcode_slice(input);
        assert_eq!(Solution::find_target(root, k), expected);
    }

    #[test]
    fn ex1() {
        test(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)], 9, true);
    }

    #[test]
    fn ex2() {
        test(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)], 28, false);
    }
}
