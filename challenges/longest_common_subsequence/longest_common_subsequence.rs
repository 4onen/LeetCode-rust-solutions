// https://leetcode.com/problems/longest-common-subsequence/

pub struct Solution;

// Raw DP sol'n
// impl Solution {
//     pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
//         let b1 = text1.as_bytes();
//         let b2 = text2.as_bytes();
//         let mut dp: Vec<Vec<u16>> = vec![vec![0; b2.len() + 1]; b1.len() + 1];
//         for i in 1..=text1.len() {
//             for j in 1..=text2.len() {
//                 if b1[i - 1] == b2[j - 1] {
//                     dp[i][j] = dp[i - 1][j - 1] + 1;
//                 } else {
//                     dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
//                 }
//             }
//         }
//         dp[text1.len()][text2.len()] as i32
//     }
// }

// Two-vec DP sol'n (fastest)
// impl Solution {
//     pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
//         let b1 = text1.as_bytes();
//         let b2 = text2.as_bytes();
//         let mut prev = vec![0u16; b2.len() + 1];
//         let mut curr = vec![0u16; b2.len() + 1];
//         for i in 1..=b1.len() {
//             for j in 1..=b2.len() {
//                 curr[j] = if b1[i - 1] == b2[j - 1] {
//                     prev[j - 1] + 1
//                 } else {
//                     std::cmp::max(prev[j], curr[j - 1])
//                 };
//             }
//             (prev, curr) = (curr, prev);
//         }
//         prev[text2.len()] as i32
//     }
// }

// Optimization of two-vec DP sol'n
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let b1 = text1.as_bytes();
        let b2 = text2.as_bytes();
        let mut prev = vec![0u16; b2.len() + 1];
        let mut curr = vec![0u16; b2.len() + 1];
        for i in 0..b1.len() {
            for j in 0..b2.len() {
                curr[j + 1] = if b1[i] == b2[j] {
                    prev[j] + 1
                } else {
                    std::cmp::max(prev[j + 1], curr[j])
                };
            }
            std::mem::swap(&mut prev, &mut curr);
        }
        prev[b2.len()] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()),
            3
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()),
            3
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "def".to_string()),
            0
        );
    }
}
