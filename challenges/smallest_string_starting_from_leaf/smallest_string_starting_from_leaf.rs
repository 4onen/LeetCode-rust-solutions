// https://leetcode.com/problems/smallest-string-starting-from-leaf/
use std::cell::RefCell;
use std::rc::Rc;

use utils::TreeNode;

pub struct Solution;

// GitHub Copilot sol'n
// impl Solution {
//     pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
//         fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<char>, result: &mut String) {
//             if let Some(node) = node {
//                 let node = node.borrow();
//                 path.push((node.val as u8 + b'a') as char);
//                 if node.left.is_none() && node.right.is_none() {
//                     let path = path.iter().rev().collect::<String>();
//                     if result.is_empty() || path < *result {
//                         *result = path;
//                     }
//                 } else {
//                     dfs(&node.left, path, result);
//                     dfs(&node.right, path, result);
//                 }
//                 path.pop();
//             }
//         }
//         let mut result = String::new();
//         dfs(&root, &mut Vec::new(), &mut result);
//         result
//     }
// }

// My initial sol'n
// (Doesn't work because abz and ababz will go left instead of right,
//  because ab < aba, but abz > abaz)
// impl Solution {
//     pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
//         fn compare_vec_lexographic(left: &Vec<u8>, right: &Vec<u8>) -> std::cmp::Ordering {
//             let mut left_iter = left.iter();
//             let mut right_iter = right.iter();
//             loop {
//                 match (left_iter.next(), right_iter.next()) {
//                     (Some(l), Some(r)) => match u8::cmp(l, r) {
//                         std::cmp::Ordering::Less => break std::cmp::Ordering::Less,
//                         std::cmp::Ordering::Greater => break std::cmp::Ordering::Greater,
//                         std::cmp::Ordering::Equal => continue,
//                     },
//                     (Some(_), None) => break std::cmp::Ordering::Greater,
//                     (None, Some(_)) => break std::cmp::Ordering::Less,
//                     (None, None) => break std::cmp::Ordering::Equal,
//                 }
//             }
//         }
//         fn dfs(node: &TreeNode) -> Vec<u8> {
//             match (node.left.as_deref(), node.right.as_deref()) {
//                 (None, None) => vec![node.val as u8 + b'a'],
//                 (Some(left), None) => {
//                     let mut left = dfs(&left.borrow());
//                     left.push(node.val as u8 + b'a');
//                     left
//                 }
//                 (None, Some(right)) => {
//                     let mut right = dfs(&right.borrow());
//                     right.push(node.val as u8 + b'a');
//                     right
//                 }
//                 (Some(left), Some(right)) => {
//                     let mut left = dfs(&left.borrow());
//                     let mut right = dfs(&right.borrow());
//                     left.push(node.val as u8 + b'a');
//                     right.push(node.val as u8 + b'a');
//                     // Select by lexicographically smallest path
//                     std::cmp::min_by(left, right, compare_vec_lexographic)
//                 }
//             }
//         }
//         match root {
//             Some(root) => {
//                 let root = root.borrow();
//                 let path = dfs(&root);
//                 unsafe { String::from_utf8_unchecked(path) }
//             }
//             None => "".to_string(),
//         }
//     }
// }

impl Solution {
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn dfs(node: &TreeNode, path: &mut Vec<u8>, result: &mut Vec<u8>) {
            path.push(node.val as u8 + b'a');
            match (node.left.as_deref(), node.right.as_deref()) {
                (None, None) => {
                    let rev_path = path.iter().rev().copied().collect::<Vec<u8>>();
                    if result.is_empty() || rev_path < *result {
                        *result = rev_path;
                    }
                }
                (Some(left), None) => dfs(&left.borrow(), path, result),
                (None, Some(right)) => dfs(&right.borrow(), path, result),
                (Some(left), Some(right)) => {
                    dfs(&left.borrow(), path, result);
                    dfs(&right.borrow(), path, result);
                }
            }
            path.pop();
        }
        if let Some(root) = root {
            let mut result = Vec::new();
            dfs(&root.borrow(), &mut Vec::new(), &mut result);
            unsafe { String::from_utf8_unchecked(result) }
        } else {
            "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(leetcode_slice: &[Option<i32>], expected: &str) {
        let root = TreeNode::from_leetcode_slice(leetcode_slice);
        let result = Solution::smallest_from_leaf(root);
        assert_eq!(result, expected);
    }

    #[test]
    fn ex1() {
        test(
            &[
                Some(0),
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(3),
                Some(4),
            ],
            "dba",
        )
    }

    #[test]
    fn ex2() {
        test(
            &[
                Some(25),
                Some(1),
                Some(3),
                Some(1),
                Some(3),
                Some(0),
                Some(2),
            ],
            "adz",
        )
    }

    #[test]
    fn ex3() {
        test(
            &[
                Some(2),
                Some(2),
                Some(1),
                None,
                Some(1),
                Some(0),
                None,
                Some(0),
            ],
            "abc",
        )
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                Some(25),
                Some(1),
                None,
                Some(0),
                Some(0),
                Some(1),
                None,
                None,
                None,
                Some(0),
            ],
            "ababz",
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
            "hud",
        )
    }

    #[test]
    fn discussion_case2_variant1() {
        test(
            &[Some(3), Some(8), Some(20), None, None, Some(15), Some(7)],
            "hud",
        )
    }

    #[test]
    fn discussion_case3() {
        test(&[Some(4), Some(0), Some(1), Some(1)], "bae")
    }

    #[test]
    fn discussion_case4() {
        test(
            &[
                Some(2),
                Some(1),
                Some(1),
                None,
                Some(1),
                Some(0),
                None,
                Some(0),
            ],
            "abbc",
        )
    }
}
