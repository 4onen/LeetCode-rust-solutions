// https://leetcode.com/problems/bitwise-xor-of-all-pairings/

pub struct Solution;

// Naive direct implementation
// impl Solution {
//     pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
//         let mut res = 0;
//         for num1 in nums1 {
//             for &num2 in &nums2 {
//                 res ^= num1 ^ num2;
//             }
//         }
//         res
//     }
// }

// Efficient impl
impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut res = 0;
        if nums2.len() % 2 == 1 {
            for num in &nums1 {
                res ^= num;
            }
        }
        if nums1.len() % 2 == 1 {
            for num in &nums2 {
                res ^= num;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums1: &[i32], nums2: &[i32], expected: i32) {
        assert_eq!(Solution::xor_all_nums(nums1.to_vec(),nums2.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(
            &[2,1,3], &[10,2,5,0], 13
        )
    }

    #[test]
    fn ex1_1() {
        test(
            &[2,1,3], &[10,2,5], 13
        )
    }

    #[test]
    fn ex1_2() {
        test(
            &[2,1,3], &[10,2,5,0,0], 13
        )
    }

    #[test]
    fn ex2() {
        test(
            &[1,2], &[3,4], 0
        )
    }
}
