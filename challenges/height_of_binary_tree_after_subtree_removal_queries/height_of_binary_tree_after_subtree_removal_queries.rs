// https://leetcode.com/problems/height-of-binary-tree-after-subtree-removal-queries/

use std::cell::RefCell;
use std::rc::Rc;
use utils::TreeNode;

pub struct Solution;

// Initial sol'n (Stack overflow for big cases.)
// impl Solution {
//     pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
//         #[derive(Clone, Copy)]
//         struct LevelInfo {
//             max_depth_node: std::num::NonZeroI32,
//             max_depth: i32,
//             second_max_depth: i32,
//         }
//         struct UnsafeDenseNumberMap<T> {
//             data: Vec<Option<T>>,
//         }
//         impl<T> UnsafeDenseNumberMap<T>
//         where
//             T: Copy,
//         {
//             fn new() -> Self {
//                 Self { data: Vec::new() }
//             }
//             fn insert<K>(&mut self, key: K, value: T)
//             where
//                 K: TryInto<usize> + Copy,
//             {
//                 let Ok(key) = key.try_into() else {
//                     panic!("key into usize failed");
//                 };
//                 if key >= self.data.len() {
//                     self.data.resize(key + 1, None);
//                 }
//                 self.data[key] = Some(value);
//             }
//             fn get_slot_mut<K>(&mut self, key: K) -> &mut Option<T>
//             where
//                 K: TryInto<usize> + Copy,
//             {
//                 let Ok(key) = key.try_into() else {
//                     panic!("key into usize failed");
//                 };
//                 if key >= self.data.len() {
//                     self.data.resize(key + 1, None);
//                 }
//                 &mut self.data[key]
//             }
//             fn get_unsafe<K>(&self, key: K) -> T
//             where
//                 K: TryInto<usize> + Copy,
//             {
//                 let Ok(key) = key.try_into() else {
//                     panic!("key into usize failed");
//                 };
//                 self.data[key].unwrap()
//             }
//         }
//         let mut node_to_level = UnsafeDenseNumberMap::new();
//         let mut level_info = UnsafeDenseNumberMap::new();
//         fn dfs(
//             node: &Option<Rc<RefCell<TreeNode>>>,
//             level: i32,
//             node_to_level: &mut UnsafeDenseNumberMap<i32>,
//             level_info: &mut UnsafeDenseNumberMap<LevelInfo>,
//         ) -> i32 {
//             if let Some(n) = node {
//                 let n = n.borrow();
//                 let left = dfs(&n.left, level + 1, node_to_level, level_info);
//                 let right = dfs(&n.right, level + 1, node_to_level, level_info);
//                 let depth = std::cmp::max(left, right) + 1;
//                 node_to_level.insert(n.val, level);
//                 match level_info.get_slot_mut(level) {
//                     Some(info) => {
//                         if depth > info.max_depth {
//                             info.second_max_depth = info.max_depth;
//                             info.max_depth = depth;
//                             info.max_depth_node = std::num::NonZeroI32::new(n.val).unwrap();
//                         } else if depth > info.second_max_depth {
//                             info.second_max_depth = depth;
//                         }
//                     }
//                     slot => {
//                         *slot = Some(LevelInfo {
//                             max_depth_node: std::num::NonZeroI32::new(n.val).unwrap(),
//                             max_depth: depth,
//                             second_max_depth: 0,
//                         });
//                     }
//                 }
//                 depth
//             } else {
//                 0
//             }
//         }
//         let max_depth = dfs(&root, 0, &mut node_to_level, &mut level_info) - 1;
//         queries
//             .into_iter()
//             .map(|q| {
//                 let level = node_to_level.get_unsafe(q);
//                 let info = level_info.get_unsafe(level);
//                 if info.max_depth_node.get() == q {
//                     level as i32 + info.second_max_depth as i32 - 1
//                 } else {
//                     max_depth as i32
//                 }
//             })
//             .collect()
//     }
// }

// Second sol'n (DFS with manual stack)
impl Solution {
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        #[derive(Clone, Copy)]
        struct LevelInfo {
            max_depth_node: i32,
            max_depth: i32,
            second_max_depth: i32,
        }
        struct UnsafeDenseNumberMap<T> {
            data: Vec<Option<T>>,
        }
        impl<T> UnsafeDenseNumberMap<T>
        where
            T: Copy,
        {
            fn new() -> Self {
                Self { data: Vec::new() }
            }
            fn insert<K>(&mut self, key: K, value: T)
            where
                K: TryInto<usize> + Copy,
            {
                let Ok(key) = key.try_into() else {
                    panic!("key into usize failed");
                };
                if key >= self.data.len() {
                    self.data.resize(key + 1, None);
                }
                self.data[key] = Some(value);
            }
            fn get_slot_mut<K>(&mut self, key: K) -> &mut Option<T>
            where
                K: TryInto<usize> + Copy,
            {
                let Ok(key) = key.try_into() else {
                    panic!("key into usize failed");
                };
                if key >= self.data.len() {
                    self.data.resize(key + 1, None);
                }
                &mut self.data[key]
            }
            fn get_unsafe<K>(&self, key: K) -> T
            where
                K: TryInto<usize> + Copy,
            {
                let Ok(key) = key.try_into() else {
                    panic!("key into usize failed");
                };
                self.data[key].unwrap()
            }
        }
        let mut node_to_level = UnsafeDenseNumberMap::new();
        let mut level_info: UnsafeDenseNumberMap<LevelInfo> = UnsafeDenseNumberMap::new();
        {
            // DFS with manual stack
            struct WaitingLeftStackFrame {
                val: i32,
                right: Option<Rc<RefCell<TreeNode>>>,
            }
            struct WaitingRightStackFrame {
                val: i32,
                left_depth: i32,
            }
            enum StackFrame {
                Start(Option<Rc<RefCell<TreeNode>>>),
                WaitingLeft(WaitingLeftStackFrame),
                WaitingRight(WaitingRightStackFrame),
            }
            let mut stack = std::vec::Vec::new();
            let mut depth_return_val = 0;
            stack.push(StackFrame::Start(root));
            while let Some(frame) = stack.pop() {
                match frame {
                    StackFrame::Start(node) => {
                        if let Some(n) = node {
                            let mut n = n.borrow_mut();
                            let level = stack.len() as i32;
                            node_to_level.insert(n.val, level);
                            stack.push(StackFrame::WaitingLeft(WaitingLeftStackFrame {
                                val: n.val,
                                right: n.right.take(),
                            }));
                            stack.push(StackFrame::Start(n.left.take()));
                        } else {
                            depth_return_val = 0;
                        }
                    }
                    StackFrame::WaitingLeft(WaitingLeftStackFrame { val, right }) => {
                        stack.push(StackFrame::WaitingRight(WaitingRightStackFrame {
                            val,
                            left_depth: depth_return_val,
                        }));
                        stack.push(StackFrame::Start(right));
                    }
                    StackFrame::WaitingRight(WaitingRightStackFrame { val, left_depth }) => {
                        let right_depth = depth_return_val;
                        let depth = std::cmp::max(left_depth, right_depth) + 1;
                        let level = stack.len() as i32;
                        match level_info.get_slot_mut(level) {
                            Some(info) => {
                                if depth > info.max_depth {
                                    info.second_max_depth = info.max_depth;
                                    info.max_depth = depth;
                                    info.max_depth_node = val;
                                } else if depth > info.second_max_depth {
                                    info.second_max_depth = depth;
                                }
                            }
                            slot => {
                                *slot = Some(LevelInfo {
                                    max_depth_node: val,
                                    max_depth: depth,
                                    second_max_depth: 0,
                                });
                            }
                        }
                        depth_return_val = depth;
                    }
                }
            }
        };
        queries
            .into_iter()
            .map(|q| {
                let level = node_to_level.get_unsafe(q);
                let info = level_info.get_unsafe(level);
                level as i32 - 1
                    + if info.max_depth_node == q {
                        info.second_max_depth as i32
                    } else {
                        info.max_depth as i32
                    }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &[Option<i32>], queries: &[i32], expected: &[i32]) {
        let input_nodes = input.iter().filter(|x| x.is_some()).count() as i32;
        assert!(input_nodes >= 2);
        assert!(input_nodes <= 100_000);
        let mut seen = std::collections::HashSet::new();
        for i in 0..input.len() {
            if let Some(x) = input[i] {
                assert!(x >= 1);
                assert!(x <= input_nodes);
                assert!(seen.insert(x));
            }
        }
        assert!(queries.len() >= 1);
        assert!(queries.len() <= std::cmp::min(10_000, input_nodes as usize));
        for i in 0..queries.len() {
            assert!(queries[i] >= 1);
            assert!(queries[i] <= input_nodes);
            assert!(Some(queries[i]) != input[0]);
        }

        let root = TreeNode::from_leetcode_slice(input);
        assert_eq!(Solution::tree_queries(root, queries.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(
            &[
                Some(1),
                Some(3),
                Some(4),
                Some(2),
                None,
                Some(6),
                Some(5),
                None,
                None,
                None,
                None,
                None,
                Some(7),
            ],
            &[4],
            &[2],
        )
    }

    #[test]
    fn ex2() {
        test(
            &[
                Some(5),
                Some(8),
                Some(9),
                Some(2),
                Some(1),
                Some(3),
                Some(7),
                Some(4),
                Some(6),
            ],
            &[3, 2, 4, 8],
            &[3, 2, 3, 2],
        )
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                Some(2),
                Some(1),
                Some(5),
                None,
                None,
                Some(3),
                Some(6),
                None,
                Some(4),
            ],
            &[1, 5, 5, 6, 4, 5],
            &[3, 1, 1, 3, 2, 1],
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                Some(7),
                Some(2),
                Some(9),
                Some(1),
                Some(3),
                Some(8),
                Some(12),
                None,
                None,
                None,
                Some(6),
                None,
                None,
                Some(10),
                None,
                Some(4),
                None,
                None,
                Some(11),
                None,
                Some(5),
            ],
            &[3, 8, 9, 10, 10, 5, 6, 12, 2, 6, 1, 11],
            &[4, 5, 5, 5, 5, 4, 4, 5, 4, 4, 5, 5],
        )
    }

    #[test]
    fn discussion_case3() {
        test(
            &[Some(1), None, Some(5), Some(3), None, Some(2), Some(4)],
            &[3, 5, 4, 2, 4],
            &[1, 0, 3, 3, 3],
        )
    }

    #[test]
    fn failing_case1() {
        // Forgot unbalanced trees exist
        let input_str = include_str!("failing_case1.txt");
        let queries_str = include_str!("failing_case1_queries.txt");
        let expected_str = include_str!("failing_case1_expected.txt");
        let input: Vec<Option<i32>> = input_str
            .lines()
            .map(|x| {
                if x == "null" {
                    None
                } else {
                    Some(x.parse().unwrap())
                }
            })
            .collect();
        let queries: Vec<i32> = queries_str.lines().map(|x| x.parse().unwrap()).collect();
        let expected: Vec<i32> = expected_str.lines().map(|x| x.parse().unwrap()).collect();
        test(&input, &queries, &expected)
    }
}
