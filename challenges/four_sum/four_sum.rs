// https://leetcode.com/problems/4sum/

pub struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // Because of a rogue test case,
        // we need to handle the case where
        // the number of elements in nums is
        // less than 4.
        if nums.len() < 4 {
            return vec![];
        }

        let mut nums = nums;
        nums.sort_unstable();
        Self::k_sum(4, &nums, target as i64)
    }

    pub fn k_sum(k: u8, nums: &[i32], target: i64) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        match k {
            0 => {}
            1 => {
                if let Ok(index) = nums.binary_search(&(target as i32)) {
                    result.push(vec![nums[index]]);
                }
            }
            2 => {
                let mut left = 0;
                let mut right = nums.len() - 1;
                while left < right {
                    let sum = nums[left] as i64 + nums[right] as i64;
                    match sum.cmp(&target) {
                        std::cmp::Ordering::Equal => {
                            if let Some(last) = result.last() {
                                if last[0] == nums[left] && last[1] == nums[right] {
                                    left += 1;
                                    right -= 1;
                                    continue;
                                }
                            }
                            result.push(vec![nums[left], nums[right]]);
                            left += 1;
                            right -= 1;
                        }
                        std::cmp::Ordering::Less => {
                            left += 1;
                        }
                        std::cmp::Ordering::Greater => {
                            right -= 1;
                        }
                    }
                }
            }
            _ => {
                for i in 0..nums.len() - k as usize + 1 {
                    if i == 0 || nums[i] != nums[i - 1] {
                        let mut sub_result =
                            Self::k_sum(k - 1, &nums[i + 1..], target - nums[i] as i64);
                        for sub in sub_result.iter_mut() {
                            sub.push(nums[i]);
                        }
                        result.append(&mut sub_result);
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_and_sort(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Solution::four_sum(nums, target);
        result.iter_mut().for_each(|v| v.sort_unstable());
        result.sort_unstable();
        result
    }

    #[test]
    fn ex1() {
        let nums = vec![1, 0, -1, 0, -2, 2];
        let target = 0;
        let expected = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];
        let result = run_and_sort(nums, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn ex2() {
        let nums = vec![2, 2, 2, 2, 2];
        let target = 8;
        let expected = vec![vec![2, 2, 2, 2]];
        let result = run_and_sort(nums, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn discussion_case1() {
        let nums = vec![-2, -1, 0, 0, 3];
        let target = 0;
        let expected = vec![vec![-2, -1, 0, 3]];
        let result = run_and_sort(nums, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn discussion_case2() {
        let nums = vec![-3, -1, 0, 2, 4, 5];
        let target = 1;
        let expected = vec![vec![-3, -1, 0, 5]];
        let result = run_and_sort(nums, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn discussion_case3() {
        let nums = vec![-5, 5, 4, -3, 0, 0, 4, -2];
        let target = 4;
        let expected = vec![vec![-5, 0, 4, 5], vec![-3, -2, 4, 5]];
        let result = run_and_sort(nums, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn discussion_case4() {
        let nums = vec![
            -1000000000,
            -1000000000,
            1000000000,
            -1000000000,
            -1000000000,
        ];
        let target = 294967296;
        let expected: Vec<Vec<i32>> = vec![];
        let result = run_and_sort(nums, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn discussion_case5() {
        let nums = vec![
            -1000000000,
            -1000000000,
            1000000000,
            -1000000000,
            -1000000000,
        ];
        let target = -294967296;
        let expected: Vec<Vec<i32>> = vec![];
        let result = run_and_sort(nums, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn my_extrema_ex1() {
        let nums = vec![
            1_000_000_000,
            -1_000_000_000,
            0,
            -1_000_000_000,
            1_000_000_000,
        ];
        let target = 1_000_000_000;
        let expected = vec![vec![-1_000_000_000, 0, 1_000_000_000, 1_000_000_000]];
        let result = run_and_sort(nums, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn failing_case1() {
        let nums = vec![0];
        let target = 0;
        let expected: Vec<Vec<i32>> = vec![];
        let result = run_and_sort(nums, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn myex1() {
        let nums = vec![-2, 0, 0, 2, 2];
        let target = 2;
        let expected = vec![vec![-2, 0, 2, 2]];
        let result = run_and_sort(nums, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn myex2() {
        let nums = vec![-2, 0, 2];
        let target = 0;
        let expected: Vec<Vec<i32>> = vec![];
        let result = run_and_sort(nums, target);
        assert_eq!(result, expected);
    }
}
