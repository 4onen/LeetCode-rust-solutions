// https://leetcode.com/problems/linked-list-in-binary-tree/

use utils::TreeNode;
use utils::ListNode;

pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn check_path(node: &Option<Rc<RefCell<TreeNode>>>, list: &Option<Box<ListNode>>) -> bool {
            let Some(list) = list else { return true; };
            let Some(node) = node else { return false; };
            if node.borrow().val == list.val {
                return check_path(&node.borrow().left, &list.next) || check_path(&node.borrow().right, &list.next);
            }
            false
        }
        fn search_for_path_start(node: Option<Rc<RefCell<TreeNode>>>, list: &Option<Box<ListNode>>) -> bool {
            let Some(list_inner) = list else { return true; };
            if node.as_deref().map_or(false,|n| n.borrow().val == list_inner.val) {
                if check_path(&node, list) {
                    return true;
                }
            }
            let Some(node) = node else { return false; };
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            search_for_path_start(left, list) || search_for_path_start(right, list)
        }
        search_for_path_start(root, &head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(list: &[i32], tree: &[Option<i32>], expected: bool) {
        assert!(list.len() > 0);
        assert!(list.len() <= 100);
        assert!(tree.len() > 0);
        for i in 0..list.len() {
            assert!(list[i] >= 1);
            assert!(list[i] <= 100);
        }
        let mut tree_nodes = 0;
        for i in 0..tree.len() {
            if let Some(val) = tree[i] {
                tree_nodes += 1;
                assert!(val >= 1);
                assert!(val <= 100);
            }
        }
        assert!(tree_nodes > 0);
        assert!(tree_nodes <= 2500);
        let head = ListNode::from_slice(list);
        let root = TreeNode::from_leetcode_slice(tree);
        assert_eq!(Solution::is_sub_path(head, root), expected);
    }

    #[test]
    fn ex1() {
        test(
            &[4,2,8],
            &[Some(1),Some(4),Some(4),None,Some(2),Some(2),None,Some(1),None,Some(6),Some(8),None,None,None,None,Some(1),Some(3)],
            true
        )
    }

    #[test]
    fn ex2() {
        test(
            &[1,4,2,6],
            &[Some(1),Some(4),Some(4),None,Some(2),Some(2),None,Some(1),None,Some(6),Some(8),None,None,None,None,Some(1),Some(3)],
            true
        )
    }

    #[test]
    fn ex3() {
        test(
            &[1,4,2,6,8],
            &[Some(1),Some(4),Some(4),None,Some(2),Some(2),None,Some(1),None,Some(6),Some(8),None,None,None,None,Some(1),Some(3)],
            false
        )
    }
}
