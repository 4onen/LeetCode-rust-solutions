// https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer/

pub struct Solution;

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let negative = nums.partition_point(|&x| x < 0);
        let rest = nums.len() - negative;
        let positive = rest - nums[negative..].partition_point(|&x| x == 0);
        std::cmp::max(negative, positive) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 2000);
        let mut last = -2000;
        for &num in nums {
            assert!(num >= -2000);
            assert!(num <= 2000);
            assert!(num >= last);
            last = num;
        }
        assert_eq!(Solution::maximum_count(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[-2, -1, -1, 1, 2, 3], 3)
    }

    #[test]
    fn ex2() {
        test(&[-3, -2, -1, 0, 0, 1, 2], 3)
    }

    #[test]
    fn ex3() {
        test(&[5, 20, 77, 1314], 4)
    }
}
