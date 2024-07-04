// https://leetcode.com/problems/merge-nodes-in-between-zeros/

use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_nodes(
        head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = head?;
        let mut acc = &mut head;
        let mut next = acc.next.take().expect("No second node!");
        loop {
            match next.val {
                0 => {
                    match next.next.take() {
                        None => {
                            break;
                        }
                        Some(next_next) => {
                            acc.next = Some(next);
                            acc = acc.next.as_mut().expect("Some() we just inserted was None!");
                            next = next_next;
                        }
                    }
                }
                n => {
                    acc.val += n;
                    next = next.next.take().unwrap();
                }
            }
        }
        Some(head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &[i32], expected: &[i32]) {
        assert!(input.len() >= 3);
        assert!(input.len() <= 200_000);
        assert!(expected.len() <= input.len());
        let mut prev = -1;
        for &n in input {
            assert!(n >= 0);
            assert!(n <= 1000);
            if prev == 0 {
                assert_ne!(n, 0);
            }
            prev = n;
        }
        assert_eq!(*input.first().unwrap(), 0);
        assert_eq!(*input.last().unwrap(), 0);
        let head = ListNode::from_slice(input);
        let expected = ListNode::from_slice(expected);
        assert_eq!(Solution::merge_nodes(head), expected);
    }

    #[test]
    fn ex1() {
        test(&[0,3,1,0,4,5,2,0], &[4,11]);
    }

    #[test]
    fn ex2() {
        test(&[0,1,0,3,0,2,2,0], &[1,3,4]);
    }

    #[test]
    fn myex1() {
        test(&[0,1,0,1,1,0,1,0], &[1,2,1]);
    }

    #[test]
    fn myex2() {
        test(&[0,1,0,1,1,0,1,0,1,0], &[1,2,1,1]);
    }

    #[test]
    fn myex3() {
        test(&[0,1,0,1,1,0,1,0,1,1,0], &[1,2,1,2]);
    }

    #[test]
    fn myex4() {
        test(&[0,1,1,1,0,1,1,0,1,0,1,1,0], &[3,2,1,2]);
    }
}
