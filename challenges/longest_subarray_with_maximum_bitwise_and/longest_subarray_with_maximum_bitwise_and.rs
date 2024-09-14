// https://leetcode.com/problems/longest-subarray-with-maximum-bitwise-and/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn longest_subarray(nums: Vec<i32>) -> i32 {
//         let mut max = 0;
//         let mut len = 0;
//         let mut max_len = 0;
//         for n in nums {
//             if n < max {
//                 len = 0;
//             } else if n == max {
//                 len += 1;
//                 max_len = max_len.max(len);
//             } else {
//                 max = n;
//                 len = 1;
//                 max_len = 1;
//             }
//         }
//         max_len
//     }
// }

// Near-Const sol'n
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut len = 1;
        let mut max_len = 1;
        let mut i = 1;
        while i < nums.len() {
            if nums[i] < max {
                len = 0;
            } else if nums[i] == max {
                len += 1;
                max_len = max_len.max(len);
            } else {
                max = nums[i];
                len = 1;
                max_len = 1;
            }
            i += 1;
        }
        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i32) {
        assert_eq!(Solution::longest_subarray(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(
            &[1,2,3,3,2,2],
            2
        )
    }

    #[test]
    fn ex2() {
        test(
            &[1,2,3,4],
            1
        )
    }
}
