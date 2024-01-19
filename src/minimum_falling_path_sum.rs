// https://leetcode.com/problems/minimum-falling-path-sum/

pub struct Solution;

// Full DP
// impl Solution {
//     pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
//         let mut dp = vec![vec![0; matrix[0].len()]; matrix.len()];
//         for i in 0..matrix[0].len() {
//             dp[0][i] = matrix[0][i]
//         }
//         for i in 1..matrix.len() {
//             for j in 0..matrix[0].len() {
//                 let mut min = dp[i - 1][j];
//                 if j > 0 {
//                     min = min.min(dp[i - 1][j - 1]);
//                 }
//                 if j < matrix[0].len() - 1 {
//                     min = min.min(dp[i - 1][j + 1]);
//                 }
//                 dp[i][j] = min + matrix[i][j];
//             }
//         }
//         dp.last().unwrap().into_iter().min().unwrap().clone()
//     }
// }

// In-place DP
// impl Solution {
//     pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
//         for i in 1..matrix.len() {
//             for j in 0..matrix[0].len() {
//                 let mut min = matrix[i - 1][j];
//                 if j > 0 {
//                     min = min.min(matrix[i - 1][j - 1]);
//                 }
//                 if j < matrix[0].len() - 1 {
//                     min = min.min(matrix[i - 1][j + 1]);
//                 }
//                 matrix[i][j] += min;
//             }
//         }
//         matrix.last().unwrap().into_iter().min().unwrap().clone()
//     }
// }

// Deconstructing in-place DP
impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        let mut drain = matrix.drain(..);
        let mut prev = drain.next().unwrap();
        for mut row in drain {
            for j in 0..row.len() {
                let mut min = prev[j];
                if j > 0 {
                    min = min.min(prev[j - 1]);
                }
                if j < row.len() - 1 {
                    min = min.min(prev[j + 1]);
                }
                row[j] += min;
            }
            prev = row;
        }
        prev.into_iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
        assert_eq!(Solution::min_falling_path_sum(matrix), 13);
    }

    #[test]
    fn ex2() {
        let matrix = vec![vec![-19, 57], vec![-40, -5]];
        assert_eq!(Solution::min_falling_path_sum(matrix), -59);
    }
}
