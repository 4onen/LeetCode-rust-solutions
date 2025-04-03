// https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-ii/

pub struct Solution;

// Naive sol'n
// impl Solution {
//     pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
//         let mut result = 0;
//         for i in 0..nums.len() {
//             for j in i + 1..nums.len() {
//                 for k in j + 1..nums.len() {
//                     let v = (nums[i] - nums[j]) as i64 * nums[k] as i64;
//                     if v > result {
//                         result = v;
//                     }
//                 }
//             }
//         }
//         result
//     }
// }

// Eliminate k loop using suffix max pass
// impl Solution {
//     pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
//         let mut suffix_max = nums.clone();
//         for i in (0..nums.len() - 1).rev() {
//             suffix_max[i] = if suffix_max[i] < suffix_max[i + 1] {
//                 suffix_max[i + 1]
//             } else {
//                 suffix_max[i]
//             };
//         }
//         let mut result = 0;
//         for i in 0..nums.len() {
//             for j in i + 1..(nums.len() - 1) {
//                 let v = (nums[i] - nums[j]) as i64 * suffix_max[j + 1] as i64;
//                 if v > result {
//                     result = v;
//                 }
//             }
//         }
//         result
//     }
// }

// Eliminate i loop using prefix max
impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut prefix_max = nums.clone();
        for i in 1..nums.len() {
            prefix_max[i] = if prefix_max[i] > prefix_max[i - 1] {
                prefix_max[i]
            } else {
                prefix_max[i - 1]
            };
        }
        let mut suffix_max = nums.clone();
        for i in (0..nums.len() - 1).rev() {
            suffix_max[i] = if suffix_max[i] < suffix_max[i + 1] {
                suffix_max[i + 1]
            } else {
                suffix_max[i]
            };
        }
        let mut result = 0;
        for j in 1..(nums.len() - 1) {
            let v = (prefix_max[j - 1] - nums[j]) as i64 * suffix_max[j + 1] as i64;
            if v > result {
                result = v;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i64) {
        assert!(nums.len() >= 3);
        assert!(nums.len() <= 100_000);
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
