// https://leetcode.com/problems/count-special-quadruplets/

pub struct Solution;

// Naive O(n^4) solution
// impl Solution {
//     pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
//         let mut count = 0;
//         for i in 0..nums.len() - 3 {
//             for j in i + 1..nums.len() - 2 {
//                 for k in j + 1..nums.len() - 1 {
//                     for l in k + 1..nums.len() {
//                         if nums[i] + nums[j] + nums[k] == nums[l] {
//                             count += 1;
//                         }
//                     }
//                 }
//             }
//         }
//         count
//     }
// }

// O(n^3) solution (store diff indices in hashmap)
// impl Solution {
//     pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
//         let mut diff_starts = std::collections::HashMap::new();
//         for i in 0..nums.len() as u8 - 1 {
//             for j in i + 1..nums.len() as u8 {
//                 diff_starts
//                     .entry(nums[j as usize] - nums[i as usize])
//                     .and_modify(|v: &mut Vec<_>| v.push(i))
//                     .or_insert(vec![i]);
//             }
//         }
//         let mut count = 0;
//         for i in 0..nums.len() as u8 - 1 {
//             for j in i + 1..nums.len() as u8 {
//                 let sum = nums[i as usize] + nums[j as usize];
//                 let Some(starts) = diff_starts.get(&sum) else {
//                     continue;
//                 };
//                 for &start in starts {
//                     if start > j {
//                         count += 1;
//                     }
//                 }
//             }
//         }
//         count
//     }
// }

// O(n^4) solution with right-sized types
// impl Solution {
//     pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
//         let mut count = 0;
//         for i in 0..nums.len() as u8 - 3 {
//             for j in i + 1..nums.len() as u8 - 2 {
//                 for k in j + 1..nums.len() as u8 - 1 {
//                     let sum = nums[i as usize] + nums[j as usize] + nums[k as usize];
//                     for l in k + 1..nums.len() as u8 {
//                         if sum == nums[l as usize] {
//                             count += 1;
//                         }
//                     }
//                 }
//             }
//         }
//         count
//     }
// }

// O(n^2) solution (store diff frequencies in hashmap)
impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let mut diff_counts = std::collections::HashMap::new();
        let mut count = 0;
        for i in (1..nums.len() as u8 - 1).rev() {
            for j in i + 1..nums.len() as u8 {
                *diff_counts
                    .entry(nums[j as usize] - nums[i as usize])
                    .or_insert(0) += 1;
            }
            let nexti = i - 1;
            for j in 0..nexti {
                count += diff_counts
                    .get(&(nums[nexti as usize] + nums[j as usize]))
                    .unwrap_or(&0);
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[u8], expected: u32) {
        assert!(nums.len() >= 4);
        assert!(nums.len() <= 50);
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 100);
        }
        assert_eq!(
            Solution::count_quadruplets(nums.iter().map(|&n| n as i32).collect()),
            expected as i32
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 3, 6], 1)
    }

    #[test]
    fn ex2() {
        test(&[3, 3, 6, 4, 5], 0)
    }

    #[test]
    fn ex3() {
        test(&[1, 1, 1, 3, 5], 4)
    }

    #[test]
    fn discussion_case1() {
        test(&[28, 8, 49, 85, 37, 90, 20, 8], 1)
    }

    #[test]
    fn discussion_case2() {
        test(&[25, 69, 18, 24, 60, 7, 49], 1)
    }
}
