// https://leetcode.com/problems/minimum-falling-path-sum-ii/

pub struct Solution;

// GitHub Copilot sol'n
// impl Solution {
//     pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
//         let n = grid.len();
//         let m = grid[0].len();
//         let mut dp = vec![vec![0; m]; n];
//         let mut min1 = 0;
//         let mut min2 = 0;
//         let mut min_index = 0;
//         for i in 0..n {
//             let mut new_min1 = i32::MAX;
//             let mut new_min2 = i32::MAX;
//             let mut new_min_index = 0;
//             for j in 0..m {
//                 dp[i][j] = grid[i][j] + if j == min_index { min2 } else { min1 };
//                 if dp[i][j] < new_min1 {
//                     new_min2 = new_min1;
//                     new_min1 = dp[i][j];
//                     new_min_index = j;
//                 } else if dp[i][j] < new_min2 {
//                     new_min2 = dp[i][j];
//                 }
//             }
//             min1 = new_min1;
//             min2 = new_min2;
//             min_index = new_min_index;
//         }
//         min1
//     }
// }

// My sol'n
impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        assert!(n > 0);
        assert!(n <= 200);
        let n = n as u8;
        let mut mins = [0i16, 0i16];
        let mut min_index = 0;
        for row in grid {
            let mut new_mins = [i16::MAX, i16::MAX];
            let mut new_min_index = 0;
            for j in 0..n {
                let el = row[j as usize] as i16 + if j == min_index { mins[1] } else { mins[0] };
                if el < new_mins[0] {
                    new_mins = [el, new_mins[0]];
                    new_min_index = j;
                } else if el < new_mins[1] {
                    new_mins[1] = el;
                }
            }
            mins = new_mins;
            min_index = new_min_index;
        }
        mins[0] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid: &[&[i32]], expected: i32) {
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(Solution::min_falling_path_sum(grid), expected);
    }

    #[test]
    fn ex1() {
        test(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]], 13)
    }

    #[test]
    fn ex2() {
        test(&[&[7]], 7)
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                &[-73, 61, 43, -48, -36],
                &[3, 30, 27, 57, 10],
                &[96, -76, 84, 59, -15],
                &[5, -49, 76, 31, -7],
                &[97, 91, 61, -46, 67],
            ],
            -192,
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                &[-37, 51, -36, 34, -22],
                &[82, 4, 30, 14, 38],
                &[-68, -52, -92, 65, -85],
                &[-49, -3, -77, 8, -19],
                &[-60, -71, -21, -62, -73],
            ],
            -268,
        )
    }
}
