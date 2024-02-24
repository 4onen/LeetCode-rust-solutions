// https://leetcode.com/problems/remove-nth-node-from-end-of-list/

use utils::ListNode;

pub struct Solution;

// What I thought was a clever solution
// impl Solution {
//     pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
//         // This ugly hack lets us borrow head as immutable and mutable at the same time
//         // Safety: we know that `fast` will always be beyond `slow`
//         let mut slow = unsafe { (&mut head as *mut Option<Box<ListNode>>).as_mut() }.unwrap();
//         let mut fast = &head;
//         for _ in 0..n {
//             fast = &fast.as_ref().unwrap().next;
//         }
//         while fast.is_some() {
//             fast = &fast.as_ref().unwrap().next;
//             slow = &mut slow.as_mut().unwrap().next;
//         }
//         let next = slow.as_mut().unwrap().next.take();
//         let _ = std::mem::replace(slow, next);
//         head
//     }
// }

// Count first sol'n
impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut node = &head;
        while let Some(n) = node {
            len += 1;
            node = &n.next;
        }
        let mut prev = &mut head;
        for _ in 0..(len - n as usize) {
            prev = &mut prev.as_mut().unwrap().next;
        }
        let next = prev.as_mut().unwrap().next.take();
        let _ = std::mem::replace(prev, next);
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let output = ListNode::from_slice(&[1, 2, 3, 5]);
        assert_eq!(Solution::remove_nth_from_end(input, 2), output);
    }

    #[test]
    fn ex2() {
        let input = ListNode::from_slice(&[1]);
        let output = ListNode::from_slice(&[]);
        assert_eq!(Solution::remove_nth_from_end(input, 1), output);
    }

    #[test]
    fn ex3() {
        let input = ListNode::from_slice(&[1, 2]);
        let output = ListNode::from_slice(&[1]);
        assert_eq!(Solution::remove_nth_from_end(input, 1), output);
    }

    #[test]
    fn myex1() {
        let input = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let output = ListNode::from_slice(&[1, 2, 3, 4]);
        assert_eq!(Solution::remove_nth_from_end(input, 1), output);
    }

    #[test]
    fn myex2() {
        let input = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let output = ListNode::from_slice(&[1, 3, 4, 5]);
        assert_eq!(Solution::remove_nth_from_end(input, 4), output);
    }

    #[test]
    fn myex3() {
        let input = ListNode::from_slice(&[1, 2, 3, 4, 5]);
        let output = ListNode::from_slice(&[2, 3, 4, 5]);
        assert_eq!(Solution::remove_nth_from_end(input, 5), output);
    }

    #[test]
    fn myex4() {
        let input = ListNode::from_slice(&[1, 2]);
        let output = ListNode::from_slice(&[2]);
        assert_eq!(Solution::remove_nth_from_end(input, 2), output);
    }
}
