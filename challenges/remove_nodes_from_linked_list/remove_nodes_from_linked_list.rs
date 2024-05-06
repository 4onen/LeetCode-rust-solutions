// https://leetcode.com/problems/remove-nodes-from-linked-list/

use utils::ListNode;

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn remove_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         let mut stack: Vec<Box<ListNode>> = Vec::new();
//         while let Some(mut node) = head {
//             head = node.next.take();
//             while !stack.is_empty() && stack.last().unwrap().val < node.val {
//                 stack.pop();
//             }
//             stack.push(node);
//         }
//         for mut node in stack.into_iter().rev() {
//             node.next = head;
//             head = Some(node);
//         }
//         head
//     }
// }

// Optimized sol'n using list reversal
impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn reverse(mut old_head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut new_head = None;
            while let Some(mut node) = old_head {
                old_head = node.next.take();
                node.next = new_head;
                new_head = Some(node);
            }
            new_head
        }
        let mut reversed = reverse(head);
        let mut ptr = reversed.as_mut().unwrap();
        loop {
            if ptr.next.is_none() {
                break;
            }
            if ptr.val > ptr.next.as_ref().unwrap().val {
                ptr.next = ptr.next.as_mut().unwrap().next.take();
            } else {
                ptr = ptr.next.as_mut().unwrap();
            }
        }
        reverse(reversed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &[i32], expected: &[i32]) {
        let head = ListNode::from_slice(input);
        let expected = ListNode::from_slice(expected);
        assert_eq!(Solution::remove_nodes(head), expected);
    }

    #[test]
    fn ex1() {
        test(&[5, 2, 13, 3, 8], &[13, 8]);
    }

    #[test]
    fn ex2() {
        test(&[1, 1, 1, 1], &[1, 1, 1, 1]);
    }
}
