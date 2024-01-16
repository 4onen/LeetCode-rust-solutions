// https://leetcode.com/problems/range-sum-of-bst/

use super::utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

// Braindead recursion
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut sum = 0;
        if let Some(node) = root {
            let node = node.borrow();
            let val = node.val;
            if val >= low && node.val <= high {
                sum += val;
            }
            if val > low {
                sum += Self::range_sum_bst(node.left.clone(), low, high);
            }
            if val < high {
                sum += Self::range_sum_bst(node.right.clone(), low, high);
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::range_sum_bst(
                TreeNode::from_leetcode_slice(&[
                    Some(10),
                    Some(5),
                    Some(15),
                    Some(3),
                    Some(7),
                    None,
                    Some(18)
                ]),
                7,
                15
            ),
            32
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::range_sum_bst(
                TreeNode::from_leetcode_slice(&[
                    Some(10),
                    Some(5),
                    Some(15),
                    Some(3),
                    Some(7),
                    Some(13),
                    Some(18),
                    Some(1),
                    None,
                    Some(6)
                ]),
                6,
                10
            ),
            23
        );
    }
}
