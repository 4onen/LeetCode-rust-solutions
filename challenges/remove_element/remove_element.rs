// https://leetcode.com/problems/remove-element/

pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        let expected = 2;
        let result = Solution::remove_element(&mut nums, val);
        assert_eq!(expected, result);
    }

    #[test]
    fn ex2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        let expected = 5;
        let result = Solution::remove_element(&mut nums, val);
        assert_eq!(expected, result);
    }
}
