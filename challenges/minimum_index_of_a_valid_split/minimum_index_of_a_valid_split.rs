// https://leetcode.com/problems/minimum-index-of-a-valid-split/

pub struct Solution;

// Naive count-everything sol'n
// impl Solution {
//     pub fn minimum_index(nums: Vec<i32>) -> i32 {
//         let mut counts = std::collections::HashMap::new();
//         for &num in &nums {
//             *counts.entry(num).or_insert(0) += 1;
//         }
//         let mut left_counts = std::collections::HashMap::new();
//         let n = nums.len() as u32;
//         for (i, &num) in nums.iter().enumerate() {
//             let i = i as u32;
//             let left_count = left_counts.entry(num).or_insert(0);
//             *left_count += 1;
//             if *left_count > (i + 1) / 2 {
//                 let right_count = counts[&num] - *left_count;
//                 let right_len = n - i - 1;
//                 if right_count > right_len / 2 {
//                     return i as i32;
//                 }
//             }
//         }
//         -1
//     }
// }

// Majority vote dominant sol'n
impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let dominant = {
            let mut dominant = 0;
            let mut count = 0;
            for &num in &nums {
                if count == 0 {
                    dominant = num;
                    count = 1;
                } else if num == dominant {
                    count += 1;
                } else {
                    count -= 1;
                }
            }
            dominant
        };
        let total_count = nums.iter().filter(|&&num| num == dominant).count() as u32;
        let n = nums.len() as u32;
        let mut left_count = 0;
        for (i, &num) in nums.iter().enumerate() {
            let i = i as u32;
            if num == dominant {
                left_count += 1;
                if left_count > (i + 1) / 2 {
                    let right_count = total_count - left_count;
                    let right_len = n - i - 1;
                    if right_count > right_len / 2 {
                        return i as i32;
                    }
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100_000);
        let mut counts = std::collections::HashMap::new();
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 1_000_000_000);
            *counts.entry(num).or_insert(0) += 1;
        }
        assert!(counts.into_values().any(|count| count > nums.len() / 2));
        assert!(expected >= -1);
        assert!(expected < nums.len() as i32 - 1);
        assert_eq!(Solution::minimum_index(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 2, 2], 2)
    }

    #[test]
    fn ex2() {
        test(&[2, 1, 3, 1, 1, 1, 7, 1, 2, 1], 4)
    }

    #[test]
    fn ex3() {
        test(&[3, 3, 3, 3, 7, 2, 2], -1)
    }

    #[test]
    fn discussion_case1() {
        test(&[1], -1)
    }

    #[test]
    fn discussion_case2() {
        test(&[1, 2, 1, 1], 0)
    }

    #[test]
    fn myex1() {
        test(&[1; 100_000], 0);
    }
}
