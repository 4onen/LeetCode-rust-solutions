// https://leetcode.com/problems/first-missing-positive/description/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
//         assert!(nums.len() <= 100_000);
//         let mut i = 0;
//         while i < nums.len() {
//             if nums[i] < 1 {
//                 nums.swap_remove(i);
//             } else {
//                 i += 1;
//             }
//         }
//         // nums[] now only contains positive elements.
//         // Sort them, dedup, then binary search for the first missing positive.
//         nums.sort_unstable();
//         nums.dedup();
//         match nums.len() {
//             0 => 1,
//             1..=100_000 => {
//                 let mut left: u32 = 0;
//                 let mut right: u32 = nums.len() as u32;
//                 while left < right {
//                     // Because nums.len() <= 100_000, cheap mid will never overflow.
//                     let mid = (left + right) / 2;
//                     if nums[mid as usize] as u32 == mid + 1 {
//                         left = mid + 1;
//                     } else {
//                         right = mid;
//                     }
//                 }
//                 (left + 1) as i32
//             }
//             _ => unreachable!("nums.len() > 100_000"),
//         }
//     }
// }

// Improve swap_remove to exclude numbers that clearly don't belong.
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        assert!(nums.len() <= 100_000);
        let mut i = 0 as i32;
        while i < nums.len() as i32 {
            if nums[i as usize] < 1 || nums[i as usize] > nums.len() as i32 {
                nums.swap_remove(i as usize);
            } else {
                i += 1;
            }
        }
        // nums[] now only contains positive elements.
        // Sort them, dedup, then binary search for the first missing positive.
        nums.sort_unstable();
        nums.dedup();
        match nums.len() {
            0 => 1,
            1..=100_000 => {
                let mut left: u32 = 0;
                let mut right: u32 = nums.len() as u32;
                while left < right {
                    // Because nums.len() <= 100_000, cheap mid will never overflow.
                    let mid = (left + right) / 2;
                    if nums[mid as usize] as u32 == mid + 1 {
                        left = mid + 1;
                    } else {
                        right = mid;
                    }
                }
                (left + 1) as i32
            }
            _ => unreachable!("nums.len() > 100_000"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let nums = [1, 2, 0];
        assert_eq!(Solution::first_missing_positive(nums.to_vec()), 3);
    }

    #[test]
    fn ex2() {
        let nums = [3, 4, -1, 1];
        assert_eq!(Solution::first_missing_positive(nums.to_vec()), 2);
    }

    #[test]
    fn ex3() {
        let nums = [7, 8, 9, 11, 12];
        assert_eq!(Solution::first_missing_positive(nums.to_vec()), 1);
    }

    #[test]
    fn my_minimum_ex1() {
        let nums = [1];
        assert_eq!(Solution::first_missing_positive(nums.to_vec()), 2);
    }

    #[test]
    fn my_minimum_ex2() {
        let nums = [2];
        assert_eq!(Solution::first_missing_positive(nums.to_vec()), 1);
    }

    #[test]
    fn my_minimum_ex3() {
        let nums = [1, 2];
        assert_eq!(Solution::first_missing_positive(nums.to_vec()), 3);
    }

    #[test]
    fn my_minimum_ex4() {
        let nums = [2, 1];
        assert_eq!(Solution::first_missing_positive(nums.to_vec()), 3);
    }

    #[test]
    fn my_minimum_ex5() {
        let nums = [1, 3];
        assert_eq!(Solution::first_missing_positive(nums.to_vec()), 2);
    }

    #[test]
    fn my_extreme_ex1() {
        let nums = [1; 100_000];
        assert_eq!(Solution::first_missing_positive(nums.to_vec()), 2);
    }

    #[test]
    fn my_extreme_ex2() {
        let nums = [2; 100_000];
        assert_eq!(Solution::first_missing_positive(nums.to_vec()), 1);
    }

    #[test]
    fn my_extreme_ex3() {
        let nums = (1..=100_000).collect::<Vec<_>>();
        assert_eq!(Solution::first_missing_positive(nums.to_vec()), 100_001);
    }

    #[test]
    fn discussion_case1() {
        let nums = [1, 4, 5];
        assert_eq!(Solution::first_missing_positive(nums.to_vec()), 2);
    }

    #[test]
    fn discussion_case2() {
        let nums = [6, 7, 8, 9, 11];
        assert_eq!(Solution::first_missing_positive(nums.to_vec()), 1);
    }
}
