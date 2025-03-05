// https://leetcode.com/problems/jump-game/

pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = (nums.len() - 1) as u32;
        let mut max = 0 as u32;
        let mut i = 0 as u32;
        while i <= max && i < n {
            let num = nums[i as usize];
            if i + num as u32 > max {
                max = i + num as u32;
            }
            i += 1;
        }
        max >= n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: bool) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 10_000);
        for &num in nums {
            assert!(num >= 0);
            assert!(num <= 100_000);
        }
        assert_eq!(Solution::can_jump(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[2, 3, 1, 1, 4], true)
    }

    #[test]
    fn ex2() {
        test(&[3, 2, 1, 0, 4], false)
    }

    #[test]
    fn discussion_case1() {
        test(&[1, 1, 2, 2, 0, 1, 1], true)
    }

    #[test]
    fn discussion_case2() {
        test(&[2, 5, 0, 0], true)
    }

    #[test]
    fn discussion_case3() {
        test(&[1, 0, 2], false)
    }

    #[test]
    fn discussion_case4() {
        test(&[0, 2, 3], false)
    }

    #[test]
    fn discussion_case5() {
        test(&[0], true)
    }
}
