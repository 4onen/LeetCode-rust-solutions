// https://leetcode.com/problems/number-of-good-leaf-nodes-pairs/

use utils::TreeNode;
pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

// Initial sol'n
// impl Solution {
//     pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
//         const MAX_DIST: u8 = 10;
//         type DistArr = [u16; MAX_DIST as usize];
//         assert!(distance >= 1);
//         assert!(distance <= MAX_DIST as i32);
//         let distance = distance as u8;
//         let mut ans = 0;
//         fn advance_distances(dists: &mut DistArr) {
//             dists.copy_within(0..MAX_DIST as usize - 1, 1);
//             dists[0] = 0;
//         }
//         fn unwrap_rc_refcell(node: Rc<RefCell<TreeNode>>) -> TreeNode {
//             std::rc::Rc::into_inner(node).unwrap().into_inner()
//         }
//         fn dfs(root: TreeNode, distance: u8, ans: &mut i32) -> DistArr {
//             let mut dists = [0; MAX_DIST as usize];
//             match (root.left, root.right) {
//                 (None, None) => {
//                     dists[0] = 1;
//                     return dists;
//                 }
//                 (None, Some(right)) => {
//                     dists = dfs(unwrap_rc_refcell(right), distance, ans);
//                 }
//                 (Some(left), None) => {
//                     dists = dfs(unwrap_rc_refcell(left), distance, ans);
//                 }
//                 (Some(left), Some(right)) => {
//                     let left = dfs(unwrap_rc_refcell(left), distance, ans);
//                     let right = dfs(unwrap_rc_refcell(right), distance, ans);
//                     for i in 0..MAX_DIST as usize {
//                         for j in 0..MAX_DIST as usize {
//                             if i + j + 2 <= distance as usize {
//                                 *ans += (left[i] * right[j]) as i32;
//                             }
//                         }
//                         dists[i] = left[i] + right[i];
//                     }
//                 }
//             }
//             advance_distances(&mut dists);
//             dists
//         }
//         if let Some(root) = root {
//             dfs(unwrap_rc_refcell(root), distance, &mut ans);
//         }
//         ans
//     }
// }

// Reduced unwrap checks
impl Solution {
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        const MAX_DIST: u8 = 10;
        type DistArr = [u16; MAX_DIST as usize];
        assert!(distance >= 1);
        assert!(distance <= MAX_DIST as i32);
        let distance = distance as u8;
        let mut ans = 0;
        fn dfs(root: Rc<RefCell<TreeNode>>, distance: u8, ans: &mut i32) -> DistArr {
            let mut dists = [0; MAX_DIST as usize];
            let root = std::rc::Rc::into_inner(root).unwrap().into_inner();
            match (root.left, root.right) {
                (None, None) => {
                    dists[0] = 1;
                    return dists;
                }
                (None, Some(right)) => {
                    dists = dfs(right, distance, ans);
                }
                (Some(left), None) => {
                    dists = dfs(left, distance, ans);
                }
                (Some(left), Some(right)) => {
                    let left = dfs(left, distance, ans);
                    let right = dfs(right, distance, ans);
                    for i in 0..MAX_DIST as usize {
                        for j in 0..MAX_DIST as usize {
                            if i + j + 2 <= distance as usize {
                                *ans += (left[i] * right[j]) as i32;
                            }
                        }
                        dists[i] = left[i] + right[i];
                    }
                }
            }
            dists.copy_within(0..MAX_DIST as usize - 1, 1);
            dists[0] = 0;
            dists
        }
        if let Some(root) = root {
            dfs(root, distance, &mut ans);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &[Option<i32>], distance: i32, expected: i32) {
        let node_count = input.iter().filter(|x| x.is_some()).count();
        assert!(node_count >= 1);
        assert!(node_count <= 1024);
        for n in input.iter() {
            if let Some(x) = n {
                assert!(*x >= 1);
                assert!(*x <= 100);
            }
        }
        assert!(distance >= 1);
        assert!(distance <= 10);
        assert_eq!(
            Solution::count_pairs(TreeNode::from_leetcode_slice(input), distance),
            expected
        )
    }

    #[test]
    fn ex1() {
        test(&[Some(1), Some(2), Some(3), None, Some(4)], 3, 1)
    }

    #[test]
    fn ex2() {
        test(
            &[
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
            ],
            3,
            2,
        )
    }

    #[test]
    fn ex3() {
        test(
            &[
                Some(7),
                Some(1),
                Some(4),
                Some(6),
                None,
                Some(5),
                Some(3),
                None,
                None,
                None,
                None,
                None,
                Some(2),
            ],
            3,
            1,
        )
    }

    #[test]
    fn myex1() {
        test(&[Some(100); 1024], 10, 7920)
    }

    #[test]
    fn discussion_case1() {
        test(&[Some(1), Some(1)], 2, 0)
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                Some(7),
                Some(1),
                Some(4),
                Some(6),
                None,
                Some(5),
                Some(3),
                None,
                None,
                None,
                None,
                None,
                Some(2),
            ],
            3,
            1,
        )
    }

    #[test]
    fn discussion_case3() {
        test(
            &[
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
                Some(8),
                Some(9),
                Some(10),
                Some(11),
                Some(12),
                Some(13),
                Some(14),
                Some(15),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(16),
                Some(17),
            ],
            6,
            24,
        )
    }

    #[test]
    fn discussion_case4() {
        test(
            &[
                Some(50),
                Some(30),
                Some(70),
                Some(20),
                Some(40),
                Some(60),
                Some(80),
                Some(10),
                Some(25),
                Some(35),
                Some(45),
                Some(55),
                Some(65),
                Some(75),
                Some(85),
                Some(5),
                Some(15),
                Some(22),
                Some(27),
                Some(33),
                Some(37),
                Some(42),
                Some(47),
                Some(52),
                Some(57),
                Some(62),
                Some(67),
                Some(72),
                Some(77),
                Some(82),
                Some(87),
                Some(3),
                Some(8),
                Some(12),
                Some(18),
                Some(23),
                Some(28),
                Some(32),
                Some(36),
                Some(41),
                Some(46),
                Some(51),
                Some(56),
                Some(61),
                Some(66),
                Some(71),
                Some(76),
                Some(81),
                Some(86),
                Some(2),
                Some(4),
                Some(7),
                Some(9),
                Some(13),
                Some(17),
                Some(21),
                Some(24),
                Some(29),
                Some(34),
                Some(39),
                Some(44),
                Some(49),
                Some(54),
                Some(59),
                Some(64),
                Some(69),
                Some(74),
                Some(79),
                Some(84),
                Some(89),
                Some(92),
                Some(94),
                Some(96),
                Some(98),
            ],
            10,
            490,
        )
    }
}
