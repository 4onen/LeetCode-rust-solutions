// https://leetcode.com/problems/special-array-i/

pub struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        for i in 1..nums.len() as u8 {
            if (nums[i as usize - 1] ^ nums[i as usize]) & 1 == 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: bool) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100);
        for &n in nums.iter() {
            assert!(n >= 1);
            assert!(n <= 100);
        }
        assert_eq!(Solution::is_array_special(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[1], true)
    }

    #[test]
    fn ex2() {
        test(&[2, 1, 4], true)
    }

    #[test]
    fn ex3() {
        test(&[4, 3, 1, 6], false)
    }
}
