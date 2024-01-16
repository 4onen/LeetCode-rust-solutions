// https://leetcode.com/problems/leaf-similar-trees/

use super::utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution;

// Braindead sol'n
// impl Solution {
//     fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, leaves: &mut Vec<i32>) {
//         if let Some(node) = root {
//             let node = node.borrow();
//             if node.left.is_none() && node.right.is_none() {
//                 leaves.push(node.val);
//             } else {
//                 Solution::dfs(&node.left, leaves);
//                 Solution::dfs(&node.right, leaves);
//             }
//         }
//     }
//     pub fn leaf_similar(
//         root1: Option<Rc<RefCell<TreeNode>>>,
//         root2: Option<Rc<RefCell<TreeNode>>>,
//     ) -> bool {
//         let mut leaves1 = Vec::new();
//         let mut leaves2 = Vec::new();
//         Solution::dfs(&root1, &mut leaves1);
//         Solution::dfs(&root2, &mut leaves2);
//         leaves1 == leaves2
//     }
// }

// Box dyn Iterator sol'n
// impl Solution {
//     fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> Box<dyn Iterator<Item = i32>> {
//         if let Some(node) = root {
//             let node = node.borrow();
//             if node.left.is_none() && node.right.is_none() {
//                 return Box::new(std::iter::once(node.val));
//             } else {
//                 return Box::new(Solution::dfs(&node.left).chain(Solution::dfs(&node.right)));
//             }
//         }
//         Box::new(std::iter::empty())
//     }
//     pub fn leaf_similar(
//         root1: Option<Rc<RefCell<TreeNode>>>,
//         root2: Option<Rc<RefCell<TreeNode>>>,
//     ) -> bool {
//         Solution::dfs(&root1).eq(Solution::dfs(&root2))
//     }
// }

// Binary Tree Leaf Iterator sol'n
impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        struct BinaryTreeLeafIntoIterator {
            stack: Vec<Rc<RefCell<TreeNode>>>,
        }
        impl BinaryTreeLeafIntoIterator {
            fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
                if let Some(node) = root {
                    Self { stack: vec![node] }
                } else {
                    Self { stack: vec![] }
                }
            }
        }
        impl Iterator for BinaryTreeLeafIntoIterator {
            type Item = i32;
            fn next(&mut self) -> Option<Self::Item> {
                while let Some(node) = self.stack.pop() {
                    let node = node.borrow();
                    match (&node.left, &node.right) {
                        (None, None) => return Some(node.val),
                        (Some(left), None) => self.stack.push(left.clone()),
                        (None, Some(right)) => self.stack.push(right.clone()),
                        (Some(left), Some(right)) => {
                            self.stack.push(left.clone());
                            self.stack.push(right.clone());
                        }
                    }
                }
                None
            }
        }
        BinaryTreeLeafIntoIterator::new(root1).eq(BinaryTreeLeafIntoIterator::new(root2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex0() {
        let root = TreeNode::from_leetcode_slice(&[
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(9),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        assert_eq!(Solution::leaf_similar(root.clone(), root), true);
    }

    #[test]
    fn ex1() {
        let rootleft = TreeNode::from_leetcode_slice(&[
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(9),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let rootright = TreeNode::from_leetcode_slice(&[
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(7),
            Some(4),
            Some(2),
            None,
            None,
            None,
            None,
            None,
            None,
            Some(9),
            Some(8),
        ]);
        assert_eq!(Solution::leaf_similar(rootleft, rootright), true);
    }

    #[test]
    fn ex2() {
        let rootleft = TreeNode::from_leetcode_slice(&[Some(1), Some(2), Some(3)]);
        let rootright = TreeNode::from_leetcode_slice(&[Some(7), Some(3), Some(2)]);
        assert_eq!(Solution::leaf_similar(rootleft, rootright), false);
    }

    #[test]
    fn myex1() {
        let rootleft = TreeNode::from_leetcode_slice(&[Some(1), Some(2), Some(3)]);
        let rootright = TreeNode::from_leetcode_slice(&[Some(7), Some(2), Some(3)]);
        assert_eq!(Solution::leaf_similar(rootleft, rootright), true);
    }

    #[test]
    fn myex2() {
        let rootleft = TreeNode::from_leetcode_slice(&[Some(1), Some(2), None]);
        let rootright = TreeNode::from_leetcode_slice(&[Some(7), Some(3), None]);
        assert_eq!(Solution::leaf_similar(rootleft, rootright), false);
    }

    #[test]
    fn myex3() {
        let rootleft = TreeNode::from_leetcode_slice(&[Some(1), Some(2), None]);
        let rootright = TreeNode::from_leetcode_slice(&[Some(7), Some(2), None]);
        assert_eq!(Solution::leaf_similar(rootleft, rootright), true);
    }
}
