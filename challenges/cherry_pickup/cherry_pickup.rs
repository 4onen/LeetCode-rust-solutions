// https://leetcode.com/problems/cherry-pickup/

pub struct Solution;

// DFS Sol'n
// impl Solution {
//     pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
//         fn dfs(
//             grid: &[Vec<i32>],
//             dp: &mut [Vec<Vec<Option<i16>>>],
//             sidelength: i8,
//             robot1_row: i8,
//             robot1_col: i8,
//             robot2_col: i8,
//         ) -> i16 {
//             let robot2_row = robot1_row + robot1_col - robot2_col;
//             if robot1_row >= sidelength
//                 || robot1_col >= sidelength
//                 || robot2_row >= sidelength
//                 || robot2_col >= sidelength
//             {
//                 return i16::MIN;
//             }
//             let robot1_val = grid[robot1_row as usize][robot1_col as usize] as i16;
//             let robot2_val = grid[robot2_row as usize][robot2_col as usize] as i16;
//             if robot1_val == -1 || robot2_val == -1 {
//                 i16::MIN
//             } else if robot1_row == sidelength - 1 && robot1_col == sidelength - 1 {
//                 robot1_val
//             } else if let Some(x) =
//                 dp[robot1_row as usize][robot1_col as usize][robot2_col as usize]
//             {
//                 x
//             } else {
//                 let mut res = robot1_val;
//                 if robot1_col != robot2_col {
//                     res += robot2_val;
//                 }
//                 res += std::cmp::max(
//                     std::cmp::max(
//                         dfs(grid, dp, sidelength, robot1_row + 1, robot1_col, robot2_col),
//                         dfs(grid, dp, sidelength, robot1_row, robot1_col + 1, robot2_col),
//                     ),
//                     std::cmp::max(
//                         dfs(
//                             grid,
//                             dp,
//                             sidelength,
//                             robot1_row + 1,
//                             robot1_col,
//                             robot2_col + 1,
//                         ),
//                         dfs(
//                             grid,
//                             dp,
//                             sidelength,
//                             robot1_row,
//                             robot1_col + 1,
//                             robot2_col + 1,
//                         ),
//                     ),
//                 );
//                 dp[robot1_row as usize][robot1_col as usize][robot2_col as usize] = Some(res);
//                 res
//             }
//         }
//         let sidelength = grid.len() as i8;
//         let mut dp =
//             vec![vec![vec![None; sidelength as usize]; sidelength as usize]; sidelength as usize];
//         let res = dfs(&grid, &mut dp, sidelength, 0, 0, 0);
//         std::cmp::max(0, res) as i32
//     }
// }

// BFS Sol'n
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let sidelength = grid.len() as i8;
        let mut prev = vec![vec![0i16; sidelength as usize]; sidelength as usize];
        let mut curr = vec![vec![0i16; sidelength as usize]; sidelength as usize];
        prev[0][0] = grid[0][0] as i16;
        for sum in 1..(sidelength * 2 - 1) {
            for robot1_col in (0..sidelength).rev() {
                for robot2_col in (0..sidelength).rev() {
                    let robot1_row = sum - robot1_col;
                    let robot2_row = sum - robot2_col;
                    if robot1_row < 0
                        || robot1_row >= sidelength
                        || robot2_row < 0
                        || robot2_row >= sidelength
                    {
                        curr[robot1_col as usize][robot2_col as usize] = i16::MIN;
                        continue;
                    }
                    let robot1_val = grid[robot1_row as usize][robot1_col as usize] as i16;
                    let robot2_val = grid[robot2_row as usize][robot2_col as usize] as i16;
                    if robot1_val < 0 || robot2_val < 0 {
                        curr[robot1_col as usize][robot2_col as usize] = i16::MIN;
                        continue;
                    }
                    let mut ans = prev[robot1_col as usize][robot2_col as usize];
                    if robot1_col > 0 {
                        ans = std::cmp::max(ans, prev[robot1_col as usize - 1][robot2_col as usize])
                    }
                    if robot2_col > 0 {
                        ans = std::cmp::max(ans, prev[robot1_col as usize][robot2_col as usize - 1])
                    }
                    if robot1_col > 0 && robot2_col > 0 {
                        ans = std::cmp::max(
                            ans,
                            prev[robot1_col as usize - 1][robot2_col as usize - 1],
                        )
                    }
                    if ans >= 0 {
                        ans += robot1_val;
                        if robot1_col != robot2_col {
                            ans += robot2_val;
                        }
                    }
                    curr[robot1_col as usize][robot2_col as usize] = ans;
                }
            }
            std::mem::swap(&mut prev, &mut curr);
        }
        std::cmp::max(0, prev[sidelength as usize - 1][sidelength as usize - 1]) as i32
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn ex1() {
        let grid = vec![vec![0, 1, -1], vec![1, 0, -1], vec![1, 1, 1]];
        assert_eq!(Solution::cherry_pickup(grid), 5);
    }

    #[test]
    fn ex2() {
        let grid = vec![vec![1, 1, -1], vec![1, -1, 1], vec![-1, 1, 1]];
        assert_eq!(Solution::cherry_pickup(grid), 0);
    }

    #[test]
    fn discussion_case1() {
        let grid = vec![
            vec![1, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 1],
            vec![1, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 1, 1],
        ];
        assert_eq!(Solution::cherry_pickup(grid), 15);
    }

    #[test]
    fn discussion_case2() {
        let grid = vec![
            vec![0, 1, 1, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![-1, 1, 1, 1, -1],
            vec![0, 1, 1, 1, 0],
            vec![1, 0, -1, 0, 0],
        ];
        assert_eq!(Solution::cherry_pickup(grid), 11);
    }

    #[test]
    fn discussion_case3() {
        let grid = vec![
            vec![1, 1, -1, 1],
            vec![1, 1, -1, 1],
            vec![1, 1, -1, 1],
            vec![1, 1, -1, 1],
        ];
        assert_eq!(Solution::cherry_pickup(grid), 0);
    }

    #[test]
    fn discussion_case4() {
        let grid = vec![
            vec![1, 1, 1, 1, -1, -1, -1, 1, 0, 0],
            vec![1, 0, 0, 0, 1, 0, 0, 0, 1, 0],
            vec![0, 0, 1, 1, 1, 1, 0, 1, 1, 1],
            vec![1, 1, 0, 1, 1, 1, 0, -1, 1, 1],
            vec![0, 0, 0, 0, 1, -1, 0, 0, 1, -1],
            vec![1, 0, 1, 1, 1, 0, 0, -1, 1, 0],
            vec![1, 1, 0, 1, 0, 0, 1, 0, 1, -1],
            vec![1, -1, 0, 1, 0, 0, 0, 1, -1, 1],
            vec![1, 0, -1, 0, -1, 0, 0, 1, 0, 0],
            vec![0, 0, -1, 0, 1, 0, 1, 0, 0, 1],
        ];
        assert_eq!(Solution::cherry_pickup(grid), 22);
    }

    #[test]
    fn my_extrema_ex1() {
        let grid = vec![vec![1; 50]; 50];
        const EXPECTED: i32 = 50 + 49 + 49 + 48;
        assert_eq!(Solution::cherry_pickup(grid), EXPECTED);
    }

    #[test]
    fn myex1() {
        let grid = vec![
            vec![1, 1, 1, 0, 0],
            vec![1, 0, 1, 0, 0],
            vec![1, 1, 1, 0, 0],
            vec![0, 0, 0, 1, 1],
            vec![0, 0, 0, 1, 1],
        ];
        assert_eq!(Solution::cherry_pickup(grid), 12);
    }

    #[test]
    fn myex2() {
        let grid = vec![
            vec![1, 1, 1, 0, 0],
            vec![1, 0, 1, 0, 0],
            vec![1, 1, -1, 0, 0],
            vec![0, 0, 0, 1, 1],
            vec![0, 0, 0, 1, 1],
        ];
        assert_eq!(Solution::cherry_pickup(grid), 11);
    }

    #[test]
    fn myex3() {
        let grid = vec![
            vec![1, 1, 1, 0, 0],
            vec![1, 0, 1, 0, 0],
            vec![1, 1, -1, 1, 0],
            vec![0, 0, 0, 1, 1],
            vec![0, 0, 1, 1, 1],
        ];
        assert_eq!(Solution::cherry_pickup(grid), 13);
    }
}
