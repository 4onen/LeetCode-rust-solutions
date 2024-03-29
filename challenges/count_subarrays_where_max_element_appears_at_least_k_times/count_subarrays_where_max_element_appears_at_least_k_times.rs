// https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/

pub struct Solution;

// Initial sliding window approach with inverted answer
// impl Solution {
//     pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
//         if k < 1 {
//             return nums.len() as i64 * (nums.len() as i64 + 1) / 2;
//         }
//         let max_ele = *nums.iter().max().unwrap();
//         let mut count = 0i64;
//         let mut sum = 0;
//         let mut left = 0;
//         for right in 0..nums.len() as i32 {
//             if nums[right as usize] == max_ele {
//                 sum += 1;
//                 while sum >= k {
//                     sum -= (nums[left as usize] == max_ele) as i32;
//                     left += 1;
//                 }
//             }
//             count += (right - left + 1) as i64;
//         }
//         nums.len() as i64 * (nums.len() as i64 + 1) / 2 - count
//     }
// }

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        if k < 1 {
            return nums.len() as i64 * (nums.len() as i64 + 1) / 2;
        }
        let max_ele = *nums.iter().max().unwrap();
        let mut count = 0i64;
        let mut sum = 0;
        let mut left = 0;
        for right in 0..nums.len() as i32 {
            if nums[right as usize] == max_ele {
                sum += 1;
                while sum >= k {
                    count += (nums.len() as i32 - right) as i64;
                    if nums[left] == max_ele {
                        sum -= 1;
                    }
                    left += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let nums = vec![1, 3, 2, 3, 3];
        let k = 2;
        assert_eq!(Solution::count_subarrays(nums, k), 6);
    }

    #[test]
    fn ex2() {
        let nums = vec![1, 4, 2, 1];
        let k = 3;
        assert_eq!(Solution::count_subarrays(nums, k), 0);
    }

    #[test]
    fn myex1() {
        let nums = vec![1, 1];
        let k = 1;
        assert_eq!(Solution::count_subarrays(nums, k), 3);
    }

    #[test]
    fn myex2() {
        let nums = vec![1, 2, 1];
        let k = 2;
        assert_eq!(Solution::count_subarrays(nums, k), 0);
    }

    #[test]
    fn myex3() {
        let nums = vec![1, 2, 1, 2];
        let k = 2;
        assert_eq!(Solution::count_subarrays(nums, k), 2);
    }

    #[test]
    fn myex4() {
        let nums = vec![1, 2, 1, 2, 1];
        let k = 2;
        assert_eq!(Solution::count_subarrays(nums, k), 4);
    }

    #[test]
    fn myex5() {
        let nums = vec![1, 2, 1, 2, 1, 2];
        let k = 2;
        assert_eq!(Solution::count_subarrays(nums, k), 8);
    }

    #[test]
    fn discussion_case1() {
        let nums = vec![1, 0, 3, 8, 8, 8, 7, 8, 7];
        let k = 2;
        assert_eq!(Solution::count_subarrays(nums, k), 26);
    }

    #[test]
    fn discussion_case2() {
        let nums = vec![1, 4, 4, 3, 4];
        let k = 2;
        assert_eq!(Solution::count_subarrays(nums, k), 7);
    }

    #[test]
    fn my_extreme_ex1() {
        let nums = vec![1; 100_000];
        let k = 1;
        assert_eq!(
            Solution::count_subarrays(nums, k),
            100_000i64 * (100_000i64 + 1i64) / 2i64
        );
    }

    #[test]
    fn my_extreme_ex2() {
        let mut nums = vec![1; 100_000];
        nums[0] = 2;
        let k = 2;
        assert_eq!(Solution::count_subarrays(nums, k), 0);
    }

    #[test]
    fn my_extreme_ex3() {
        let nums = vec![1; 100_000];
        let k = 2;
        let expected = 100_000i64 * (100_000i64 + 1i64) / 2i64 - nums.len() as i64;
        assert_eq!(Solution::count_subarrays(nums, k), expected);
    }

    #[test]
    fn failure_case1() {
        // Whoops! Forgot that if k=1, we can only count subarrays including
        // that largest element.
        let nums = vec![
            28, 5, 58, 91, 24, 91, 53, 9, 48, 85, 16, 70, 91, 91, 47, 91, 61, 4, 54, 61, 49,
        ];
        let k = 1;
        assert_eq!(Solution::count_subarrays(nums, k), 187);
    }
}
