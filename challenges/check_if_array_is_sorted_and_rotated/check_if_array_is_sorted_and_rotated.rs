// https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/

pub struct Solution;

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut count = nums.windows(2).filter(|w| w[0] > w[1]).count();
        if nums[nums.len() - 1] > nums[0] {
            count += 1;
        }
        count <= 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::check(vec![3, 4, 5, 1, 2]), true);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::check(vec![2, 1, 3, 4]), false);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::check(vec![1, 2, 3]), true);
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::check(vec![1, 2, 3, 4, 5]), true);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::check(vec![5, 1, 2, 3, 4]), true);
    }
}