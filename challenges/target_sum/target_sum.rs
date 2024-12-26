// https://leetcode.com/problems/target-sum/

pub struct Solution;

// Naive sol'n
// impl Solution {
//     pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
//         fn rec(nums: &[i32], i: u8, sum: i32) -> u32 {
//             if i >= nums.len() as u8 {
//                 return (sum == 0) as u32;
//             }
//             if nums[i as usize] == 0 {
//                 return 2 * rec(nums, i + 1, sum);
//             }
//             return rec(nums, i + 1, sum + nums[i as usize])
//                 + rec(nums, i + 1, sum - nums[i as usize]);
//         }
//         rec(&nums, 0, target) as i32
//     }
// }

// Bounds checking
// impl Solution {
//     pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
//         fn rec(nums: &[i32], postfix_sum: &[i32], i: u8, sum: i32) -> u32 {
//             if i >= nums.len() as u8 {
//                 return (sum == 0) as u32;
//             }
//             if sum > postfix_sum[i as usize] {
//                 return 0;
//             }
//             if nums[i as usize] == 0 {
//                 return 2 * rec(nums, postfix_sum, i + 1, sum);
//             }
//             return rec(nums, postfix_sum, i + 1, sum + nums[i as usize])
//                 + rec(nums, postfix_sum, i + 1, sum - nums[i as usize]);
//         }
//         let postfix_sum = {
//             let mut postfix_sum = vec![0; nums.len()];
//             postfix_sum[nums.len() - 1] = nums[nums.len() - 1];
//             for i in (0..(nums.len() - 1) as u8).rev() {
//                 postfix_sum[i as usize] = postfix_sum[(i + 1) as usize] + nums[i as usize]
//             }
//             postfix_sum
//         };
//         if postfix_sum[0] < target || (target < 0 && postfix_sum[0] < -target) {
//             return 0;
//         }
//         rec(&nums, &postfix_sum, 0, target) as i32
//     }
// }

// Add-only sol'n (not working)
// impl Solution {
//     pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
//         fn rec(nums: &[i32], postfix_zeros: &[u8], i: u8, remaining: i32) -> u32 {
//             if i >= nums.len() as u8 {
//                 return 0;
//             }
//             if nums[i as usize] > remaining {
//                 return 0;
//             } else if nums[i as usize] == remaining {
//                 return 2u32.pow(postfix_zeros[i as usize] as u32);
//             } else {
//                 // nums[i] > remaining
//                 return rec(nums, postfix_zeros, i + 1, remaining - 2 * nums[i as usize])
//                     + rec(nums, postfix_zeros, i + 1, remaining - nums[i as usize]);
//             }
//         }
//         let num_sum = nums.iter().sum::<i32>() + target;
//         if num_sum % 2 != 0 {
//             return 0;
//         }
//         let mut postfix_zeros = (0..nums.len() as u8)
//             .rev()
//             .scan(0, |zeros, val| {
//                 *zeros += (nums[val as usize] == 0) as u8;
//                 Some(*zeros)
//             })
//             .collect::<Vec<_>>();
//         postfix_zeros.reverse();
//         rec(&nums, &postfix_zeros, 0, num_sum + target) as i32
//     }
// }

// Best solution adaptation
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let num_sum = nums.iter().sum::<i32>();
        if num_sum < target || -num_sum > target {
            // Impossible
            return 0;
        }
        let num_sum = num_sum + target;
        if num_sum % 2 == 1 {
            return 0;
        }
        let num_sum = num_sum / 2;
        let mut dp = vec![0; num_sum as usize + 1];
        dp[0] = 1;
        for i in 1..=nums.len() {
            for j in (0..=num_sum as usize).rev() {
                if j >= nums[i - 1] as usize {
                    dp[j] += dp[j - nums[i - 1] as usize]
                } else {
                    break;
                }
            }
        }
        dp[num_sum as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[u16], target: i32, expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 20);
        let mut sum = 0;
        for &num in nums {
            assert!(num <= 1000);
            sum += num;
        }
        assert!(sum <= 1000);
        assert!(target >= -1000);
        assert!(target <= 1000);
        assert_eq!(
            Solution::find_target_sum_ways(nums.iter().map(|&x| x as i32).collect(), target),
            expected
        );
        assert_eq!(
            Solution::find_target_sum_ways(nums.iter().map(|&x| x as i32).collect(), -target),
            expected
        );
    }

    #[test]
    fn ex0() {
        test(&[2, 1], 1, 1)
    }

    #[test]
    fn ex1() {
        test(&[1, 1, 1, 1, 1], 3, 5)
    }

    #[test]
    fn ex2() {
        test(&[1], 1, 1)
    }

    #[test]
    fn discussion_case1() {
        test(
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            1,
            2i32.pow(19),
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
            2i32.pow(20),
        )
    }
    #[test]
    fn discussion_case3() {
        test(&[1], 2, 0);
    }

    #[test]
    fn discussion_case4() {
        test(&[100, 100], -300, 0);
    }

    #[test]
    fn discussion_case5() {
        test(&[12, 25, 42, 49, 41, 15, 22, 34, 28, 31], 35, 8)
    }
}
