// https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-i/

pub struct Solution;

// Naive brute force
// impl Solution {
//     pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
//         let mut max = 0;
//         for i in 0..nums.len() - 2 {
//             for j in i + 1..nums.len() - 1 {
//                 if nums[i] < nums[j] {
//                     continue;
//                 }
//                 for k in j + 1..nums.len() {
//                     let val = (nums[i] - nums[j]) as i64 * nums[k] as i64;
//                     if val > max {
//                         max = val;
//                     }
//                 }
//             }
//         }
//         max
//     }
// }

// Postfix max for k-index discovery
impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut max = 0;
        let mut postfix_max = vec![nums[nums.len() - 1]; nums.len()];
        for i in (0..nums.len() - 1).rev() {
            postfix_max[i] = std::cmp::max(nums[i], postfix_max[i + 1]);
        }
        for i in 0..nums.len() - 2 {
            for j in i + 1..nums.len() - 1 {
                if nums[i] < nums[j] {
                    continue;
                }
                let val = (nums[i] - nums[j]) as i64 * postfix_max[j + 1] as i64;
                if val > max {
                    max = val;
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i64) {
        assert!(nums.len() >= 3);
        assert!(nums.len() <= 100);
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 1_000_000);
        }
        assert_eq!(Solution::maximum_triplet_value(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[12, 6, 1, 2, 7], 77)
    }

    #[test]
    fn ex2() {
        test(&[1, 10, 3, 4, 19], 133)
    }

    #[test]
    fn ex3() {
        test(&[1, 2, 3], 0)
    }
}
