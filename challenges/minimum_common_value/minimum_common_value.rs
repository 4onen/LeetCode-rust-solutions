// https://leetcode.com/problems/minimum-common-value/

pub struct Solution;

// Braindead sol'n
// impl Solution {
//     pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
//         let mut i = 0;
//         let mut j = 0;
//         loop {
//             if i >= nums1.len() {
//                 break -1;
//             }
//             if j >= nums2.len() {
//                 break -1;
//             }
//             match nums1[i].cmp(&nums2[j]) {
//                 std::cmp::Ordering::Greater => j += 1,
//                 std::cmp::Ordering::Less => i += 1,
//                 std::cmp::Ordering::Equal => break nums1[i],
//             }
//         }
//     }
// }

// Optimized sol'n
impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        while i < nums1.len() && j < nums2.len() {
            match nums1[i].cmp(&nums2[j]) {
                std::cmp::Ordering::Greater => j += 1,
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Equal => return nums1[i],
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4];
        assert_eq!(Solution::get_common(nums1, nums2), 2);
    }

    #[test]
    fn ex2() {
        let nums1 = vec![1, 2, 3, 6];
        let nums2 = vec![2, 3, 4, 5];
        assert_eq!(Solution::get_common(nums1, nums2), 2);
    }

    #[test]
    fn discussion_case1() {
        let nums1 = vec![100, 200, 300];
        let nums2 = vec![50, 100, 150, 200];
        assert_eq!(Solution::get_common(nums1, nums2), 100);
    }

    #[test]
    fn discussion_case2() {
        let nums1 = vec![2, 4, 6, 8, 10];
        let nums2 = vec![1, 3, 5, 7, 9];
        assert_eq!(Solution::get_common(nums1, nums2), -1);
    }
}
