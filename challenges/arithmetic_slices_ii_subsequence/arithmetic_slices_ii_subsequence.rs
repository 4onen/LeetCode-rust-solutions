// https://leetcode.com/problems/arithmetic-slices-ii-subsequence/

pub struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut dp = vec![std::collections::HashMap::new(); nums.len()];
        let mut result = 0;
        for i in 0..nums.len() {
            for j in 0..i {
                let diff = nums[i] as i64 - nums[j] as i64;
                let count = dp[j].get(&diff).unwrap_or(&0);
                result += count;
                *dp[i].entry(diff).or_insert(0) += count + 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]),
            7
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7]),
            16
        );
    }

    #[test]
    fn description_ex1() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![1, 3, 5, 7, 9]),
            7
        );
    }

    #[test]
    fn description_ex2() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7]), 5);
    }

    #[test]
    fn description_ex3() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![3, -1, -5, -9]),
            3
        )
    }

    #[test]
    fn myex0() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![0, 1, 2, 2, 2, 3]),
            10
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![0, 2000000000, -294967296]),
            0
        );
    }

    #[test]
    fn myex2() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![0, 2000000000, -294967296, 294967296]),
            0
        );
    }

    #[test]
    fn myex3() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![0, 2000000000, -294967296, 294967296, 0]),
            0
        );
    }

    #[test]
    fn myex4() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![-294967296, 0, 294967296]),
            1
        );
    }

    #[test]
    fn myex5() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![-294967296, 0, 0, 294967296]),
            2
        );
    }

    #[test]
    fn myex6() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![-294967296, 0, 0, 0, 294967296]),
            4
        );
    }

    #[test]
    fn my_extreme_ex1() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![
                -2147483648,
                -2147483648,
                -2147483648,
                -2147483648,
                -2147483648,
                2147483647,
                2147483647,
                2147483647,
                2147483647,
                2147483647,
            ]),
            32
        );
    }

    #[test]
    fn my_extreme_ex2() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![
                -2147483648,
                -2147483648,
                -2147483648,
                -2147483648,
                -2147483648,
                0,
                2147483647,
                2147483647,
                2147483647,
                2147483647,
                2147483647,
            ]),
            32
        );
    }

    #[test]
    fn my_extreme_ex3() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![
                -2147483647,
                -2147483647,
                -2147483647,
                -2147483647,
                -2147483647,
                0,
                2147483647,
                2147483647,
                2147483647,
                2147483647,
                2147483647,
            ]),
            32 + 25
        );
    }

    #[test]
    fn my_extreme_ex4() {
        assert_eq!(
            Solution::number_of_arithmetic_slices((1..=1000).collect()),
            2781846
        );
    }
}
