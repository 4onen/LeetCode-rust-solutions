// https://leetcode.com/problems/add-two-numbers/

use utils::ListNode;

pub struct Solution;

// Braindead reallocating sol'n
// impl Solution {
//     pub fn add_two_numbers(
//         mut l1: Option<Box<ListNode>>,
//         mut l2: Option<Box<ListNode>>,
//     ) -> Option<Box<ListNode>> {
//         let mut head = Some(Box::new(ListNode::new(0)));
//         let mut tail = &mut head;
//         let mut carry = 0;
//         while l1.is_some() || l2.is_some() || carry > 0 {
//             let mut sum = carry;
//             if let Some(node) = l1 {
//                 sum += node.val;
//                 l1 = node.next;
//             }
//             if let Some(node) = l2 {
//                 sum += node.val;
//                 l2 = node.next;
//             }
//             carry = sum / 10;
//             tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
//             tail = &mut tail.as_mut().unwrap().next;
//         }
//         head.unwrap().next
//     }
// }

// Node stealing sol'n
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut head;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry > 0 {
            let mut sum = carry;
            let mut spare_node = None;
            if let Some(mut node) = l1.take() {
                sum += node.val;
                l1 = node.next.take();
                spare_node = Some(node);
            }
            if let Some(mut node) = l2.take() {
                sum += node.val;
                l2 = node.next.take();
                spare_node = Some(node);
            }
            carry = sum / 10;
            if let Some(mut node) = spare_node {
                node.val = sum % 10;
                tail.as_mut().unwrap().next = Some(node);
            } else {
                tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
            }
            tail = &mut tail.as_mut().unwrap().next;
        }
        head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let l1 = ListNode::from_slice(&[2, 4, 3]);
        let l2 = ListNode::from_slice(&[5, 6, 4]);
        let expected = ListNode::from_slice(&[7, 0, 8]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn ex2() {
        let l1 = ListNode::from_slice(&[0]);
        let l2 = ListNode::from_slice(&[0]);
        let expected = ListNode::from_slice(&[0]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn ex3() {
        let l1 = ListNode::from_slice(&[9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from_slice(&[9, 9, 9, 9]);
        let expected = ListNode::from_slice(&[8, 9, 9, 9, 0, 0, 0, 1]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }
}
