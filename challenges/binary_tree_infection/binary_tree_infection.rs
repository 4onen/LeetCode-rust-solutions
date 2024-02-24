// https://leetcode.com/problems/amount-of-time-for-binary-tree-to-be-infected/

use utils::TreeNode;

pub struct Solution;

// At each node, we're interested in a few cases
// Case 1: Neither of my children has the start value, so I'd infect first.
//     The answer is the max of my children's answers plus one (me).
// Case 2: One of my children has the start value, so I'd pass it to my other.
//     I need to pass up two answers: the start's answer, and the other child's answer plus my distance from the start plus one (me).
// Case 3: I am the start node.
//     The answer is the max of my children's answers.

// We can use a recursive function to calculate the answer for each node.
// We just need a way to store the state of our analysis.

#[derive(Debug, Clone, Copy)]
enum State {
    NoInfectionBelow {
        height_below: usize,
    },
    InfectionBelow {
        height_below_infection: usize,
        distance_from_infection: usize,
        otherchild_infection_time: usize,
    },
}

impl State {
    fn merge(&self, other: &Self) -> Self {
        match (*self, *other) {
            (
                Self::NoInfectionBelow { height_below },
                Self::NoInfectionBelow {
                    height_below: other_height_below,
                },
            ) => Self::NoInfectionBelow {
                height_below: 1 + height_below.max(other_height_below),
            },
            (
                Self::NoInfectionBelow { height_below },
                Self::InfectionBelow {
                    height_below_infection,
                    distance_from_infection,
                    otherchild_infection_time,
                },
            ) => Self::InfectionBelow {
                height_below_infection: height_below_infection,
                distance_from_infection: distance_from_infection + 1,
                otherchild_infection_time: otherchild_infection_time
                    .max(height_below + distance_from_infection + 1),
            },
            (
                Self::InfectionBelow {
                    height_below_infection,
                    distance_from_infection,
                    otherchild_infection_time,
                },
                Self::NoInfectionBelow { height_below },
            ) => Self::InfectionBelow {
                height_below_infection: height_below_infection,
                distance_from_infection: distance_from_infection + 1,
                otherchild_infection_time: otherchild_infection_time
                    .max(height_below + distance_from_infection + 1),
            },
            (Self::InfectionBelow { .. }, Self::InfectionBelow { .. }) => {
                unreachable!("Only one infection start can exist.")
            }
        }
    }
}

fn max_depth(node: &TreeNode) -> usize {
    let left_depth = node.left.as_ref().map_or(0, |n| max_depth(&n.borrow()));
    let right_depth = node.right.as_ref().map_or(0, |n| max_depth(&n.borrow()));
    1 + left_depth.max(right_depth)
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let root = root.unwrap();
        let root_state = Self::recurse(&root.borrow(), start);

        match root_state {
            State::NoInfectionBelow { .. } => {
                unreachable!("The infection must reach the root node.")
            }
            State::InfectionBelow {
                height_below_infection,
                otherchild_infection_time,
                ..
            } => height_below_infection.max(otherchild_infection_time) as i32,
        }
    }

    fn recurse(node: &TreeNode, start: i32) -> State {
        if node.val == start {
            let left_depth = node.left.as_ref().map_or(0, |n| max_depth(&n.borrow()));
            let right_depth = node.right.as_ref().map_or(0, |n| max_depth(&n.borrow()));
            State::InfectionBelow {
                height_below_infection: left_depth.max(right_depth),
                distance_from_infection: 0,
                otherchild_infection_time: 0,
            }
        } else {
            let left_state = node
                .left
                .as_ref()
                .map_or(State::NoInfectionBelow { height_below: 0 }, |n| {
                    Self::recurse(&n.borrow(), start)
                });
            let right_state = node
                .right
                .as_ref()
                .map_or(State::NoInfectionBelow { height_below: 0 }, |n| {
                    Self::recurse(&n.borrow(), start)
                });
            left_state.merge(&right_state)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        #[rustfmt::skip]
        const INPUT:&[Option<i32>]=&[Some(1),Some(5),Some(3),None,Some(4),Some(10),Some(6),Some(9),Some(2)];
        assert_eq!(
            Solution::amount_of_time(TreeNode::from_leetcode_slice(INPUT), 3),
            4
        );
    }

    #[test]
    fn ex2() {
        const INPUT: &[Option<i32>] = &[Some(1)];
        assert_eq!(
            Solution::amount_of_time(TreeNode::from_leetcode_slice(INPUT), 1),
            0
        );
    }

    #[test]
    fn myex1_extensions() {
        #[rustfmt::skip]
        const INPUT:&[Option<i32>]=&[Some(1),Some(5),Some(3),None,Some(4),Some(10),Some(6),Some(9),Some(2)];
        assert_eq!(
            Solution::amount_of_time(TreeNode::from_leetcode_slice(INPUT), 6),
            5
        );
        assert_eq!(
            Solution::amount_of_time(TreeNode::from_leetcode_slice(INPUT), 10),
            5
        );
        assert_eq!(
            Solution::amount_of_time(TreeNode::from_leetcode_slice(INPUT), 1),
            3
        );
        assert_eq!(
            Solution::amount_of_time(TreeNode::from_leetcode_slice(INPUT), 5),
            3
        );
        assert_eq!(
            Solution::amount_of_time(TreeNode::from_leetcode_slice(INPUT), 4),
            4
        );
        assert_eq!(
            Solution::amount_of_time(TreeNode::from_leetcode_slice(INPUT), 9),
            5
        );
        assert_eq!(
            Solution::amount_of_time(TreeNode::from_leetcode_slice(INPUT), 2),
            5
        );
    }
}
