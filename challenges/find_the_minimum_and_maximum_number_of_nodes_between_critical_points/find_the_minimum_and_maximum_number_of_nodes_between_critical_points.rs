// https://leetcode.com/problems/find-the-minimum-and-maximum-number-of-nodes-between-critical-points/

use utils::ListNode;

pub struct Solution;

// Initial linear pass NonZeroU32 sol'n
// impl Solution {
//     pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
//         use std::num::NonZeroU32;
//         let mut first_critical_idx: Option<NonZeroU32> = None;
//         let mut last_critical_idx: Option<NonZeroU32> = None;
//         let mut min_distance: Option<NonZeroU32> = None;
//         // Max distance we'll calculate when we're done walking the list
//         // We know there are at least 2 nodes in the list
//         // and the last node cannot be a critical point,
//         // so we can hold only the next node inside its box
//         // (Essentially walking nodes a step ahead of the index.)
//         let ListNode {
//             val: mut prev_val,
//             next: curr,
//         } = *head.unwrap();
//         let ListNode {
//             val: mut curr_val,
//             mut next,
//         } = *curr.unwrap();
//         let mut idx = unsafe { NonZeroU32::new_unchecked(1) };
//         loop {
//             if next.is_none() {
//                 break;
//             }
//             let ListNode {
//                 val: next_val,
//                 next: next_next,
//             } = *next.unwrap();
//             if (prev_val > curr_val && next_val > curr_val)
//                 || (prev_val < curr_val && next_val < curr_val)
//             {
//                 match last_critical_idx {
//                     None => {
//                         first_critical_idx = Some(idx);
//                         last_critical_idx = Some(idx);
//                     }
//                     Some(last_critical_idx_unwrapped) => {
//                         let distance = idx.get() - last_critical_idx_unwrapped.get();
//                         match min_distance {
//                             None => {
//                                 min_distance = Some(unsafe { NonZeroU32::new_unchecked(distance) });
//                             }
//                             Some(min_distance_unwrapped) => {
//                                 if distance < min_distance_unwrapped.get() {
//                                     min_distance =
//                                         Some(unsafe { NonZeroU32::new_unchecked(distance) });
//                                 }
//                             }
//                         }
//                         last_critical_idx = Some(idx);
//                     }
//                 }
//             }
//             prev_val = curr_val;
//             curr_val = next_val;
//             next = next_next;
//             idx = unsafe { NonZeroU32::new_unchecked(idx.get() + 1) };
//         }
//         match (first_critical_idx, last_critical_idx) {
//             (Some(first_critical_idx_unwrapped), Some(last_critical_idx_unwrapped)) => {
//                 let distance =
//                     last_critical_idx_unwrapped.get() - first_critical_idx_unwrapped.get();
//                 match min_distance {
//                     None => vec![-1, -1],
//                     Some(min_distance_unwrapped) => {
//                         vec![min_distance_unwrapped.get() as i32, distance as i32]
//                     }
//                 }
//             }
//             _ => vec![-1, -1],
//         }
//     }
// }

// Linear pass sol'n with raw indices
impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut first_critical_idx: u32 = 0;
        let mut last_critical_idx: u32 = 0;
        let mut min_distance: u32 = u32::MAX;
        // Max distance we'll calculate when we're done walking the list
        // We know there are at least 2 nodes in the list
        // and the last node cannot be a critical point,
        // so we can hold only the next node inside its box
        // (Essentially walking nodes a step ahead of the index.)
        let ListNode {
            val: mut prev_val,
            next: curr,
        } = *head.unwrap();
        let ListNode {
            val: mut curr_val,
            mut next,
        } = *curr.unwrap();
        let mut idx = 1u32;
        loop {
            if next.is_none() {
                break;
            }
            let ListNode {
                val: next_val,
                next: next_next,
            } = *next.unwrap();
            next = next_next;
            if (prev_val > curr_val && next_val > curr_val)
                || (prev_val < curr_val && next_val < curr_val)
            {
                if last_critical_idx < 1 {
                    first_critical_idx = idx;
                    last_critical_idx = idx;
                } else {
                    let distance = idx - last_critical_idx;
                    if distance < min_distance {
                        min_distance = distance;
                    }
                    last_critical_idx = idx;
                }
            }
            prev_val = curr_val;
            curr_val = next_val;
            idx += 1;
        }
        if last_critical_idx > first_critical_idx {
            vec![
                min_distance as i32,
                (last_critical_idx - first_critical_idx) as i32,
            ]
        } else {
            vec![-1, -1]
        }
    }
}

// Linear pass with only one step history
// impl Solution {
//     pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
//         let mut first_critical_idx: u32 = 0;
//         let mut last_critical_idx: u32 = 0;
//         let mut min_distance: u32 = u32::MAX;
//         // Max distance we'll calculate when we're done walking the list
//         // We know there are at least 2 nodes in the list
//         // and the last node cannot be a critical point,
//         // so we can hold only the next node inside its box
//         // (Essentially walking nodes a step ahead of the index.)
//         let ListNode {
//             val: prev_val,
//             next: curr,
//         } = *head.unwrap();
//         let ListNode {
//             val: mut curr_val,
//             mut next,
//         } = *curr.unwrap();
//         let mut prev_comparison = curr_val.cmp(&prev_val);
//         let mut idx = 1u32;
//         loop {
//             if next.is_none() {
//                 break;
//             }
//             let ListNode {
//                 val: next_val,
//                 next: next_next,
//             } = *next.unwrap();
//             next = next_next;
//             let curr_comparison = curr_val.cmp(&next_val);
//             if prev_comparison == curr_comparison && prev_comparison != std::cmp::Ordering::Equal {
//                 if last_critical_idx < 1 {
//                     first_critical_idx = idx;
//                     last_critical_idx = idx;
//                 } else {
//                     let distance = idx - last_critical_idx;
//                     if distance < min_distance {
//                         min_distance = distance;
//                     }
//                     last_critical_idx = idx;
//                 }
//             }
//             prev_comparison = curr_comparison.reverse();
//             curr_val = next_val;
//             idx += 1;
//         }
//         if last_critical_idx > first_critical_idx {
//             vec![
//                 min_distance as i32,
//                 (last_critical_idx - first_critical_idx) as i32,
//             ]
//         } else {
//             vec![-1, -1]
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let head = ListNode::from_slice(&[3, 1]);
        let result = Solution::nodes_between_critical_points(head);
        assert_eq!(result, vec![-1, -1]);
    }

    #[test]
    fn ex2() {
        let head = ListNode::from_slice(&[5, 3, 1, 2, 5, 1, 2]);
        let result = Solution::nodes_between_critical_points(head);
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn ex3() {
        let head = ListNode::from_slice(&[1, 3, 2, 2, 3, 2, 2, 2, 7]);
        let result = Solution::nodes_between_critical_points(head);
        assert_eq!(result, vec![3, 3]);
    }

    #[test]
    fn ex4() {
        let head = ListNode::from_slice(&[
            43, 69, 47, 14, 12, 80, 96, 55, 47, 7, 45, 75, 8, 51, 82, 35, 33, 27, 38, 27, 60, 85,
            39, 57, 100, 57, 41, 87, 8, 20, 75, 62, 1, 4, 55, 59, 67, 57, 79, 33, 38, 14, 39, 16,
            22, 55, 44, 64, 55, 31, 65, 73, 66, 34, 41, 72, 3, 78, 87, 49, 77, 76,
        ]);
        let result = Solution::nodes_between_critical_points(head);
        assert_eq!(result, vec![1, 59]);
    }

    #[test]
    fn ex5() {
        let head = ListNode::from_slice(&[7, 7, 10, 1, 7, 1, 2, 1, 5]);
        let result = Solution::nodes_between_critical_points(head);
        assert_eq!(result, vec![1, 5]);
    }

    #[test]
    fn ex6() {
        let head = ListNode::from_slice(&[6, 8, 4, 1, 9, 6, 6, 10, 6]);
        let result = Solution::nodes_between_critical_points(head);
        assert_eq!(result, vec![1, 6]);
    }
}
