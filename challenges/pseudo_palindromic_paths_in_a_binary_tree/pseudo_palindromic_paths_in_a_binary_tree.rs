// https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree/

use utils::TreeNode;

pub struct Solution;

// Recursive sol'n
// use std::cell::RefCell;
// use std::rc::Rc;
// impl Solution {
//     pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         #[derive(Debug, Clone, Copy)]
//         #[repr(transparent)]
//         struct Alphabet {
//             odds: u16,
//         }
//         impl Alphabet {
//             fn new() -> Self {
//                 Self { odds: 0 }
//             }
//             fn add(&self, i: u8) -> Self {
//                 Alphabet {
//                     odds: self.odds ^ (1 << i),
//                 }
//             }
//             fn is_palindrome(&self) -> bool {
//                 self.odds.count_ones() <= 1
//             }
//         }
//         fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, alphabet: Alphabet) -> u32 {
//             if let Some(node) = node {
//                 let node = node.borrow();
//                 let alphabet = alphabet.add(node.val as u8);
//                 if node.left.is_none() && node.right.is_none() {
//                     return alphabet.is_palindrome() as u32;
//                 }
//                 dfs(&node.left, alphabet) + dfs(&node.right, alphabet)
//             } else {
//                 0
//             }
//         }
//         dfs(&root, Alphabet::new()) as i32
//     }
// }

// Stack sol'n
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0;
        if let Some(root) = root {
            let mut stack = vec![(root, 0u16)];
            while let Some((node, mut alphabet)) = stack.pop() {
                let mut node = node.borrow_mut();
                alphabet ^= 1 << node.val;
                if node.left.is_none() && node.right.is_none() {
                    if alphabet.count_ones() <= 1 {
                        count += 1;
                    }
                } else {
                    if let Some(left) = node.left.take() {
                        stack.push((left, alphabet));
                    }
                    if let Some(right) = node.right.take() {
                        stack.push((right, alphabet));
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::pseudo_palindromic_paths(TreeNode::from_leetcode_slice(&[
                Some(2),
                Some(3),
                Some(1),
                Some(3),
                Some(1),
                None,
                Some(1)
            ])),
            2
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::pseudo_palindromic_paths(TreeNode::from_leetcode_slice(&[
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                Some(3),
                None,
                None,
                None,
                None,
                None,
                Some(1)
            ])),
            1
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::pseudo_palindromic_paths(TreeNode::from_leetcode_slice(&[Some(9),])),
            1
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(
            Solution::pseudo_palindromic_paths(TreeNode::from_leetcode_slice(&[Some(9), Some(1)])),
            0
        );
    }
}
