// https://leetcode.com/problems/intersection-of-two-arrays-ii/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
//         nums1.sort_unstable();
//         nums2.sort_unstable();
//         let mut i = 0;
//         let mut j = 0;
//         let mut result = Vec::new();
//         while i < nums1.len() && j < nums2.len() {
//             if nums1[i] == nums2[j] {
//                 result.push(nums1[i]);
//                 i += 1;
//                 j += 1;
//             } else if nums1[i] < nums2[j] {
//                 i += 1;
//             } else {
//                 j += 1;
//             }
//         }
//         result
//     }
// }

// Counting sol'n
// impl Solution {
//     pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
//         use std::collections::HashMap;
//         let mut map = HashMap::new();
//         for num in nums1 {
//             *map.entry(num).or_insert(0) += 1;
//         }
//         let mut result = Vec::new();
//         for num in nums2 {
//             if let Some(count) = map.get_mut(&num) {
//                 if *count > 0 {
//                     result.push(num);
//                     *count -= 1;
//                 }
//             }
//         }
//         result
//     }
// }

// Linear counting sol'n
impl Solution {
    pub fn intersect(nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let mut map = [0u16;1001];
        for num in nums1 {
            map[num as usize] += 1;
        }
        nums2.retain(|&num| {
            if let Some(count) = map.get_mut(num as usize) {
                if *count > 0 {
                    *count -= 1;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        });
        nums2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums1: &[i32], nums2: &[i32], expected: &[i32]) {
        let mut result = Solution::intersect(nums1.to_vec(), nums2.to_vec());
        assert!(result.len() <= nums1.len());
        assert!(result.len() <= nums2.len());
        result.sort_unstable();
        let mut sorted_expectation = expected.to_vec();
        sorted_expectation.sort_unstable();
        assert_eq!(result, sorted_expectation);
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 2, 1], &[2, 2], &[2, 2]);
    }

    #[test]
    fn ex2() {
        test(&[4, 9, 5], &[9, 4, 9, 8, 4], &[4, 9]);
    }
}
