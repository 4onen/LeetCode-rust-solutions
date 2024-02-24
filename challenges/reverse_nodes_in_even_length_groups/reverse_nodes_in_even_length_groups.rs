// https://leetcode.com/problems/reverse-nodes-in-even-length-groups/

use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_even_length_groups(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut new_head = None;
        let mut tail = &mut new_head;
        let mut group_start = None;
        let mut group_end = &mut group_start;
        let mut count = 0;
        let mut k = 1;
        while let Some(mut node) = head {
            head = node.next.take();
            count += 1;
            *group_end = Some(node);
            group_end = &mut group_end.as_mut().unwrap().next;
            if count >= k {
                if count & 1 == 0 {
                    let new_tail: *mut Option<Box<ListNode>> =
                        &mut group_start.as_mut().unwrap().next;
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
                } else {
                    let new_tail: *mut Option<Box<ListNode>> = group_end;
                    *tail = group_start.take();
                    tail = unsafe { &mut *new_tail };
                }
                group_start = None;
                group_end = &mut group_start;
                count = 0;
                k += 1;
            }
        }
        *tail = if count & 1 == 0 {
            let mut head = group_start.take();
            let mut prev = None;
            while let Some(mut node) = head {
                head = node.next.take();
                node.next = prev;
                prev = Some(node);
            }
            prev
        } else {
            group_start
        };
        new_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::reverse_even_length_groups(ListNode::from_slice(&[
                5, 2, 6, 3, 9, 1, 7, 3, 8, 4
            ])),
            ListNode::from_slice(&[5, 6, 2, 3, 9, 1, 4, 8, 3, 7])
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::reverse_even_length_groups(ListNode::from_slice(&[1, 1, 0, 6])),
            ListNode::from_slice(&[1, 0, 1, 6])
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::reverse_even_length_groups(ListNode::from_slice(&[1, 1, 0, 6, 5])),
            ListNode::from_slice(&[1, 0, 1, 5, 6])
        );
    }
}
