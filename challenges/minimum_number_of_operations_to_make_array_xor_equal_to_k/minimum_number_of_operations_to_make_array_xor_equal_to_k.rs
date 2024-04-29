// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-xor-equal-to-k/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
//         let arr_xor = nums.into_iter().fold(0, core::ops::BitXor::bitxor);
//         (arr_xor ^ k).count_ones() as i32
//     }
// }

// Better folding (yet somehow slower)
// impl Solution {
//     pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
//         nums.into_iter()
//             .fold(k, core::ops::BitXor::bitxor)
//             .count_ones() as i32
//     }
// }

// Const-ified
// impl Solution {
//     pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
//         const fn min_operations_impl(nums: &[i32], k: i32) -> i32 {
//             let mut acc = k;
//             let mut i = 0u32;
//             while i < nums.len() as u32 {
//                 acc ^= nums[i as usize];
//                 i += 1;
//             }
//             acc.count_ones() as i32
//         }
//         min_operations_impl(&nums, k)
//     }
// }

// Oh just .iter it
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().fold(k, |a, &x| a ^ x).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[u32], k: u32, expected: u8) {
        assert_eq!(
            Solution::min_operations(nums.into_iter().map(|i| *i as i32).collect(), k as i32),
            expected as i32
        );
    }

    #[test]
    fn ex1() {
        test(&[2, 1, 3, 4], 1, 2);
    }

    #[test]
    fn ex2() {
        test(&[2, 0, 2, 0], 0, 0);
    }

    #[test]
    fn myex1() {
        test(&[1, 1, 1, 1], 2, 1);
    }
}
