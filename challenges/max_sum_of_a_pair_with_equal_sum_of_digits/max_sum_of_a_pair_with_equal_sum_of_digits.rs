// https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn maximum_sum(nums: Vec<i32>) -> i32 {
//         const fn digit_sum(mut num: i32) -> u8 {
//             let mut res = 0;
//             while num > 0 {
//                 res += (num % 10) as u8;
//                 num /= 10;
//             }
//             res
//         }
//         let mut digit_sums_to_nums = std::collections::HashMap::new();
//         let mut max_sum = -1;
//         for num in nums {
//             let digit_sum = digit_sum(num);
//             if let Some(max_num) = digit_sums_to_nums.get_mut(&digit_sum) {
//                 let sum = *max_num + num;
//                 if max_sum < sum {
//                     max_sum = sum;
//                 }
//                 *max_num = num.max(*max_num);
//             } else {
//                 digit_sums_to_nums.insert(digit_sum, num);
//             }
//         }
//         max_sum
//     }
// }

// Array-based u8-key map sol'n
impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        const fn digit_sum(mut num: i32) -> u8 {
            let mut res = 0;
            while num > 0 {
                res += (num % 10) as u8;
                num /= 10;
            }
            res
        }
        let mut digit_sums_to_nums = [0; 100];
        let mut max_sum = -1;
        for num in nums {
            let key = digit_sum(num);
            if digit_sums_to_nums[key as usize] > 0 {
                let old_num = digit_sums_to_nums[key as usize];
                let sum = old_num + num;
                if max_sum < sum {
                    max_sum = sum;
                }
                if old_num < num {
                    digit_sums_to_nums[key as usize] = num;
                }
            } else {
                digit_sums_to_nums[key as usize] = num;
            }
        }
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100_000);
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 1_000_000_000);
        }
        assert_eq!(Solution::maximum_sum(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[18, 43, 36, 13, 7], 54)
    }

    #[test]
    fn ex2() {
        test(&[10, 12, 19, 14], -1)
    }
}
