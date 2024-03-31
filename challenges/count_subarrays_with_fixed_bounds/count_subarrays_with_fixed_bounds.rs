// https://leetcode.com/problems/count-subarrays-with-fixed-bounds/

pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        assert!(2 <= nums.len());
        assert!(nums.len() <= 100_000);
        assert!(1 <= min_k);
        if max_k < min_k {
            return 0;
        }
        assert!(max_k <= 1_000_000);
        let mut count = 0i64;
        let mut last_min = -1i32;
        let mut last_max = -1i32;
        let mut after_last_break = 0i32;
        for right in 0..nums.len() as i32 {
            if nums[right as usize] < min_k || nums[right as usize] > max_k {
                last_min = -1;
                last_max = -1;
                after_last_break = right + 1;
            } else {
                if nums[right as usize] == min_k {
                    last_min = right;
                }
                if nums[right as usize] == max_k {
                    last_max = right;
                }
                count += std::cmp::max(0, std::cmp::min(last_min, last_max) - after_last_break + 1)
                    as i64;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let nums = vec![1, 3, 5, 2, 7, 5];
        let min_k = 1;
        let max_k = 5;
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), 2);
    }

    #[test]
    fn ex2() {
        let nums = vec![1, 1, 1, 1];
        let min_k = 1;
        let max_k = 1;
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), 10);
    }

    #[test]
    fn discussion_case1() {
        let nums = vec![
            35054, 398719, 945315, 945315, 820417, 945315, 35054, 945315, 171832, 945315, 35054,
            109750, 790964, 441974, 552913,
        ];
        let min_k = 35054;
        let max_k = 945315;
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), 81);
    }

    #[test]
    fn discussion_case2() {
        let nums = vec![
            934372, 927845, 479424, 49441, 17167, 17167, 65553, 927845, 17167, 927845, 17167,
            425106, 17167, 927845, 17167, 927845, 251338, 17167,
        ];
        let min_k = 17167;
        let max_k = 927845;
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), 118);
    }

    #[test]
    fn discussion_case3() {
        let nums = vec![1, 3, 2, 2, 1, 3, 2, 2];
        let min_k = 1;
        let max_k = 3;
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), 20);
    }
}
