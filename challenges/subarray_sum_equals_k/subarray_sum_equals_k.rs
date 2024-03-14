// https://leetcode.com/problems/subarray-sum-equals-k/

pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0i32;
        let mut sum = 0;
        let mut map = std::collections::HashMap::<i32, u16>::with_capacity(nums.len() + 1);
        map.insert(0, 1);
        for i in 0..nums.len() {
            sum += nums[i];
            if let Some(&v) = map.get(&(sum - k)) {
                count += v as i32;
            }
            *map.entry(sum).or_insert(0) += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let nums = vec![1, 1, 1];
        let k = 2;
        assert_eq!(Solution::subarray_sum(nums, k), 2);
    }

    #[test]
    fn ex2() {
        let nums = vec![1, 2, 3];
        let k = 3;
        assert_eq!(Solution::subarray_sum(nums, k), 2);
    }

    #[test]
    fn discussion_case1() {
        let nums = vec![3, 4, 7, -2, 2, 1, 4, 2];
        let k = 7;
        assert_eq!(Solution::subarray_sum(nums, k), 6);
    }

    #[test]
    fn discussion_case2() {
        let nums = vec![2, 3, 6, 5, 4, 1, 10];
        let k = 5;
        assert_eq!(Solution::subarray_sum(nums, k), 3);
    }

    #[test]
    fn discussion_case3() {
        let nums = vec![-4, 3, 6, -2, 1, -1, 0, 2, -2, 3, 1];
        let k = 5;
        assert_eq!(Solution::subarray_sum(nums, k), 3);
    }
}
