// https://leetcode.com/problems/maximum-width-of-binary-tree/

use utils::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
// Vec-expansion BFS -- too much memory use
// impl Solution {
//     pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         let mut max_width = 1;
//         let mut level = vec![root];

//         loop {
//             level = level
//                 .into_iter()
//                 .flat_map(|node| match node {
//                     Some(node) => {
//                         let node = node.borrow();
//                         vec![node.left.clone(), node.right.clone()]
//                     }
//                     None => vec![None, None],
//                 })
//                 .collect();

//             let p1 = level.iter().position(|node| node.is_some());
//             let p2 = level.iter().rposition(|node| node.is_some());
//             let width = Option::zip(p1, p2).map(|(p1, p2)| p2 - p1 + 1);

//             if let Some(width) = width {
//                 max_width = max_width.max(width as i32);
//             } else {
//                 break max_width;
//             }
//         }
//     }
// }

// BFS with queue
impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let mut max_width = 1;
            let mut queue = vec![(root, 0u32)];
            let mut level = 0;

            while !queue.is_empty() {
                level += 1;
                dbg!(level, &queue.iter().map(|t| t.1).collect::<Vec<_>>());
                let mut next_queue = Vec::new();
                let mut ps: Option<(u32, u32)> = None;

                for (node, pos) in queue.into_iter() {
                    // Use try_unwrap to avoid cloning the
                    // node when nobody else is using it.
                    match Rc::try_unwrap(node).map(RefCell::into_inner) {
                        Ok(node) => {
                            if let Some(left) = node.left {
                                next_queue.push((left, pos * 2))
                            }
                            if let Some(right) = node.right {
                                next_queue.push((right, pos * 2 + 1))
                            }
                        }
                        Err(node) => {
                            let node = node.borrow();
                            if let Some(left) = node.left.as_ref() {
                                next_queue.push((left.clone(), pos * 2))
                            }
                            if let Some(right) = node.right.as_ref() {
                                next_queue.push((right.clone(), pos * 2 + 1))
                            }
                        }
                    }

                    ps = match ps {
                        Some((p1, p2)) => Some((p1.min(pos), p2.max(pos))),
                        None => Some((pos, pos)),
                    }
                }

                if let Some((p1, p2)) = ps {
                    let width = p2 - p1 + 1;
                    dbg!(width);
                    max_width = max_width.max(width);
                }

                queue = next_queue;
            }
            max_width as i32
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_leetcode_slice_ex2() {
        #[rustfmt::skip]
        const INPUT: &[Option<i32>] = &[Some(1),Some(3),Some(2),Some(5),None,None,Some(9),Some(6),None,Some(7)];

        let root = TreeNode::from_leetcode_slice(INPUT);

        let expected_root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let left_left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let right_right = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let left_left_left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let right_right_left = Some(Rc::new(RefCell::new(TreeNode::new(7))));

        right_right.as_ref().unwrap().borrow_mut().left = right_right_left;
        left_left.as_ref().unwrap().borrow_mut().left = left_left_left;
        left.as_ref().unwrap().borrow_mut().left = left_left;
        right.as_ref().unwrap().borrow_mut().right = right_right;
        expected_root.as_ref().unwrap().borrow_mut().left = left;
        expected_root.as_ref().unwrap().borrow_mut().right = right;

        assert_eq!(root, expected_root);
    }

    #[test]
    fn from_leetcode_slice_ex3() {
        #[rustfmt::skip]
        const INPUT: &[Option<i32>] = &[Some(1),Some(3),Some(2),Some(5)];

        let root = TreeNode::from_leetcode_slice(INPUT);

        let expected_root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let left_left = Some(Rc::new(RefCell::new(TreeNode::new(5))));

        left.as_ref().unwrap().borrow_mut().left = left_left;
        expected_root.as_ref().unwrap().borrow_mut().left = left;
        expected_root.as_ref().unwrap().borrow_mut().right = right;

        assert_eq!(root, expected_root);
    }

    #[test]
    fn ex1() {
        #[rustfmt::skip]
        const INPUT: &[Option<i32>] = &[Some(1),Some(3),Some(2),Some(5),Some(3),None,Some(9)];
        let root = TreeNode::from_leetcode_slice(INPUT);

        assert_eq!(Solution::width_of_binary_tree(root), 4);
    }

    #[test]
    fn ex2() {
        #[rustfmt::skip]
        const INPUT: &[Option<i32>] = &[Some(1),Some(3),Some(2),Some(5),None,None,Some(9),Some(6),None,Some(7)];

        let root = TreeNode::from_leetcode_slice(INPUT);

        assert_eq!(Solution::width_of_binary_tree(root), 7);
    }

    #[test]
    fn ex3() {
        #[rustfmt::skip]
        const INPUT: &[Option<i32>] = &[Some(1),Some(3),Some(2),Some(5)];

        let root = TreeNode::from_leetcode_slice(INPUT);

        assert_eq!(Solution::width_of_binary_tree(root), 2);
    }

    #[test]
    fn memory_limit_failing_case() {
        #[rustfmt::skip]
        const INPUT: [Option<i32>;271] = [Some(0),Some(0),Some(0),None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None,None,Some(0),Some(0),None];

        let root = TreeNode::from_slice(&INPUT);
        assert_eq!(Solution::width_of_binary_tree(root), 44); //Hope this is actually right!
    }

    #[test]
    fn wrong_answer_failing_case() {
        #[rustfmt::skip]
        const INPUT: [Option<i32>; 127] = [Some(1),Some(1),Some(1),Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None,None,Some(1),Some(1),None,Some(1),None,Some(1),None,Some(1),None,Some(1),None];

        let root = TreeNode::from_leetcode_slice(&INPUT);
        assert_eq!(Solution::width_of_binary_tree(root), 2147483645);
    }

    #[test]
    fn discussion_case() {
        #[rustfmt::skip]
        const INPUT: [Option<i32>; 194] = [Some(-64),Some(12),Some(18),Some(-4),Some(-53),None,Some(76),None,Some(-51),None,None,Some(-93),Some(3),None,Some(-31),Some(47),None,Some(3),Some(53),Some(-81),Some(33),Some(4),None,Some(-51),Some(-44),Some(-60),Some(11),None,None,None,None,Some(78),None,Some(-35),Some(-64),Some(26),Some(-81),Some(-31),Some(27),Some(60),Some(74),None,None,Some(8),Some(-38),Some(47),Some(12),Some(-24),None,Some(-59),Some(-49),Some(-11),Some(-51),Some(67),None,None,None,None,None,None,None,Some(-67),None,Some(-37),Some(-19),Some(10),Some(-55),Some(72),None,None,None,Some(-70),Some(17),Some(-4),None,None,None,None,None,None,None,Some(3),Some(80),Some(44),Some(-88),Some(-91),None,Some(48),Some(-90),Some(-30),None,None,Some(90),Some(-34),Some(37),None,None,Some(73),Some(-38),Some(-31),Some(-85),Some(-31),Some(-96),None,None,Some(-18),Some(67),Some(34),Some(72),None,Some(-17),Some(-77),None,Some(56),Some(-65),Some(-88),Some(-53),None,None,None,Some(-33),Some(86),None,Some(81),Some(-42),None,None,Some(98),Some(-40),Some(70),Some(-26),Some(24),None,None,None,None,Some(92),Some(72),Some(-27),None,None,None,None,None,None,Some(-67),None,None,None,None,None,None,None,Some(-54),Some(-66),Some(-36),None,Some(-72),None,None,Some(43),None,None,None,Some(-92),Some(-1),Some(-98),None,None,None,None,None,None,None,Some(39),Some(-84),None,None,None,None,None,None,None,None,None,None,None,None,None,Some(-93),None,None,None,Some(98)];

        let root = TreeNode::from_leetcode_slice(&INPUT);
        assert_eq!(Solution::width_of_binary_tree(root), 169);
    }
}
