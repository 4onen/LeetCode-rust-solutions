// https://leetcode.com/problems/minimum-limit-of-balls-in-a-bag/

pub struct Solution;

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut left = 1;
        let mut right = *nums.iter().max().unwrap();
        let mut result = right;
        while left <= right {
            let mid = left + (right - left) / 2;
            let mut operations = 0;
            for &num in &nums {
                operations += (num - 1) / mid;
            }
            if operations <= max_operations {
                result = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], max_operations: i32, expected: i32) {
        assert_eq!(
            Solution::minimum_size(nums.to_vec(), max_operations),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[9], 2, 3);
        test(&[3, 6], 1, 3);
    }

    #[test]
    fn ex2() {
        test(&[2, 4, 8, 2], 4, 2);
        test(&[2, 4, 4, 4, 2], 3, 2);
        test(&[2, 4, 4, 2, 2, 2], 2, 2);
        test(&[2, 4, 2, 2, 2, 2, 2, 2], 1, 2);
    }

    #[test]
    fn ex2_1() {
        test(&[2, 12, 2], 5, 2);
    }

    #[test]
    fn myex1() {
        test(&[7, 17], 2, 7);
        test(&[7, 7, 10], 1, 7);
        test(&[7, 6, 11], 1, 7);
        test(&[7, 5, 12], 1, 7);
        test(&[7, 4, 13], 1, 7);
        test(&[7, 3, 14], 1, 7);
    }
}
