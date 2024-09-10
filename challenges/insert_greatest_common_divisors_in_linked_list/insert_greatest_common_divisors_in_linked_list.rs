// https://leetcode.com/problems/insert-greatest-common-divisors-in-linked-list/

use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn insert_greatest_common_divisors(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        const fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            a
        }
        let mut curr = &mut head;
        loop {
            let next = match curr.as_deref_mut() {
                Some(n) if n.next.is_some() => n.next.take(),
                _ => break head,
            };
            let gcd = gcd(curr.as_ref().unwrap().val, next.as_ref().unwrap().val);
            let node = Box::new(ListNode { next, val: gcd });
            curr.as_deref_mut().unwrap().next = Some(node);
            curr = &mut curr
                .as_deref_mut()
                .unwrap()
                .next
                .as_deref_mut()
                .unwrap()
                .next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &[i32], expected: &[i32]) {
        assert!(input.len() >= 1);
        assert!(input.len() <= 5000);
        assert_eq!(expected.len(), input.len() + input.len() - 1);
        for &n in input {
            assert!(n >= 1);
            assert!(n <= 1000);
        }
        for &n in expected {
            assert!(n >= 1);
            assert!(n <= 1000);
        }
        let head = ListNode::from_slice(input);
        let expected = ListNode::from_slice(expected);
        assert_eq!(Solution::insert_greatest_common_divisors(head), expected)
    }

    #[test]
    fn ex1() {
        test(&[18, 6, 10, 3], &[18, 6, 6, 2, 10, 1, 3]);
    }

    #[test]
    fn ex2() {
        test(&[7], &[7]);
    }

    #[test]
    fn myex1_1() {
        test(&[18, 6, 10, 3, 9], &[18, 6, 6, 2, 10, 1, 3, 3, 9]);
    }
}
