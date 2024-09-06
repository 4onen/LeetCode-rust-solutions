// https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array/

use utils::ListNode;

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn modified_list(nums: Vec<i32>, mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         let nums_set: std::collections::HashSet<i32> = nums.into_iter().collect();
//         let mut prev_ref = &mut head;
//         loop {
//             if !prev_ref.is_some() { break; }
//             if !nums_set.contains(&prev_ref.as_ref().unwrap().val) {
//                 prev_ref = &mut prev_ref.as_deref_mut().unwrap().next;
//                 continue;
//             } else {
//                 let next = prev_ref.as_deref_mut().unwrap().next.take();
//                 *prev_ref = next;
//             }
//         }
//         head
//     }
// }

// Bool array set
impl Solution {
    pub fn modified_list(nums: Vec<i32>, mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut nums_set = vec![false; 100001];
        for num in nums {
            nums_set[num as usize] = true;
        }
        let mut prev_ref = &mut head;
        loop {
            if !prev_ref.is_some() {
                break;
            }
            if !nums_set[prev_ref.as_ref().unwrap().val as usize] {
                prev_ref = &mut prev_ref.as_deref_mut().unwrap().next;
            } else {
                let next = prev_ref.as_deref_mut().unwrap().next.take();
                *prev_ref = next;
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], list: &[i32], expected: &[i32]) {
        let head = ListNode::from_slice(list);
        let expected = ListNode::from_slice(expected);
        assert_eq!(Solution::modified_list(nums.to_vec(), head), expected);
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 3], &[1, 2, 3, 4, 5], &[4, 5])
    }

    #[test]
    fn ex2() {
        test(&[1], &[1, 2, 1, 2, 1, 2], &[2, 2, 2])
    }

    #[test]
    fn ex3() {
        test(&[5], &[1, 2, 3, 4], &[1, 2, 3, 4])
    }
}
