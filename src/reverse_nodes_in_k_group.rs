// https://leetcode.com/problems/reverse-nodes-in-k-group/
// Next: https://leetcode.com/problems/reverse-nodes-in-even-length-groups/

use crate::utils::ListNode;

pub struct Solution;

// Stack sol'n
// impl Solution {
//     pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
//         let k = k as usize;
//         if k <= 1 || head.is_none() {
//             return head;
//         }
//         let mut dummy = None;
//         let mut tail = &mut dummy;
//         let mut stack = Vec::new();
//         while let Some(mut node) = head {
//             head = node.next.take();
//             stack.push(node);
//             if stack.len() == k {
//                 while let Some(node) = stack.pop() {
//                     *tail = Some(node);
//                     tail = &mut tail.as_mut().unwrap().next;
//                 }
//             }
//         }
//         if stack.len() > 0 {
//             stack.reverse();
//             while let Some(node) = stack.pop() {
//                 *tail = Some(node);
//                 tail = &mut tail.as_mut().unwrap().next;
//             }
//         }
//         dummy
//     }
// }

// Constant space sol'n
// impl Solution {
//     pub fn reverse_in_place(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         let mut prev = None;
//         while let Some(mut node) = head {
//             head = node.next.take();
//             node.next = prev;
//             prev = Some(node);
//         }
//         prev
//     }
//     pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
//         let k = k as usize;
//         if k <= 1 || head.is_none() {
//             return head;
//         }
//         let mut new_head = None;
//         let mut tail = &mut new_head;
//         let mut group_start = None;
//         let mut group_end = &mut group_start;
//         let mut count = 0;
//         while let Some(mut node) = head {
//             head = node.next.take();
//             count += 1;
//             *group_end = Some(node);
//             group_end = &mut group_end.as_mut().unwrap().next;
//             if count >= k {
//                 count = 0;
//                 *tail = Self::reverse_in_place(group_start.take());
//                 while tail.is_some() {
//                     tail = &mut tail.as_mut().unwrap().next;
//                 }
//                 group_end = &mut group_start;
//             }
//         }
//         *tail = group_start;
//         new_head
//     }
// }

// Unsafe constant space sol'n
impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let k = k as usize;
        if k <= 1 || head.is_none() {
            return head;
        }
        let mut new_head = None;
        let mut tail = &mut new_head;
        let mut group_start = None;
        let mut group_end = &mut group_start;
        let mut count = 0;
        while let Some(mut node) = head {
            head = node.next.take();
            count += 1;
            *group_end = Some(node);
            if count < k {
                group_end = &mut group_end.as_mut().unwrap().next;
            } else {
                count = 0;
                let new_tail: *mut Option<Box<ListNode>> = &mut group_start.as_mut().unwrap().next;
                *tail = {
                    let mut head = group_start.take();
                    let mut prev = None;
                    while let Some(mut node) = head {
                        head = node.next.take();
                        node.next = prev;
                        prev = Some(node);
                    }
                    prev
                };
                // Safety: `new_tail` is a pointer to the `next` field of the
                // former first node in the group, which is now the last node
                // in the group. We are guaranteed that this pointer is valid
                // so long as nothing has moved the ListNode out of the box.
                // Pretty sure I have to Pin the ListNode to prevent this,
                // but it doesn't implement Unpin and this hasn't broken yet.
                tail = unsafe { &mut *new_tail };
                group_start = None;
                group_end = &mut group_start;
            }
        }
        *tail = group_start;
        new_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let expected = ListNode::from_slice(&[2, 1, 4, 3, 5]);
        assert_eq!(Solution::reverse_k_group(head, 2), expected);
    }

    #[test]
    fn ex2() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let expected = ListNode::from_slice(&[3, 2, 1, 4, 5]);
        assert_eq!(Solution::reverse_k_group(head, 3), expected);
    }

    #[test]
    fn myex1() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let expected = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        assert_eq!(Solution::reverse_k_group(head, 1), expected);
    }

    #[test]
    fn discussion_case1() {
        let head = ListNode::from_slice(&[1]);
        let expected = ListNode::from_slice(&[1]);
        assert_eq!(Solution::reverse_k_group(head, 2), expected);
    }

    #[test]
    fn discussion_case2() {
        let head = ListNode::from_slice(&[1, 2]);
        let expected = ListNode::from_slice(&[1, 2]);
        assert_eq!(Solution::reverse_k_group(head, 3), expected);
    }

    #[test]
    fn discussion_case3() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let expected = ListNode::from_slice(&[4, 3, 2, 1, 5]);
        assert_eq!(Solution::reverse_k_group(head, 4), expected);
    }

    #[test]
    fn discussion_case4() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let expected = ListNode::from_slice(&[5, 4, 3, 2, 1, 10, 9, 8, 7, 6]);
        assert_eq!(Solution::reverse_k_group(head, 5), expected);
    }
}
