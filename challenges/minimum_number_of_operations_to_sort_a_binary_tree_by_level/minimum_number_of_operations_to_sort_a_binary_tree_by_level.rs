// https://leetcode.com/problems/minimum-number-of-operations-to-sort-a-binary-tree-by-level/

use utils::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let mut nodes = vec![root.expect("Tree cannot be empty")];
        let mut next_nodes = std::vec::Vec::new();
        while nodes.len() > 0 {
            let mut sorted = true;
            let mut last = 0;
            for i in 0..nodes.len() {
                let node = nodes[i].borrow();
                if node.val < last {
                    sorted = false;
                }
                last = node.val;
                if let Some(left) = node.left.clone() {
                    next_nodes.push(left);
                }
                if let Some(right) = node.right.clone() {
                    next_nodes.push(right);
                }
            }
            if sorted {
                nodes.clear();
            } else {
                #[derive(Debug, PartialEq, Eq)]
                struct IntWithPos {
                    val: i32,
                    pos: u32,
                }
                impl PartialOrd for IntWithPos {
                    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                        Some(self.cmp(other))
                    }
                }
                impl Ord for IntWithPos {
                    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                        self.val.cmp(&other.val)
                    }
                }
                let mut values: Vec<_> = nodes
                    .drain(..)
                    .enumerate()
                    .map(|(i, node)| IntWithPos {
                        val: node.borrow().val,
                        pos: i as u32,
                    })
                    .collect();
                values.sort_unstable();
                let mut seen = vec![false; values.len()];
                for i in 0..values.len() as u32 {
                    if seen[i as usize] || i == values[i as usize].pos {
                        continue;
                    }
                    let mut cycle_len = 0;
                    let mut j = i;
                    while !seen[j as usize] {
                        seen[j as usize] = true;
                        j = values[j as usize].pos;
                        cycle_len += 1;
                    }
                    if cycle_len > 0 {
                        res += cycle_len - 1;
                    }
                }
            }
            std::mem::swap(&mut nodes, &mut next_nodes);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(tree: &[Option<i32>], expected: i32) {
        let node_count = tree
            .iter()
            .copied()
            .map(|x| x.is_some() as u32)
            .sum::<u32>();
        assert!(node_count >= 1);
        assert!(node_count <= 100_000);
        let mut seen = std::collections::HashSet::new();
        for &val in tree {
            let Some(val) = val else {
                continue;
            };
            assert!(val >= 1);
            assert!(val <= 100_000);
            assert!(seen.insert(val));
        }
        assert_eq!(
            Solution::minimum_operations(TreeNode::from_leetcode_slice(tree)),
            expected
        );
    }

    #[test]
    fn ex1() {
        let null = None;
        test(
            &[
                Some(1),
                Some(4),
                Some(3),
                Some(7),
                Some(6),
                Some(8),
                Some(5),
                null,
                null,
                null,
                null,
                Some(9),
                null,
                Some(10),
            ],
            3,
        )
    }

    #[test]
    fn ex2() {
        test(
            &[
                Some(1),
                Some(3),
                Some(2),
                Some(7),
                Some(6),
                Some(5),
                Some(4),
            ],
            3,
        )
    }

    #[test]
    fn ex3() {
        test(&[Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)], 0)
    }

    #[test]
    fn discussion_case1() {
        let null = None;
        test(&[Some(5), Some(4), null, Some(8), null, null, Some(1)], 0)
    }

    #[test]
    fn discussion_case2() {
        let null = None;
        test(&[Some(1), Some(3), null, null, Some(4), null, Some(5)], 0)
    }

    #[test]
    fn discussion_case3() {
        let null = None;
        test(
            &[
                Some(50),
                Some(38),
                Some(30),
                Some(45),
                null,
                Some(1),
                null,
                null,
                Some(31),
            ],
            2,
        )
    }

    #[test]
    fn discussion_case4() {
        let null = None;
        test(
            &[
                Some(29),
                Some(13),
                Some(40),
                Some(32),
                null,
                Some(20),
                null,
                null,
                null,
                Some(49),
                Some(21),
                Some(50),
                null,
                null,
                null,
                Some(1),
            ],
            2,
        )
    }

    #[test]
    fn discussion_case5() {
        let null = None;
        test(
            &[
                Some(370),
                Some(108),
                Some(365),
                Some(235),
                Some(447),
                Some(301),
                Some(26),
                Some(311),
                Some(259),
                null,
                null,
                null,
                null,
                null,
                Some(21),
                null,
                null,
                null,
                Some(274),
                Some(352),
                null,
                null,
                null,
                Some(401),
                Some(467),
            ],
            3,
        )
    }

    #[test]
    fn discussion_case6() {
        let null = None;
        test(
            &[
                Some(789),
                Some(637),
                Some(24),
                Some(321),
                Some(247),
                Some(81),
                Some(171),
                Some(285),
                Some(367),
                null,
                null,
                Some(69),
                Some(478),
                Some(483),
                Some(76),
                Some(46),
                Some(454),
                null,
                Some(437),
                Some(1863),
                Some(348),
                null,
                Some(466),
                Some(262),
                null,
                Some(205),
                Some(40),
                null,
                Some(441),
                Some(473),
                Some(202),
                Some(246),
                Some(332),
                Some(488),
                Some(183),
                Some(394),
                null,
                Some(411),
                null,
                Some(191),
                null,
                Some(176),
                Some(33),
                Some(203),
                null,
                Some(302),
                null,
                null,
                Some(58),
                Some(53),
                Some(365),
                null,
                Some(343),
                null,
                Some(138),
                null,
                Some(43),
                Some(56),
                null,
                Some(233),
                null,
                null,
                Some(122),
                null,
                null,
                Some(407),
                Some(465),
                null,
                Some(154),
                Some(312),
                Some(420),
                null,
                null,
                null,
                null,
                Some(425),
                Some(93),
                Some(222),
                Some(17),
                Some(101),
                null,
                null,
                Some(112),
                null,
                null,
                Some(259),
                null,
                null,
                null,
                null,
                null,
                Some(353),
                Some(250),
                Some(275),
                null,
                null,
                Some(168),
                null,
                null,
                null,
                null,
                Some(391),
                Some(458),
                Some(277),
                null,
                Some(288),
                null,
                null,
                null,
                Some(268),
                Some(244),
                Some(393),
                Some(339),
                null,
                null,
                Some(482),
                null,
                null,
                Some(481),
                null,
                Some(360),
                null,
                null,
                null,
                null,
                null,
                Some(99),
                null,
                null,
                Some(220),
                null,
                null,
                null,
                null,
                Some(374),
                null,
                Some(399),
                Some(270),
                null,
                null,
                null,
                Some(371),
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                Some(177),
                null,
                Some(79),
            ],
            55,
        )
    }

    #[test]
    fn discussion_case7() {
        let input_string = include_str!("discussion_case7.txt");
        let inputs = input_string[1..input_string.len() - 2]
            .split(',')
            .map(|s| {
                if s == "null" {
                    None
                } else {
                    Some(s.parse().unwrap())
                }
            })
            .collect::<Vec<Option<i32>>>();
        test(&inputs, 1215)
    }
}
