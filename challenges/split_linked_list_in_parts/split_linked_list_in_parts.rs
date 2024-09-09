// https://leetcode.com/problems/split-linked-list-in-parts/

use utils::ListNode;

pub struct Solution;

// Initial sol'n
// impl Solution {
//     fn take(
//         head: Option<Box<ListNode>>,
//         n: usize,
//     ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
//         let mut taken = head;
//         let mut current = &mut taken;
//         for _ in 0..n {
//             if let Some(node) = current {
//                 current = &mut node.next;
//             } else {
//                 break;
//             }
//         }
//         let rest = current.take();
//         (taken, rest)
//     }
//     pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
//         assert!(k >= 1);
//         let mut len = 0usize;
//         let mut current = &head;
//         while let Some(node) = current {
//             len += 1;
//             current = &node.next;
//         }
//         let per_range = len / k as usize;
//         let mut remainder = len % k as usize;
//         let mut result = vec![];
//         let mut current = head;
//         for _ in 0..k {
//             let (taken, rest) = Self::take(current, per_range + if remainder > 0 { 1 } else { 0 });
//             remainder = remainder.saturating_sub(1);
//             result.push(taken);
//             current = rest;
//         }
//         result
//     }
// }

// Optimized sol'n
impl Solution {
    fn take(
        head: Option<Box<ListNode>>,
        n: u16,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut taken = head;
        let mut current = &mut taken;
        for _ in 0..n {
            if let Some(node) = current {
                current = &mut node.next;
            } else {
                break;
            }
        }
        let rest = current.take();
        (taken, rest)
    }
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        assert!(k >= 1);
        let mut len = 0u16;
        let mut current = &head;
        while let Some(node) = current {
            len += 1;
            current = &node.next;
        }
        let per_range = len / k as u16;
        let mut remainder = len % k as u16;
        let mut result = vec![];
        let mut current = head;
        for _ in 0..k {
            let (taken, rest) = Self::take(current, per_range + if remainder > 0 { 1 } else { 0 });
            remainder = remainder.saturating_sub(1);
            result.push(taken);
            current = rest;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_take(input: &[i32], n: u16, expected_taken: &[i32], expected_rest: &[i32]) {
        let head = ListNode::from_slice(input);
        let (taken, rest) = Solution::take(head, n);
        assert_eq!(taken, ListNode::from_slice(expected_taken));
        assert_eq!(rest, ListNode::from_slice(expected_rest));
    }

    #[test]
    fn test_take1() {
        test_take(&[1, 2, 3], 1, &[1], &[2, 3])
    }

    #[test]
    fn test_take2() {
        test_take(&[1, 2, 3], 2, &[1, 2], &[3])
    }

    #[test]
    fn test_take3() {
        test_take(&[1, 2, 3], 3, &[1, 2, 3], &[])
    }

    fn test(input: &[i32], expected: &[&[i32]]) {
        assert!(input.len() <= 1000);
        assert!(expected.len() >= 1);
        assert!(expected.len() <= 50);
        let head = ListNode::from_slice(input);
        let k = expected.len() as i32;
        let result = Solution::split_list_to_parts(head, k);
        let expected = expected
            .iter()
            .map(|slice| ListNode::from_slice(slice))
            .collect::<Vec<_>>();
        assert_eq!(result, expected);
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 3], &[&[1], &[2], &[3], &[], &[]])
    }

    #[test]
    fn ex2() {
        test(
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            &[&[1, 2, 3, 4], &[5, 6, 7], &[8, 9, 10]],
        )
    }

    #[test]
    fn myex1() {
        test(
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            &[&[1, 2, 3, 4, 5], &[6, 7, 8, 9, 10]],
        )
    }

    #[test]
    fn discussion_case1() {
        test(&[0, 1, 2, 3, 4], &[&[0, 1], &[2, 3], &[4]])
    }

    #[test]
    fn discussion_case2() {
        let mut input = [0; 1000];
        for i in 0..1000 {
            input[i] = i as i32;
        }
        let k = 6;
        let per_range = 1000 / k + 1; //167
        let mut expected = vec![];
        expected.push(&input[0 * per_range..1 * per_range]); // 167 elements
        expected.push(&input[1 * per_range..2 * per_range]); // 167 elements
        expected.push(&input[2 * per_range..3 * per_range]); // 167 elements
        expected.push(&input[3 * per_range..4 * per_range]); // 167 elements
        expected.push(&input[4 * per_range..5 * per_range - 1]); // 166 elements
        expected.push(&input[5 * per_range - 1..6 * per_range - 2]); // 166 elements
        test(&input, &expected);
    }

    #[test]
    fn discussion_case3() {
        test(&[1, 2, 3, 4, 5, 6, 7], &[&[1, 2, 3], &[4, 5], &[6, 7]])
    }

    #[test]
    fn discussion_case0() {
        let result: [&[i32]; 50] = [&[]; 50];
        test(&[], &result[..])
    }
}
