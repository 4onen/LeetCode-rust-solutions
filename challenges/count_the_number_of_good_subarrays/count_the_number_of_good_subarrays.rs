// https://leetcode.com/problems/count-the-number-of-good-subarrays/

pub struct Solution;

// First Naive sol'n
// impl Solution {
//     pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
//         const fn n_choose_2(n: u32) -> u32 {
//             if n < 2 {
//                 return 0;
//             } else if n == 2 {
//                 return 1;
//             }
//             let Some(numerator) = n.checked_mul(n - 1) else {
//                 return u32::MAX;
//             };
//             numerator / 2
//         }
//         assert!(k > 0);
//         let k = k as u32;
//         let mut count = 0;
//         let mut counts = std::collections::HashMap::new();
//         for i in 0..nums.len() {
//             counts.clear();
//             let mut paircount = 0;
//             for j in i..nums.len() {
//                 let entry = counts.entry(nums[j]).or_insert(0);
//                 paircount -= n_choose_2(*entry);
//                 *entry += 1;
//                 paircount += n_choose_2(*entry);
//                 if paircount >= k {
//                     count += 1;
//                 }
//             }
//         }
//         count
//     }
// }

// Optimization: difference when new element counted
// impl Solution {
//     pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
//         assert!(k > 0);
//         let k = k as u32;
//         let mut count = 0;
//         let mut counts = std::collections::HashMap::new();
//         for i in 0..nums.len() {
//             counts.clear();
//             let mut paircount = 0;
//             for j in i..nums.len() {
//                 let entry = counts.entry(nums[j]).or_insert(0);
//                 paircount += *entry; // pair with every prev entry
//                 *entry += 1;
//                 if paircount >= k {
//                     count += 1;
//                 }
//             }
//         }
//         count
//     }
// }

// Optimization: Sliding window
impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        assert!(k > 0);
        let k = k as u32;
        let mut count = 0;
        let mut counts = std::collections::HashMap::new();
        let mut paircount = 0;
        let mut j = 0;
        for i in 0..nums.len() as u32 {
            let entry = counts.entry(nums[i as usize]).or_insert(0);
            paircount += *entry;
            *entry += 1;
            while paircount >= k {
                count += (nums.len() as u32 - i) as i64;
                let entry = counts.entry(nums[j as usize]).or_insert(0);
                *entry -= 1;
                paircount -= *entry;
                j += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], k: i32, expected: i64) {
        assert_eq!(
            Solution::count_good(nums.iter().map(|&x| x as i32).collect(), k as i32),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 1, 1, 1, 1], 10, 1)
    }

    #[test]
    fn ex2() {
        test(&[3, 1, 4, 3, 2, 2, 4], 2, 4)
    }

    #[test]
    fn discussion_case1() {
        test(&[3, 1, 4, 3, 3, 3, 4], 2, 9)
    }

    #[test]
    fn discussion_case2() {
        test(&[1, 2, 2, 4, 1, 9, 10, 11, 12, 13, 14], 2, 7)
    }

    #[test]
    fn my_extreme_ex10() {
        test(&[1; 10], 1, 45)
    }

    #[test]
    fn my_extreme_ex100() {
        test(&[1; 100], 1, 4950)
    }

    #[test]
    fn my_extreme_ex1000() {
        test(&[1; 1_000], 1, 499500)
    }
    #[test]
    fn my_extreme_ex10000() {
        test(&[1; 10_000], 1, 49995000)
    }

    #[test]
    fn my_extreme_ex100000() {
        test(&[1; 100_000], 1, 4999950000)
    }
}
