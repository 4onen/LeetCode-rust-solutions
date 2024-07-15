// https://leetcode.com/problems/create-binary-tree-from-descriptions/

use utils::TreeNode;

pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        type Map = std::collections::HashMap<i32,[i32;2]>;
        let mut child_map = Map::new();
        let mut seen_children = std::collections::HashSet::<i32>::with_capacity(descriptions.len());
        let mut seen_parents = std::collections::HashSet::<i32>::with_capacity(descriptions.len());
        for d in descriptions {
            let [parent, child, isleft] = [d[0],d[1],d[2]];
            seen_children.insert(child);
            seen_parents.insert(parent);
            assert!(isleft == 0 || isleft == 1);
            child_map.entry(parent).or_insert([0;2])[isleft as usize] = child;
        }
        let root_node: i32 = *seen_parents.difference(&seen_children).next().unwrap();
        fn build(node: i32, map: &Map) -> Option<Rc<RefCell<TreeNode>>> {
            if node < 1 {
                None
            } else {
                Some(Rc::new(RefCell::new(match map.get(&node) {
                    None => {
                        TreeNode {
                            val: node,
                            left: None,
                            right: None,
                        }
                    }
                    Some(&[right,left]) => {
                        TreeNode {
                            val: node,
                            left: build(left, map),
                            right: build(right, map),
                        }
                    }
                })))
            }
        }
        build(root_node, &child_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &[[i32; 3]], expected: &[Option<i32>]) {
        assert!(input.len() >= 1);
        assert!(input.len() <= 10_000);
        for &[parent,child,isleft] in input {
            assert!(parent >= 1);
            assert!(parent <= 100_000);
            assert!(child >= 1);
            assert!(child <= 100_000);
            assert_ne!(parent, child);
            assert!(isleft == 0 || isleft == 1);
        }
        let input: Vec<_> = input.iter().map(|x| x.to_vec()).collect();
        assert_eq!(Solution::create_binary_tree(input), TreeNode::from_leetcode_slice(expected));
    }

    #[test]
    fn ex1() {
        test(&[[20,15,1],[20,17,0],[50,20,1],[50,80,0],[80,19,1]],
            &[Some(50),Some(20),Some(80),Some(15),Some(17),Some(19)]);
    }

    #[test]
    fn ex2() {
        test(
            &[[1,2,1],[2,3,0],[3,4,1]],
            &[Some(1),Some(2),None,None,Some(3),Some(4)]
        )
    }

    #[test]
    fn discussion_case1() {
        test(
            &[[100000,99999,1]],
            &[Some(100000),Some(99999)]
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[[2,15,0],[80,19,0],[19,5,1],[2,17,1],[50,2,0],[50,80,1]],
            &[Some(50),Some(80),Some(2),None,Some(19),Some(17),Some(15),Some(5)]
        )
    }

    #[test]
    fn discussion_case3() {
        test(
            &[[80,19,1],[1,2,1],[2,3,0],[3,4,1],[20,15,1],[20,17,0],[4,50,0],[50,20,1],[50,80,0]],
            &[Some(1),Some(2),None,None,Some(3),Some(4),None,None,Some(50),Some(20),Some(80),Some(15),Some(17),Some(19)]
        )
    }

    #[test]
    fn discussion_case4() {
        test(
            &[[39,70,1],[13,39,1],[85,74,1],[74,13,1],[38,82,1],[82,85,1]],
            &[Some(38),Some(82),None,Some(85),None,Some(74),None,Some(13),None,Some(39),None,Some(70)]
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            &[[85,74,0],[38,82,0],[39,70,0],[82,85,0],[74,13,0],[13,39,0]],
            &[Some(38),None,Some(82),None,Some(85),None,Some(74),None,Some(13),None,Some(39),None,Some(70)]
        )
    }

    #[test]
    fn discussion_case6() {
        test(
            &[[85,82,1],[74,85,1],[39,70,0],[82,38,1],[74,39,0],[39,13,1]],
            &[Some(74),Some(85),Some(39),Some(82),None,Some(13),Some(70),Some(38)]
        )
    }
}
