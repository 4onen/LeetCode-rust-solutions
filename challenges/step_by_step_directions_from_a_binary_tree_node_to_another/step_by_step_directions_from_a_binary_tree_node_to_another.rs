// https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another/

use utils::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn get_directions(
        root: Option<Rc<RefCell<TreeNode>>>,
        start_value: i32,
        dest_value: i32,
    ) -> String {
        let mut directions = std::vec::Vec::new();
        fn push_ups(directions: &mut std::vec::Vec<u8>, count: i32) {
            for _ in 0..count {
                directions.push(b'U');
            }
        }
        #[derive(Debug)]
        enum State {
            StartFound { start_below: i32 },
            DestFound,
            None,
            Done,
        }
        fn dfs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            start_value: i32,
            dest_value: i32,
            directions: &mut std::vec::Vec<u8>,
        ) -> State {
            let Some(root) = root else { return State::None };
            let root = root.borrow();
            let left_state = dfs(&root.left, start_value, dest_value, directions);
            let right_state = dfs(&root.right, start_value, dest_value, directions);
            return match (left_state, right_state) {
                (State::Done, _) => State::Done,
                (_, State::Done) => State::Done,
                (State::StartFound { start_below }, State::None) if root.val == dest_value => {
                    push_ups(directions, start_below);
                    State::Done
                }
                (State::DestFound, State::None) if root.val == start_value => {
                    directions.push(b'L');
                    State::Done
                }
                (State::None, State::StartFound { start_below }) if root.val == dest_value => {
                    push_ups(directions, start_below);
                    State::Done
                }
                (State::None, State::DestFound) if root.val == start_value => {
                    directions.push(b'R');
                    State::Done
                }
                (State::StartFound { start_below }, State::None) => State::StartFound {
                    start_below: start_below + 1,
                },
                (State::None, State::StartFound { start_below }) => State::StartFound {
                    start_below: start_below + 1,
                },
                (State::DestFound, State::None) => {
                    directions.push(b'L');
                    State::DestFound
                }
                (State::None, State::DestFound) => {
                    directions.push(b'R');
                    State::DestFound
                }
                (State::StartFound { start_below }, State::DestFound) => {
                    directions.push(b'R');
                    push_ups(directions, start_below);
                    State::Done
                }
                (State::DestFound, State::StartFound { start_below }) => {
                    directions.push(b'L');
                    push_ups(directions, start_below);
                    State::Done
                }
                (State::None, State::None) if root.val == start_value => {
                    State::StartFound { start_below: 1 }
                }
                (State::None, State::None) if root.val == dest_value => State::DestFound,
                (State::None, State::None) => State::None,
                _ => unreachable!(),
            };
        }
        let d = dfs(&root, start_value, dest_value, &mut directions);
        dbg!(d);
        directions.reverse();
        unsafe { std::string::String::from_utf8_unchecked(directions) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &[Option<i32>], start_value: i32, dest_value: i32, output: &str) {
        let node_count = input.iter().filter(|x| x.is_some()).count() as i32;
        assert!(node_count >= 2);
        assert!(node_count <= 100_000);
        assert!(input.iter().all(|x| match x {
            Some(v) => *v >= 1 && *v <= node_count,
            None => true,
        }));
        assert!(start_value >= 1);
        assert!(start_value <= node_count);
        assert!(dest_value >= 1);
        assert!(dest_value <= node_count);
        assert_ne!(start_value, dest_value);
        assert!(input.iter().filter(|x| x == &&Some(start_value)).count() == 1);
        let tree = TreeNode::from_leetcode_slice(input);
        assert_eq!(
            Solution::get_directions(tree, start_value, dest_value),
            output
        );
    }

    #[test]
    fn ex1() {
        test(
            &[Some(5), Some(1), Some(2), Some(3), None, Some(6), Some(4)],
            3,
            6,
            "UURL",
        )
    }

    #[test]
    fn ex2() {
        test(&[Some(2), Some(1)], 2, 1, "L")
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                Some(1),
                None,
                Some(10),
                Some(12),
                Some(13),
                Some(4),
                Some(6),
                None,
                Some(15),
                None,
                None,
                Some(5),
                Some(11),
                None,
                Some(2),
                Some(14),
                Some(7),
                None,
                Some(8),
                None,
                None,
                None,
                Some(9),
                Some(3),
            ],
            6,
            15,
            "UURR",
        )
    }
    #[test]
    fn discussion_case2() {
        test(
            &[
                Some(1),
                Some(3),
                Some(8),
                Some(7),
                None,
                Some(4),
                Some(5),
                Some(6),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(2),
            ],
            2,
            1,
            "UUUU",
        )
    }
}
