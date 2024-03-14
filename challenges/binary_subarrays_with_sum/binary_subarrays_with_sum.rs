// https://leetcode.com/problems/binary-subarrays-with-sum/

pub struct Solution;

// Copied from ../subarray_sum_equals_k/subarray_sum_equals_k.rs
// Adjusted types to match the problem's constraints
// impl Solution {
//     pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
//         assert!(nums.len() <= 30_000);
//         assert!(goal >= 0);
//         assert!(goal as usize <= nums.len());
//         let goal = goal as i16;
//         let mut count = 0u32;
//         let mut sum = 0i16;
//         let mut map = std::collections::HashMap::<i16, u16>::with_capacity(nums.len() + 1);
//         map.insert(0, 1);
//         for i in 0..nums.len() {
//             sum += nums[i] as i16;
//             if let Some(&v) = map.get(&(sum - goal)) {
//                 count += v as u32;
//             }
//             *map.entry(sum).or_insert(0) += 1;
//         }
//         count as i32
//     }
// }

// Skip goal subtractions by adjusting initial sum (BROKEN)
// impl Solution {
//     pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
//         assert!(nums.len() <= 30_000);
//         assert!(goal >= 0);
//         assert!(goal as usize <= nums.len());
//         let goal = goal as i16;
//         let mut count = 0u32;
//         let mut sum = 0i16;
//         let mut map = std::collections::HashMap::<i16, u16>::with_capacity(nums.len() + 1);
//         map.insert(goal, 1);
//         for i in 0..nums.len() {
//             sum += nums[i] as i16;
//             if let Some(&v) = map.get(&sum) {
//                 count += v as u32;
//             }
//             *map.entry(sum).or_insert(0) += 1;
//         }
//         count as i32
//     }
// }
// Yeah so turns out this only works if the goal is 0. Oops.

// Two pointer sol'n
impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        match goal {
            0 => {
                let mut count = 0;
                let mut this_zero_run = 0;
                for n in nums {
                    match n {
                        0 => this_zero_run += 1,
                        1 => {
                            count += (this_zero_run * (this_zero_run + 1)) / 2;
                            this_zero_run = 0;
                        }
                        _ => panic!("Invalid input, must be 0 or 1"),
                    }
                }
                count + (this_zero_run * (this_zero_run + 1)) / 2
            }
            1..=30_000 if (goal as u16 <= nums.len() as u16) => {
                let goal = goal as u16;
                let mut count: i32 = 0;
                let mut sum: u16 = nums[0] as u16;
                let mut left: u16 = 0;
                let mut right: u16 = 0;
                'outer: loop {
                    if right >= nums.len() as u16 {
                        break 'outer;
                    }
                    while sum < goal {
                        right += 1;
                        if right < nums.len() as u16 {
                            sum += nums[right as usize] as u16;
                        } else {
                            break 'outer;
                        }
                    }
                    let right_begin = right;
                    loop {
                        right += 1;
                        if right >= nums.len() as u16 || nums[right as usize] != 0 {
                            break;
                        }
                    }
                    let left_begin = left;
                    while left < right && nums[left as usize] == 0 {
                        left += 1;
                    }
                    left += 1;
                    count += (left - left_begin) as i32 * (right - right_begin) as i32;
                }
                count
            }
            _ => panic!("Invalid goal, must be 0 to nums.len()"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let nums = [1, 0, 1, 0, 1];
        let goal = 2;
        let expected = 4;
        assert_eq!(
            Solution::num_subarrays_with_sum(nums.to_vec(), goal),
            expected
        );
    }

    #[test]
    fn ex2() {
        let nums = [0, 0, 0, 0, 0];
        let goal = 0;
        let expected = 15;
        assert_eq!(
            Solution::num_subarrays_with_sum(nums.to_vec(), goal),
            expected
        );
    }

    #[test]
    fn failed_case1() {
        let nums = [0, 0, 0, 0, 1];
        let goal = 2;
        let expected = 0;
        assert_eq!(
            Solution::num_subarrays_with_sum(nums.to_vec(), goal),
            expected
        );
    }

    #[test]
    fn myex1() {
        let nums = [1, 0, 1, 0];
        let goal = 2;
        let expected = 2;
        assert_eq!(
            Solution::num_subarrays_with_sum(nums.to_vec(), goal),
            expected
        );
    }

    #[test]
    fn myex2() {
        let nums = [1, 0, 1];
        let goal = 2;
        let expected = 1;
        assert_eq!(
            Solution::num_subarrays_with_sum(nums.to_vec(), goal),
            expected
        );
    }
}
