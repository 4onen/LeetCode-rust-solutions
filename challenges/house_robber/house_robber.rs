// https://leetcode.com/problems/house-robber/

pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut prev2 = 0;
        let mut prev1 = 0;
        for i in nums {
            let temp = std::cmp::max(i + prev2, prev1);
            prev2 = prev1;
            prev1 = temp;
        }
        prev1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
