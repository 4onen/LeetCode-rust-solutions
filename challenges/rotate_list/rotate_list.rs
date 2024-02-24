// https://leetcode.com/problems/rotate-list/

use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        head.map(|head_node| {
            let head_node: *mut ListNode = Box::leak(head_node);
            let mut tail_node: &mut ListNode = unsafe { &mut *head_node };
            let mut len = 1;
            while tail_node.next.is_some() {
                tail_node = tail_node.next.as_mut().unwrap();
                len += 1;
            }
            tail_node.next = Some(unsafe { Box::from_raw(head_node) });
            match k % len {
                0 => {}
                k => {
                    for _ in 0..len - k {
                        tail_node = tail_node.next.as_mut().unwrap();
                    }
                }
            }
            tail_node.next.take().unwrap()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let k = 2;
        let expected = ListNode::from_slice(&[4, 5, 1, 2, 3]);
        assert_eq!(Solution::rotate_right(head, k), expected);
    }

    #[test]
    fn ex2() {
        let head = ListNode::from_slice(&[0, 1, 2]);
        let k = 4;
        let expected = ListNode::from_slice(&[2, 0, 1]);
        assert_eq!(Solution::rotate_right(head, k), expected);
    }
}
