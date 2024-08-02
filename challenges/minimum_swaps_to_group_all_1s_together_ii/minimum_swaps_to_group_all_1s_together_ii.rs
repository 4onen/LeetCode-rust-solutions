// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together-ii/

pub struct Solution;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let ones = nums.iter().filter(|&&x| x == 1).count();
        if ones == 0 || ones == nums.len() {
            return 0;
        }
        let mut zeros_in_window = 0;
        for i in 0..ones {
            if nums[i] == 0 {
                zeros_in_window += 1;
            }
        }
        let mut min_swaps = zeros_in_window;
        for i in ones..nums.len() + ones {
            if nums[i - ones] == 0 {
                zeros_in_window -= 1;
            }
            if nums[i % nums.len()] == 0 {
                zeros_in_window += 1;
            }
            min_swaps = min_swaps.min(zeros_in_window);
            if min_swaps == 0 {
                return 0;
            }
        }
        min_swaps as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100_000);
        for &num in nums {
            assert!(num == 0 || num == 1);
        }
        assert_eq!(Solution::min_swaps(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[0, 1, 0, 1, 1, 0, 0], 1);
    }

    #[test]
    fn ex2() {
        test(&[0, 1, 1, 1, 0, 0, 1, 1, 0], 2);
    }

    #[test]
    fn ex3() {
        test(&[1, 1, 0, 0, 1], 0);
    }

    #[test]
    fn discussion_case1() {
        test(&[1, 1, 1, 0, 0, 1, 0, 1, 1, 0], 1);
    }
}
