// https://leetcode.com/problems/reorder-list/

use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        fn split_list(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            // Safety: The fact head was passed to us as a mutable reference
            // means that it is not aliased, so only our changes can affect
            // its allocation. The only change we do to it is to take ownership
            // of the list node that is the middle of the list,
            // after `fast` has traversed the list twice as fast as `slow`.
            let mut fast = unsafe { &*(head as *mut Option<Box<ListNode>>) };
            let mut slow = &mut *head;
            while let Some(fast_node) = fast {
                fast = &fast_node.next;
                if let Some(fast_node) = fast {
                    fast = &fast_node.next;
                    slow = &mut slow.as_mut().unwrap().next;
                }
            }
            slow.as_mut().unwrap().next.take()
        }
        fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut prev = None;
            let mut curr = head;
            while let Some(mut curr_node) = curr {
                let next = std::mem::replace(&mut curr_node.next, prev);
                prev = Some(curr_node);
                curr = next;
            }
            prev
        }
        let mut source = reverse_list(split_list(head));
        let mut dest = &mut head.as_mut().unwrap().next;
        while let Some(source_node) = source {
            source = dest.replace(source_node);
            dest = &mut dest.as_mut().unwrap().next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(input: &[i32]) {
        let mut expected = std::vec::Vec::with_capacity(input.len());
        let mut input_iter = input.iter();
        loop {
            let Some(val) = input_iter.next() else {
                break;
            };
            expected.push(*val);
            let Some(val) = input_iter.next_back() else {
                break;
            };
            expected.push(*val);
        }
        let mut head = ListNode::from_slice(input);
        Solution::reorder_list(&mut head);
        assert_eq!(head, ListNode::from_slice(&expected));
    }

    #[test]
    fn ex1() {
        run_test(&[1, 2, 3, 4]);
    }

    #[test]
    fn ex2() {
        run_test(&[1, 2, 3, 4, 5]);
    }

    #[test]
    fn myex10() {
        run_test(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn myex1() {
        run_test(&[1]);
    }

    #[test]
    fn myex2() {
        run_test(&[1, 2]);
    }
}
