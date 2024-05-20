// https://leetcode.com/problems/sum-of-all-subset-xor-totals/

pub struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        assert!(nums.len() > 0);
        assert!(nums.len() <= 20);
        let n = nums.len() as u8;
        let mut result = 0;
        for i in 0..(1 << n) {
            let mut xor = 0;
            for j in 0..n {
                if i & (1 << j) != 0 {
                    xor ^= nums[j as usize];
                }
            }
            result += xor;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[u8], expected: i32) {
        assert!(nums.len() > 0);
        assert!(nums.len() <= 20);
        let nums = nums.iter().map(|&x| x as i32).collect();
        assert_eq!(Solution::subset_xor_sum(nums), expected);
    }

    #[test]
    fn ex1() {
        test(&[1, 3], 6)
    }

    #[test]
    fn ex2() {
        test(&[5, 1, 6], 28)
    }

    #[test]
    fn ex3() {
        test(&[3, 4, 5, 6, 7, 8], 480)
    }

    #[test]
    fn discussion_case1() {
        test(&[1, 1, 1], 4)
    }

    #[test]
    fn discussion_case2() {
        test(&[20], 20)
    }

    #[test]
    fn discussion_case3() {
        test(&[3, 4, 5, 6, 7, 8, 20, 19, 18, 17, 16, 15], 63488)
    }

    #[test]
    fn discussion_case4() {
        test(&[9, 9, 9, 9, 9, 9, 9, 9, 9], 2304)
    }

    #[test]
    fn discussion_case5() {
        test(&[1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2], 6144)
    }

    #[test]
    fn discussion_case6() {
        test(&[1, 2, 4, 8, 16], 496)
    }
}
