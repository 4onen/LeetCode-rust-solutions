// https://leetcode.com/problems/patching-array/

pub struct Solution;

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        assert!(nums.len() > 0);
        assert!(nums.len() <= 1000);
        assert!(n > 0);
        let n = n as u32;
        let mut reachable: u32 = 0;
        let mut result = 0;
        let mut i: u16 = 0;
        while reachable < n {
            if i < nums.len() as u16 && nums[i as usize] as u32 <= reachable + 1 {
                reachable += nums[i as usize] as u32;
                i += 1;
            } else {
                reachable += reachable + 1;
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[u16], n: i32, expected: i32) {
        assert!(nums.len() > 0);
        assert!(nums.len() <= 1000);
        for i in nums {
            assert!(*i >= 1);
            assert!(*i <= 10_000);
        }
        for i in 1..nums.len() {
            assert!(nums[i] >= nums[i - 1]);
        }
        assert!(n >= 1);
        assert!(n <= i32::MAX);
        let nums = nums.iter().map(|&x| x as i32).collect();
        assert_eq!(Solution::min_patches(nums, n), expected);
    }

    #[test]
    fn ex1() {
        test(&[1, 3], 6, 1)
    }

    #[test]
    fn ex2() {
        test(&[1, 5, 10], 20, 2)
    }

    #[test]
    fn ex3() {
        test(&[1, 2, 2], 5, 0)
    }

    #[test]
    fn discussion_case1() {
        // test 143
        test(
            &[
                1, 1, 2, 6, 9, 9, 12, 15, 16, 17, 17, 22, 23, 24, 26, 28, 28, 28, 29, 30, 31, 33,
                34, 34, 41, 42, 45, 52, 52, 52, 52, 53, 54, 67, 67, 73, 74, 74, 76, 79, 80, 81, 82,
                83, 83, 84, 86, 87, 90, 91, 91, 93, 93, 94, 95, 96, 97, 97,
            ],
            21,
            1,
        )
    }

    #[test]
    fn discussion_case2() {
        test(&[10, 30, 36, 42, 50, 76, 87, 88, 91, 92], 54, 5)
    }

    #[test]
    fn myex1() {
        test(&[2, 4], 7, 1)
    }

    #[test]
    fn myex2() {
        test(&[1], 31, 4)
    }

    #[test]
    fn myex3() {
        test(&[1], i32::MAX, 30)
    }
}
