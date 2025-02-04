// https://leetcode.com/problems/maximum-ascending-subarray-sum/

pub struct Solution;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = 0;
        let mut sum = 0;
        for i in 0..nums.len() as u8 {
            sum += nums[i as usize];
            if nums[i as usize] >= nums.get((i + 1) as usize).copied().unwrap_or(0) {
                max_sum = max_sum.max(sum);
                sum = 0;
            }
        }
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100);
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 100);
        }
        assert_eq!(Solution::max_ascending_sum(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[10, 20, 30, 5, 10, 50], 65)
    }

    #[test]
    fn ex2() {
        test(&[10, 20, 30, 40, 50], 150)
    }

    #[test]
    fn ex3() {
        test(&[12, 17, 15, 13, 10, 11, 12], 33)
    }

    #[test]
    fn discussion_case1() {
        test(&[3, 6, 10, 1, 8, 9, 9, 8, 9], 19)
    }

    #[test]
    fn my_extreme_ex0() {
        test(&[1], 1)
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[100], 100)
    }

    #[test]
    fn my_extreme_ex2() {
        test(&[1, 100], 101)
    }

    #[test]
    fn my_extreme_ex3() {
        let mut inputs = vec![1; 100];
        inputs[99] = 100;
        test(&inputs, 101)
    }

    #[test]
    fn my_extreme_ex4() {
        let mut inputs = vec![100; 100];
        inputs[99] = 1;
        test(&inputs, 100)
    }

    #[test]
    fn my_extreme_ex5() {
        let mut inputs = vec![100; 100];
        inputs[99] = 1;
        inputs[98] = 1;
        test(&inputs, 100)
    }

    #[test]
    fn my_extreme_ex6() {
        let mut inputs = vec![1; 100];
        inputs[99] = 100;
        inputs[98] = 100;
        test(&inputs, 101)
    }
}
