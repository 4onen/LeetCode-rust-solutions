// https://leetcode.com/problems/reverse-linked-list-ii/description/

use super::utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut head = head;

        let mut traversed = 1;
        let reversal_start = {
            let mut ptr = &mut head;
            while traversed < left {
                traversed += 1;
                ptr = &mut ptr.as_mut().unwrap().next;
            }
            ptr
        };

        // Unsafe impl:
        if right > left {
            let first = Box::into_raw(reversal_start.take().unwrap());
            let mut prev = unsafe { Box::from_raw(first) };
            let mut next = prev.next.take();
            while traversed < right {
                traversed += 1;
                let mut boxed = next.unwrap();
                next = boxed.next.replace(prev);
                prev = boxed;
            }
            reversal_start.replace(prev);
            if let Some(boxed) = next {
                unsafe { (*first).next.replace(boxed) };
            }
        }

        head
    }
}

#[cfg(test)]
mod test {
    use super::{ListNode, Solution};

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::reverse_between(ListNode::from_slice(&[1, 2, 3, 4, 5]), 2, 4),
            ListNode::from_slice(&[1, 4, 3, 2, 5])
        );
    }

    #[test]
    fn ex1_var1() {
        assert_eq!(
            Solution::reverse_between(ListNode::from_slice(&[1, 2, 3, 4, 5]), 1, 4),
            ListNode::from_slice(&[4, 3, 2, 1, 5])
        );
    }

    #[test]
    fn ex1_var2() {
        assert_eq!(
            Solution::reverse_between(ListNode::from_slice(&[1, 2, 3, 4, 5]), 2, 5),
            ListNode::from_slice(&[1, 5, 4, 3, 2])
        );
    }

    #[test]
    fn ex1_var3() {
        assert_eq!(
            Solution::reverse_between(ListNode::from_slice(&[1, 2, 3, 4, 5]), 5, 5),
            ListNode::from_slice(&[1, 2, 3, 4, 5])
        );
    }

    #[test]
    fn ex1_var4() {
        assert_eq!(
            Solution::reverse_between(ListNode::from_slice(&[1, 2, 3, 4, 5]), 4, 5),
            ListNode::from_slice(&[1, 2, 3, 5, 4])
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::reverse_between(ListNode::from_slice(&[5]), 1, 1),
            ListNode::from_slice(&[5])
        );
    }
}
