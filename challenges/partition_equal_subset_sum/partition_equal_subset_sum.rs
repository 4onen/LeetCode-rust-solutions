// https://leetcode.com/problems/partition-equal-subset-sum/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn can_partition(nums: Vec<i32>) -> bool {
//         let sum = nums.iter().sum::<i32>();
//         if sum & 1 != 0 {
//             return false;
//         }
//         let target = (sum / 2) as u16; // Max sum 20k, max target 10k
//         let n = nums.len() as u8;
//         let mut dp = vec![vec![false; target as usize + 1]; n as usize + 1];
//         dp[0][0] = true;
//         for i in 1..=n {
//             for j in 0..=target as u16 {
//                 dp[i as usize][j as usize] = dp[i as usize - 1][j as usize];
//                 if j as i32 >= nums[i as usize - 1] {
//                     dp[i as usize][j as usize] |=
//                         dp[i as usize - 1][j as usize - nums[i as usize - 1] as usize];
//                 }
//             }
//         }
//         dp[n as usize][target as usize]
//     }
// }

// Skip j too small for inner pass (slower)
// impl Solution {
//     pub fn can_partition(nums: Vec<i32>) -> bool {
//         let sum = nums.iter().sum::<i32>();
//         if sum & 1 != 0 {
//             return false;
//         }
//         let target = (sum / 2) as u16; // Max sum 20k, max target 10k
//         let n = nums.len() as u8;
//         let mut dp = vec![vec![false; target as usize + 1]; n as usize + 1];
//         dp[0][0] = true;
//         for i in 1..=n {
//             for j in 0..=target as i32 {
//                 dp[i as usize][j as usize] = dp[i as usize - 1][j as usize];
//             }
//             for j in nums[i as usize - 1]..=target as i32 {
//                 dp[i as usize][j as usize] |=
//                     dp[i as usize - 1][j as usize - nums[i as usize - 1] as usize];
//             }
//         }
//         dp[n as usize][target as usize]
//     }
// }

// Only one dp row alloc
// impl Solution {
//     pub fn can_partition(nums: Vec<i32>) -> bool {
//         let sum = nums.iter().sum::<i32>();
//         if sum & 1 != 0 {
//             return false;
//         }
//         let target = (sum / 2) as u16; // Max sum 20k, max target 10k
//         let mut dp = vec![false; target as usize + 1];
//         dp[0] = true;
//         for num in nums {
//             for j in (num..=target as i32).rev() {
//                 dp[j as usize] |= dp[j as usize - num as usize];
//             }
//         }
//         dp[target as usize]
//     }
// }

// Alter iteration variables to compare with zero (Most of the speedup here!)
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum & 1 != 0 {
            return false;
        }
        let target = (sum / 2) as u16; // Max sum 20k, max target 10k
        let mut dp = vec![false; target as usize + 1];
        dp[0] = true;
        for num in nums {
            for j in (0..=target as i32 - num).rev() {
                dp[j as usize + num as usize] |= dp[j as usize];
            }
        }
        dp[target as usize]
    }
}

// conditional write instead of always OR-ing (slower)
// impl Solution {
//     pub fn can_partition(nums: Vec<i32>) -> bool {
//         let sum = nums.iter().sum::<i32>();
//         if sum & 1 != 0 {
//             return false;
//         }
//         let target = (sum / 2) as u16; // Max sum 20k, max target 10k
//         let mut dp = vec![false; target as usize + 1];
//         dp[0] = true;
//         for num in nums {
//             for j in (0..=target as i32 - num).rev() {
//                 if dp[j as usize] {
//                     dp[j as usize + num as usize] = true;
//                 }
//             }
//         }
//         dp[target as usize]
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[u8], expected: bool) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 200);
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 100);
        }
        assert_eq!(
            Solution::can_partition(nums.iter().map(|&x| x as i32).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 5, 11, 5], true)
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 3, 5], false)
    }

    #[test]
    fn discussion_case1() {
        test(&[23, 13, 11, 7, 6, 5, 5], true)
    }

    #[test]
    fn discussion_case2() {
        test(&[1, 1], true)
    }

    #[test]
    fn discussion_case3() {
        test(
            &[6, 14, 19, 10, 17, 10, 8, 15, 16, 1, 12, 4, 9, 2, 15],
            true,
        )
    }

    #[test]
    fn discussion_case4() {
        test(&[4, 10, 7, 9, 7, 1, 11, 9, 13, 15], true)
    }

    #[test]
    fn discussion_case5() {
        test(&[9, 10, 15, 3, 9, 2, 9, 10, 13, 1], false)
    }

    #[test]
    fn discussion_case6() {
        test(
            &[
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 99, 97,
            ],
            false,
        )
    }

    #[test]
    fn discussion_case7() {
        test(
            &[
                5, 79, 2, 4, 8, 16, 32, 64, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
                100, 100, 100,
            ],
            true,
        )
    }

    #[test]
    fn discussion_case8() {
        test(&[3, 3, 3, 4, 5], true)
    }

    #[test]
    fn discussion_case9() {
        test(
            &[
                48, 23, 81, 49, 89, 4, 29, 58, 61, 91, 26, 79, 81, 48, 64, 1, 81, 2, 36, 82, 16,
                97, 23, 56, 50, 48, 74, 50, 74, 81, 10, 5, 76, 66, 77, 16, 19, 57, 88, 89, 98, 31,
                16, 99, 58, 84, 63, 100, 79, 66, 15, 62, 68, 54, 29, 74, 53, 83, 5, 34, 42, 28, 84,
                62, 64, 90, 29, 56, 14, 69, 9, 2, 45, 1, 53, 5, 51, 9, 42, 97, 33, 42, 55, 25, 96,
                7, 86, 79, 1, 42, 94, 43, 2, 16, 51, 21, 40, 19, 12,
            ],
            true,
        )
    }

    #[test]
    fn discussion_case10() {
        test(&[5, 5, 10, 1], false)
    }

    #[test]
    fn myex1() {
        test(&[100, 1], false)
    }
}
