// https://leetcode.com/problems/minimum-increment-to-make-array-unique/

pub struct Solution;

impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut moves = 0;
        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                moves += nums[i - 1] - nums[i] + 1;
                nums[i] = nums[i - 1] + 1;
            }
        }
        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100_000);
        for &num in nums {
            assert!(num >= 0);
            assert!(num <= 100_000);
        }
        assert_eq!(Solution::min_increment_for_unique(nums.to_vec()), expected)
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 2], 1)
    }

    #[test]
    fn ex2() {
        test(&[3, 2, 1, 2, 1, 7], 6)
    }

    #[test]
    fn discussion_case0() {
        test(&[0], 0)
    }

    #[test]
    fn myex0() {
        test(&[1], 0)
    }

    #[test]
    fn discussion_case1() {
        test(&[1, 0, 5, 8, 4, 5, 5, 5, 5, 5, 4], 26)
    }

    #[test]
    fn discussion_case2() {
        test(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0], 55)
    }

    #[test]
    fn discussion_case3() {
        test(&[8, 7, 8, 7, 8, 7, 8, 7, 8, 7, 8], 49)
    }
}
