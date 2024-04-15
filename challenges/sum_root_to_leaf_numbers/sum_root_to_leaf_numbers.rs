// https://leetcode.com/problems/sum-root-to-leaf-numbers/

use utils::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         let Some(root) = root else {
//             return 0;
//         };
//         let mut stack = vec![(root, 0)];
//         let mut res = 0;
//         while let Some((node, val)) = stack.pop() {
//             let node = node.borrow();
//             let val = val * 10 + node.val;
//             if node.left.is_none() && node.right.is_none() {
//                 res += val;
//             } else {
//                 if let Some(left) = &node.left {
//                     stack.push((left.clone(), val));
//                 }
//                 if let Some(right) = &node.right {
//                     stack.push((right.clone(), val));
//                 }
//             }
//         }
//         res
//     }
// }

// Stack alloc sol'n
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };
        // Safety: The depth of the tree is guaranteed to be at most 10.
        // Thus, the maximum number of elements in the DFS stack is 2*10+1 = 21
        // (Which is every node in one path from root to leaf, plus one
        // alternate choice at each node in the path.)
        const MAX_DEPTH: usize = 21;
        let mut stack_data: [std::mem::MaybeUninit<(Rc<RefCell<TreeNode>>, i32)>; MAX_DEPTH] =
            unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        stack_data[0].write((root, 0));
        let mut stack_len = 1;
        let mut res = 0;
        while stack_len > 0 {
            stack_len -= 1;
            let (node, val) = unsafe { stack_data[stack_len].assume_init_read() };
            let node = node.borrow();
            let val = val * 10 + node.val;
            if node.left.is_none() && node.right.is_none() {
                res += val;
            } else {
                if let Some(left) = &node.left {
                    stack_data[stack_len].write((left.clone(), val));
                    stack_len += 1;
                }
                if let Some(right) = &node.right {
                    stack_data[stack_len].write((right.clone(), val));
                    stack_len += 1;
                }
                assert!(stack_len < MAX_DEPTH, "Stack overflow")
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(slice: &[Option<i32>], expected: i32) {
        let root = TreeNode::from_leetcode_slice(slice);
        assert_eq!(Solution::sum_numbers(root), expected);
    }

    #[test]
    fn ex1() {
        test(&[Some(1), Some(2), Some(3)], 25)
    }

    #[test]
    fn ex2() {
        test(&[Some(4), Some(9), Some(0), Some(5), Some(1)], 1026)
    }

    #[test]
    fn discussion_case1() {
        test(&[Some(1), Some(0)], 10)
    }

    #[test]
    fn discussion_case2() {
        test(&[Some(0), Some(1)], 1)
    }

    #[test]
    fn discussion_case3() {
        test(&[Some(0), Some(2), Some(1), Some(3)], 24)
    }

    #[test]
    fn myex1() {
        test(&[Some(1)], 1)
    }

    #[test]
    fn my_extreme_example1() {
        test(
            &[0, 0, 0, 1]
                .repeat(250)
                .into_iter()
                .map(|n| Some(n))
                .collect::<Vec<Option<i32>>>(),
            1422221775,
        )
    }
}
