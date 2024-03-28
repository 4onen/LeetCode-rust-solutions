// https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency/

pub struct Solution;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        if k < 1 {
            0
        } else if k >= nums.len() as i32 {
            nums.len() as i32
        } else {
            let mut count = 0;
            let mut frequencies = std::collections::HashMap::new();
            let mut left = 0;
            for right in 0..nums.len() {
                let right_num = nums[right];
                *frequencies.entry(right_num).or_insert(0) += 1;
                while frequencies[&right_num] > k {
                    frequencies.entry(nums[left]).and_modify(|e| *e -= 1);
                    left += 1;
                }
                count = std::cmp::max(count, right - left + 1);
            }
            count as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let nums = vec![1, 2, 3, 1, 2, 3, 1, 2];
        let k = 2;
        assert_eq!(Solution::max_subarray_length(nums, k), 6);
    }

    #[test]
    fn ex2() {
        let nums = vec![1, 2, 1, 2, 1, 2, 1, 2];
        let k = 1;
        assert_eq!(Solution::max_subarray_length(nums, k), 2);
    }

    #[test]
    fn ex3() {
        let nums = vec![5; 7];
        let k = 4;
        assert_eq!(Solution::max_subarray_length(nums, k), 4);
    }

    #[test]
    fn discussion_case1() {
        let nums = vec![2];
        let k = 2;
        assert_eq!(Solution::max_subarray_length(nums, k), 1);
    }

    #[test]
    fn discussion_case2() {
        let nums = vec![1, 4, 4, 3];
        let k = 1;
        assert_eq!(Solution::max_subarray_length(nums, k), 2);
    }

    #[test]
    fn discussion_case3() {
        let nums = vec![2, 2, 3];
        let k = 1;
        assert_eq!(Solution::max_subarray_length(nums, k), 2);
    }

    #[test]
    fn my_extreme_ex1() {
        let nums = vec![1; 100_000];
        let k = 1;
        assert_eq!(Solution::max_subarray_length(nums, k), 1);
    }

    #[test]
    fn my_extreme_ex2() {
        let nums = vec![1; 100_000];
        let k = 100_000;
        assert_eq!(Solution::max_subarray_length(nums, k), 100_000);
    }

    #[test]
    fn my_extreme_ex3() {
        let nums = vec![1; 100_000];
        let k = 100_001;
        assert_eq!(Solution::max_subarray_length(nums, k), 100_000);
    }

    #[test]
    fn my_extreme_ex4() {
        let nums = vec![1; 100_000];
        let k = 50_002;
        assert_eq!(Solution::max_subarray_length(nums, k), 50_002);
    }
}
