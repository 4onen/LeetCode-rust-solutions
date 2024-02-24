// https://leetcode.com/problems/remove-duplicates-from-sorted-list/

use utils::ListNode;

pub struct Solution;

// impl Solution {
//     pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         if let Some(mut head_node) = head {
//             let mut current_node = head_node.as_mut();
//             while let Some(next_node) = current_node.next.as_deref_mut() {
//                 if current_node.val == next_node.val {
//                     current_node.next = next_node.next.take();
//                 } else {
//                     current_node = current_node.next.as_deref_mut().unwrap();
//                 }
//             }
//             Some(head_node)
//         } else {
//             None
//         }
//     }
// }

// Nested loop
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current_node = &mut head;
        while let Some(ref mut node) = current_node {
            while node.next.is_some() && node.val == node.next.as_ref().unwrap().val {
                node.next = node.next.as_mut().unwrap().next.take();
            }
            current_node = &mut node.next;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = ListNode::from_slice(&[1, 1, 2]);
        let output = ListNode::from_slice(&[1, 2]);
        assert_eq!(Solution::delete_duplicates(input), output);
    }

    #[test]
    fn ex2() {
        let input = ListNode::from_slice(&[1, 1, 2, 3, 3]);
        let output = ListNode::from_slice(&[1, 2, 3]);
        assert_eq!(Solution::delete_duplicates(input), output);
    }

    #[test]
    fn myex1() {
        let input = ListNode::from_slice(&[1, 1, 1, 1, 1]);
        let output = ListNode::from_slice(&[1]);
        assert_eq!(Solution::delete_duplicates(input), output);
    }

    #[test]
    fn myex2() {
        let input = ListNode::from_slice(&[1, 1, 1, 2, 2, 3, 3, 3, 3, 3]);
        let output = ListNode::from_slice(&[1, 2, 3]);
        assert_eq!(Solution::delete_duplicates(input), output);
    }

    #[test]
    fn myex3() {
        let input = ListNode::from_slice(&[]);
        let output = ListNode::from_slice(&[]);
        assert_eq!(Solution::delete_duplicates(input), output);
    }
}
