// https://leetcode.com/problems/double-a-number-represented-as-a-linked-list/

use utils::ListNode;

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         fn reverse(mut old_head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//             let mut new_head = None;
//             while let Some(mut node) = old_head {
//                 old_head = node.next.take();
//                 node.next = new_head;
//                 new_head = Some(node);
//             }
//             new_head
//         }
//         let mut head = reverse(head);
//         let mut carry = 0;
//         let mut new_head = None;
//         while let Some(mut node) = head {
//             let mut val = node.val * 2 + carry as i32;
//             carry = (val / 10) as u8;
//             val %= 10;
//             node.val = val;
//             head = node.next.take();
//             node.next = new_head;
//             new_head = Some(node);
//         }
//         if carry > 0 {
//             let mut node = Box::new(ListNode::new(carry as i32));
//             node.next = new_head;
//             new_head = Some(node);
//         }
//         new_head
//     }
// }

// Impossible sol'n from best
impl Solution {
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = ListNode { val: 0, next: head };
        let mut prev = &mut head;
        while let Some(digit) = prev.next.as_mut() {
            let double = digit.val * 2;
            digit.val = double % 10;
            prev.val += double / 10;
            prev = digit;
        }
        if head.val == 0 {
            head.next
        } else {
            Some(Box::new(head))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn linkedlist_from_u8_slice(slice: &[u8]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &x in slice.iter().rev() {
            let mut node = Box::new(ListNode::new(x as i32));
            node.next = head;
            head = Some(node);
        }
        head
    }

    fn test(input: &[u8], expected: &[u8]) {
        let input = linkedlist_from_u8_slice(input);
        let expected = linkedlist_from_u8_slice(expected);
        assert_eq!(Solution::double_it(input), expected);
    }

    #[test]
    fn ex1() {
        test(&[1, 8, 9], &[3, 7, 8])
    }

    #[test]
    fn ex2() {
        test(&[9, 9, 9], &[1, 9, 9, 8])
    }

    #[test]
    fn myex1() {
        test(&[1, 2, 3], &[2, 4, 6])
    }

    #[test]
    fn discussion_case0() {
        test(&[0], &[0])
    }

    #[test]
    fn discussion_case9() {
        test(&[9], &[1, 8])
    }

    #[test]
    fn myex99() {
        test(&[9, 9], &[1, 9, 8])
    }

    #[test]
    fn discussion_case1() {
        test(&[1], &[2])
    }

    #[test]
    fn discussion_case5() {
        test(&[5], &[1, 0])
    }

    #[test]
    fn discussion_case_nines() {
        test(
            &[9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
            &[
                1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 8,
            ],
        )
    }
}
