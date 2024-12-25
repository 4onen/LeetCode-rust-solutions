// https://leetcode.com/problems/find-largest-value-in-each-tree-row/

use utils::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let Some(root) = root else { return vec![] };
        let mut nodes = vec![root];
        let mut next_nodes = vec![];
        let mut answer = vec![];
        loop {
            let mut loop_max = i32::MIN;
            for node in nodes.drain(..) {
                let node = Rc::into_inner(node).unwrap().into_inner();
                if node.val > loop_max {
                    loop_max = node.val;
                }
                if let Some(lnode) = node.left {
                    next_nodes.push(lnode);
                }
                if let Some(rnode) = node.right {
                    next_nodes.push(rnode);
                }
            }
            answer.push(loop_max);
            std::mem::swap(&mut nodes, &mut next_nodes);
            if nodes.len() < 1 {
                break;
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &[Option<i32>], expected: &[i32]) {
        let mut count = 0;
        for &m in input {
            count += m.is_some() as u16;
        }
        assert!(count <= 10_000);
        let tree = TreeNode::from_leetcode_slice(input);
        assert_eq!(Solution::largest_values(tree), expected);
    }

    #[test]
    fn ex1() {
        let null = None;
        test(
            &[Some(1), Some(3), Some(2), Some(5), Some(3), null, Some(9)],
            &[1, 3, 9],
        )
    }

    #[test]
    fn ex2() {
        test(&[Some(1), Some(2), Some(3)], &[1, 3])
    }

    #[test]
    fn discussion_case1() {
        test(&[], &[])
    }

    #[test]
    fn discussion_case2() {
        let null = None;
        test(
            &[Some(100), Some(-100), null, Some(-2000)],
            &[100, -100, -2000],
        )
    }

    #[test]
    fn discussion_case3() {
        let null = None;
        test(
            &[
                Some(-23351),
                Some(-99006),
                null,
                Some(-63508),
                Some(86343),
                Some(-66012),
                Some(54651),
                Some(-38287),
                Some(2762),
                Some(39604),
                null,
                Some(-33734),
                Some(2587),
                Some(66356),
                Some(-99032),
                Some(50112),
                Some(37759),
                Some(59078),
                Some(-89953),
                Some(44359),
                Some(-73181),
                Some(-18323),
            ],
            &[-23351, -99006, 86343, 54651, 66356, 59078],
        )
    }
}
