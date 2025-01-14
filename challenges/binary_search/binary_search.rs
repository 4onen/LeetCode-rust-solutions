// https://leetcode.com/problems/binary-search/

pub struct Solution;

// Initial sol'n
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as u16;
        loop {
            if left >= right {
                if left >= nums.len() as u16 || nums[left as usize] != target {
                    break -1;
                } else {
                    break left as i32;
                }
            }
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                break mid as i32;
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], target: i32, expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 10_000);
        let mut seen = std::collections::HashSet::new();
        for &num in nums {
            assert!(num >= -10_000);
            assert!(num <= 10_000);
            assert!(seen.insert(num))
        }
        assert!(target >= -10_000);
        assert!(target <= 10_000);
        assert_eq!(Solution::search(nums.to_vec(), target), expected);
    }

    fn test_all(nums: &[i32]) {
        for i in 0..nums.len() as u16 {
            test(nums, nums[i as usize], i as i32)
        }
    }

    #[test]
    fn ex1() {
        test(&[-1, 0, 3, 5, 9, 12], 9, 4)
    }

    #[test]
    fn ex1_1() {
        test_all(&[-1, 0, 3, 5, 9, 12])
    }

    #[test]
    fn ex2() {
        test(&[-1, 0, 3, 5, 9, 12], 2, -1)
    }

    #[test]
    fn ex2_1() {
        test_all(&[-1, 0, 3, 5, 9, 12])
    }

    #[test]
    fn myex1() {
        test_all(&[0])
    }

    #[test]
    fn myex2() {
        test(&[0], -1, -1)
    }

    #[test]
    fn myex3() {
        test(&[0], 1, -1)
    }
}
