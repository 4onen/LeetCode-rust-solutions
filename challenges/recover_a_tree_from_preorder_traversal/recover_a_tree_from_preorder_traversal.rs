// https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/

use utils::TreeNode;

pub struct Solution;

// Initial sol'n
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let depths_and_values = {
            let (mut results, depth, value) = traversal.into_bytes().into_iter().fold(
                (Vec::new(), 0u16, 0),
                |(mut results, mut depth, mut value), b| {
                    match b {
                        b'-' => {
                            if value > 0 {
                                results.push((depth, value));
                                depth = 0;
                                value = 0;
                            }
                            depth += 1;
                        }
                        b'0'..=b'9' => {
                            value = value * 10 + (b - b'0') as i32;
                        }
                        _ => unreachable!(),
                    }
                    (results, depth, value)
                },
            );
            results.push((depth, value));
            results
        };
        fn build_tree(
            depths_and_values: &[(u16, i32)],
            index: &mut u16,
            current_depth: u16,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if *index >= depths_and_values.len() as u16 {
                return None;
            }
            let (node_depth, val) = depths_and_values[*index as usize];
            assert!(current_depth >= node_depth);
            if current_depth > node_depth {
                return None;
            }
            *index += 1;
            let left = build_tree(depths_and_values, index, current_depth + 1);
            let right = build_tree(depths_and_values, index, current_depth + 1);
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }
        build_tree(&depths_and_values, &mut 0, 0)
    }
}

// std::iter::from_fn code simplification?
// use std::cell::RefCell;
// use std::rc::Rc;
// impl Solution {
//     pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
//         let depths_and_values = {
//             let mut traversal = traversal.into_bytes().into_iter();
//             let mut depth = 0;
//             let mut value = 0;
//             std::iter::from_fn(move || {
//                 while let Some(b) = traversal.next() {
//                     match b {
//                         b'-' => {
//                             if value > 0 {
//                                 let out = (depth, value);
//                                 depth = 1;
//                                 value = 0;
//                                 return Some(out);
//                             }
//                             depth += 1;
//                         }
//                         b'0'..=b'9' => {
//                             value = value * 10 + (b - b'0') as i32;
//                         }
//                         _ => unreachable!(),
//                     }
//                 }
//                 if value > 0 {
//                     let out = (depth, value);
//                     value = 0;
//                     return Some(out);
//                 } else {
//                     None
//                 }
//             })
//         };
//         fn build_tree(
//             depths_and_values: &mut std::iter::Peekable<impl Iterator<Item = (u16, i32)>>,
//             current_depth: u16,
//         ) -> Option<Rc<RefCell<TreeNode>>> {
//             let Some((_, val)) =
//                 depths_and_values.next_if(|(node_depth, _)| *node_depth == current_depth)
//             else {
//                 return None;
//             };
//             let left = build_tree(depths_and_values, current_depth + 1);
//             let right = build_tree(depths_and_values, current_depth + 1);
//             Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
//         }
//         build_tree(&mut depths_and_values.peekable(), 0)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(traversal: &str, expected: &[Option<i32>]) {
        assert!(traversal.len() >= 1);
        for &b in traversal.as_bytes() {
            assert!(b.is_ascii_digit() || b == b'-');
        }
        assert_ne!(traversal.as_bytes().last().copied().unwrap(), b'-');
        let mut node_count = 0;
        for &node in expected {
            if let Some(n) = node {
                assert!(n >= 1);
                assert!(n <= 1_000_000_000);
                node_count += 1;
            }
        }
        assert!(node_count >= 1);
        assert!(node_count <= 1000, "Node count too large: {}", node_count);
        let result = Solution::recover_from_preorder(traversal.to_owned());
        fn linearize_to_leetcode_slice(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
            let mut result = Vec::new();
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(node);
            while let Some(node) = queue.pop_front() {
                if let Some(node) = node {
                    result.push(Some(node.borrow().val));
                    queue.push_back(node.borrow_mut().left.take());
                    queue.push_back(node.borrow_mut().right.take());
                } else {
                    result.push(None);
                }
            }
            while result.last().copied().unwrap().is_none() {
                result.pop();
            }
            result
        }
        let result_linearized = linearize_to_leetcode_slice(result);
        assert_eq!(result_linearized, expected);
    }

    #[test]
    fn ex1() {
        // [1,2,5,3,4,6,7]
        test(
            "1-2--3--4-5--6--7",
            &[
                Some(1),
                Some(2),
                Some(5),
                Some(3),
                Some(4),
                Some(6),
                Some(7),
            ],
        )
    }

    #[test]
    fn ex2() {
        // [1,2,5,3,null,6,null,4,null,7]
        test(
            "1-2--3---4-5--6---7",
            &[
                Some(1),
                Some(2),
                Some(5),
                Some(3),
                None,
                Some(6),
                None,
                Some(4),
                None,
                Some(7),
            ],
        )
    }

    #[test]
    fn ex3() {
        // [1,401,null,349,88,90]
        test(
            "1-401--349---90--88",
            &[Some(1), Some(401), None, Some(349), Some(88), Some(90)],
        )
    }

    #[test]
    fn discussion_case1() {
        // [4,4,7,8,null,7,null,5]
        test(
            "4-4--8---5-7--7",
            &[
                Some(4),
                Some(4),
                Some(7),
                Some(8),
                None,
                Some(7),
                None,
                Some(5),
            ],
        )
    }

    #[test]
    fn myex1() {
        // All left string tree
        test(
            "1-2--3---4----5-----6------7-------8--------9---------10----------11-----------12------------13-------------14--------------15",
            &[
                Some(1),
                Some(2),
                None,
                Some(3),
                None,
                Some(4),
                None,
                Some(5),
                None,
                Some(6),
                None,
                Some(7),
                None,
                Some(8),
                None,
                Some(9),
                None,
                Some(10),
                None,
                Some(11),
                None,
                Some(12),
                None,
                Some(13),
                None,
                Some(14),
                None,
                Some(15),
            ]
        )
    }

    #[test]
    fn my_extreme_ex1() {
        // All left string tree
        let mut input = std::vec::Vec::new();
        let mut expected = std::vec::Vec::new();
        for i in 1..=1000 {
            input.extend(std::iter::repeat(b'-').take(i as usize - 1));
            input.extend_from_slice(i.to_string().as_bytes());
            expected.push(Some(i));
            if i > 1 {
                expected.push(None);
            }
        }
        while expected.last().copied().unwrap().is_none() {
            expected.pop();
        }
        test(unsafe { std::str::from_utf8_unchecked(&input) }, &expected)
    }
}
