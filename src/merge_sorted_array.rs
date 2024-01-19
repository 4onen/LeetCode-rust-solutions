// https://leetcode.com/problems/merge-sorted-array/

pub struct Solution;

// Fill-from-back sol'n
// impl Solution {
//     pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
//         let mut i = m - 1;
//         let mut j = n - 1;
//         let mut k = m + n - 1;
//         while i >= 0 && j >= 0 {
//             if nums1[i as usize] > nums2[j as usize] {
//                 nums1[k as usize] = nums1[i as usize];
//                 i -= 1;
//             } else {
//                 nums1[k as usize] = nums2[j as usize];
//                 j -= 1;
//             }
//             k -= 1;
//         }
//         while j >= 0 {
//             nums1[k as usize] = nums2[j as usize];
//             j -= 1;
//             k -= 1;
//         }
//     }
// }

// Fill-from-back without k, because k=i+j+1
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m - 1;
        let mut j = n - 1;
        while i >= 0 && j >= 0 {
            if nums1[i as usize] > nums2[j as usize] {
                nums1[(i + j + 1) as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[(i + j + 1) as usize] = nums2[j as usize];
                j -= 1;
            }
        }
        if j >= 0 {
            nums1[..=j as usize].copy_from_slice(&nums2[..=j as usize]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        let expected = vec![1, 2, 2, 3, 5, 6];
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, expected);
    }

    #[test]
    fn ex2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        let expected = vec![1];
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, expected);
    }

    #[test]
    fn ex3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;
        let expected = vec![1];
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, expected);
    }
}
