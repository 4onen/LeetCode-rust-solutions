// https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree/

use utils::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
pub struct FindElements {
    existence_set: std::collections::HashSet<i32>,
}
impl FindElements {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        fn traversal(
            node: Option<Rc<RefCell<TreeNode>>>,
            incoming_val: i32,
            existence_set: &mut std::collections::HashSet<i32>,
        ) {
            let Some(node) = node else { return };
            let node = match std::rc::Rc::<RefCell<TreeNode>>::try_unwrap(node) {
                Ok(node) => node.into_inner(),
                Err(rc) => {
                    let mut inner = rc.borrow_mut();
                    TreeNode {
                        val: 0,
                        left: inner.left.take(),
                        right: inner.right.take(),
                    }
                }
            };
            existence_set.insert(incoming_val);
            traversal(node.left, incoming_val * 2 + 1, existence_set);
            traversal(node.right, incoming_val * 2 + 2, existence_set);
        }
        let mut existence_set = std::collections::HashSet::new();
        traversal(root, 0, &mut existence_set);
        FindElements { existence_set }
    }
    pub fn find(&self, target: i32) -> bool {
        self.existence_set.contains(&target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(initial_tree: &[bool], queries: &[u32], expected: &[bool]) {
        // Check conditions
        let tree_nodes: u16 = initial_tree.iter().map(|&x| x as u16).sum();
        assert!(tree_nodes >= 1);
        assert!(tree_nodes <= 10_000);
        assert_eq!(queries.len(), expected.len());
        assert!(queries.len() >= 1);
        assert!(queries.len() <= 10_000);
        for &query in queries {
            assert!(query <= 1_000_000);
        }
        // Run test
        let tree = TreeNode::from_leetcode_slice(
            &initial_tree
                .iter()
                .map(|&x| if x { Some(-1) } else { None })
                .collect::<Vec<_>>(),
        );
        #[allow(unused_mut)]
        let mut obj = FindElements::new(tree);
        for (&query, &expectation) in std::iter::zip(queries, expected) {
            let ans = obj.find(query as i32);
            if expectation {
                assert!(ans, "{} not found when it should have been", query)
            } else {
                assert!(!ans, "{} found when it shouldn't have been", query)
            }
        }
    }

    #[test]
    fn ex1() {
        // Input
        // ["FindElements","find","find"]
        // [[[-1,null,-1]],[1],[2]]
        // Output
        // [null,false,true]
        test(&[true, false, true], &[1, 2], &[false, true])
    }

    #[test]
    fn ex2() {
        // Input
        // ["FindElements","find","find","find"]
        // [[[-1,-1,-1,-1,-1]],[1],[3],[5]]
        // Output
        // [null,true,true,false]
        test(
            &[true, true, true, true, true],
            &[1, 3, 5],
            &[true, true, false],
        )
    }

    #[test]
    fn ex3() {
        // Input
        // ["FindElements","find","find","find","find"]
        // [[[-1,null,-1,-1,null,-1]],[2],[3],[4],[5]]
        // Output
        // [null,true,false,false,true]
        test(
            &[true, false, true, true, false, true],
            &[2, 3, 4, 5],
            &[true, false, false, true],
        )
    }

    #[test]
    fn discussion_case1() {
        // Input
        // ["FindElements","find","find","find","find","find"]
        // [[[-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1]],[1],[2],[3],[4],[15]]
        // Output
        // [null,true,true,true,true,false]
        test(
            &[
                true, true, true, true, true, true, true, true, true, true, true, true, true, true,
                true,
            ],
            &[1, 2, 3, 4, 15],
            &[true, true, true, true, false],
        )
    }
}
