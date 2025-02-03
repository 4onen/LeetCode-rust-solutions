// https://leetcode.com/problems/number-of-unequal-triplets-in-array/

pub struct Solution;

// Naive brute force
impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] {
                    continue;
                }
                for k in j + 1..nums.len() {
                    if nums[i] != nums[k] && nums[j] != nums[k] {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i32) {
        assert!(nums.len() >= 3);
        assert!(nums.len() <= 100);
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 1000);
        }
        assert_eq!(Solution::unequal_triplets(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[4, 4, 2, 4, 3], 3)
    }

    #[test]
    fn ex2() {
        test(&[1, 1, 1, 1, 1], 0)
    }

    #[test]
    fn discussion_case1() {
        test(&[1, 3, 1, 2, 4], 7)
    }
}
