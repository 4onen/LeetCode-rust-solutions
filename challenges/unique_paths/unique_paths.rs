// https://leetcode.com/problems/unique-paths/

pub struct Solution;

// Braindead 2D DP solution
// impl Solution {
//     pub fn unique_paths(m: i32, n: i32) -> i32 {
//         let mut dp = vec![1; m as usize * n as usize];
//         for i in 1..m as usize {
//             for j in 1..n as usize {
//                 dp[i * n as usize + j] = dp[(i - 1) * n as usize + j] + dp[i * n as usize + j - 1];
//             }
//         }
//         dp[m as usize * n as usize - 1]
//     }
// }

// Keeping only one row of DP
// impl Solution {
//     pub fn unique_paths(m: i32, n: i32) -> i32 {
//         let mut dp = vec![1; n as usize];
//         for _ in 1..m as usize {
//             for j in 1..n as usize {
//                 dp[j] += dp[j - 1];
//             }
//         }
//         dp[n as usize - 1]
//     }
// }

// Const-ified
impl Solution {
    pub const fn unique_paths(m: i32, n: i32) -> i32 {
        let n = n as u8;
        let m = m as u8;
        let mut dp = [1; 100];
        let mut i: u8 = 1;
        while i < m {
            let mut j: u8 = 1;
            while j < n {
                dp[j as usize] += dp[j as usize - 1];
                j += 1;
            }
            i += 1;
        }
        dp[n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(Solution::unique_paths(1, 1), 1);
    }
}
