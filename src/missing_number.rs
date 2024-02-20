// https://leetcode.com/problems/missing-number/

pub struct Solution;

// Const sum sol'n
// impl Solution {
//     pub const fn missing_number_const(nums: &[i32]) -> u32 {
//         let n = nums.len() as u32;
//         let sum: u32 = n * (n + 1) / 2;
//         let mut sum2 = 0;
//         let mut i = 0;
//         while i < n {
//             sum2 += nums[i as usize] as u32;
//             i += 1;
//         }
//         sum - sum2
//     }
//     pub fn missing_number(nums: Vec<i32>) -> i32 {
//         Self::missing_number_const(&nums) as i32
//     }
// }

// Bit manipulation sol'n v1
impl Solution {
    pub const fn missing_number_const(nums: &[i32]) -> i32 {
        let n = nums.len() as i32;
        let mut missing = nums.len() as i32;
        let mut i = 0;
        while i < n {
            missing ^= nums[i as usize] ^ i as i32;
            i += 1;
        }
        missing
    }
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        Self::missing_number_const(&nums)
    }
}

// Bit manipulation sol'n v2
// impl Solution {
//     pub const fn missing_number_const(nums: &[i32]) -> i32 {
//         let mut missing = nums.len() as i32;
//         let mut direct_reduction = 0;
//         let mut i = 0;
//         while i < nums.len() {
//             missing ^= nums[i];
//             direct_reduction ^= i as i32;
//             i += 1;
//         }
//         missing ^ direct_reduction
//     }
//     pub fn missing_number(nums: Vec<i32>) -> i32 {
//         Self::missing_number_const(&nums)
//     }
// }

// Iterator sol'n
// impl Solution {
//     pub fn missing_number(nums: Vec<i32>) -> i32 {
//         let n = nums.len() as i32;
//         let sum: i32 = (n * (n + 1)) / 2;
//         let sum2: i32 = nums.into_iter().sum();
//         sum - sum2
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(Solution::missing_number(vec![0]), 1);
    }

    #[test]
    fn myex0() {
        assert_eq!(Solution::missing_number(vec![1]), 0);
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::missing_number(vec![1, 2, 3]), 0);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::missing_number(vec![0, 1, 2]), 3);
    }

    #[test]
    fn my_extrema_ex1() {
        let input = (0..10000).collect::<Vec<i32>>();
        assert_eq!(Solution::missing_number(input), 10000);
    }

    #[test]
    fn my_extrema_ex2() {
        let input = (1..=10000).collect::<Vec<i32>>();
        assert_eq!(Solution::missing_number(input), 0);
    }
}
