// https://leetcode.com/problems/cherry-pickup-ii/

pub struct Solution;

// DFS Sol'n
// impl Solution {
//     pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
//         fn dfs(
//             grid: &[Vec<i32>],
//             dp: &mut [Vec<Vec<i16>>],
//             columns: i8,
//             robot_row: u8,
//             robot1_col: i8,
//             robot2_col: i8,
//         ) -> i16 {
//             let ans = dp[robot_row as usize][robot1_col as usize][robot2_col as usize];
//             if ans >= 0 {
//                 return ans;
//             }
//             let grid_row: &[i32] = &grid[robot_row as usize];
//             let mut ans = 0;
//             if robot_row < grid.len() as u8 - 1 {
//                 for i in -1i8..=1 {
//                     for j in -1i8..=1 {
//                         let nc2 = robot2_col as i8 + j;
//                         if nc2 >= columns {
//                             continue;
//                         }
//                         let nc1 = robot1_col as i8 + i;
//                         if nc1 < 0 || nc1 > nc2 {
//                             continue;
//                         }
//                         ans = std::cmp::max(ans, dfs(grid, dp, columns, robot_row + 1, nc1, nc2));
//                     }
//                 }
//             }
//             ans += grid_row[robot1_col as usize] as i16;
//             if robot1_col != robot2_col {
//                 ans += grid_row[robot2_col as usize] as i16;
//             }
//             dp[robot_row as usize][robot1_col as usize][robot2_col as usize] = ans;
//             ans
//         }
//         let rows = grid.len() as u8;
//         let columns = grid[0].len() as i8;
//         dfs(
//             &grid,
//             &mut vec![vec![vec![-1; columns as usize]; columns as usize]; rows as usize],
//             columns,
//             0,
//             0,
//             columns as i8 - 1,
//         ) as i32
//     }
// }

// Bfs Sol'n
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len() as u8;
        let columns = grid[0].len() as i8;
        let mut dp: Vec<Vec<Vec<i16>>> =
            vec![vec![vec![-1; columns as usize]; columns as usize]; rows as usize];
        let mut ans = (grid[0][0] as u8 + grid[0][columns as usize - 1] as u8) as u16;
        dp[0][0][columns as usize - 1] = ans as i16;
        for robot_row in 1..rows {
            for robot1_col in 0..columns - 1 {
                for robot2_col in robot1_col..columns {
                    let old_ans =
                        dp[robot_row as usize - 1][robot1_col as usize][robot2_col as usize];
                    if old_ans < 0 {
                        continue;
                    }
                    for i in -1i8..=1 {
                        let nc1 = robot1_col as i8 + i;
                        if nc1 < 0 {
                            continue;
                        }
                        for j in -1i8..=1 {
                            let nc2 = robot2_col as i8 + j;
                            if nc2 >= columns || nc1 > nc2 {
                                continue;
                            }
                            let new_ans = std::cmp::max(
                                dp[robot_row as usize][nc1 as usize][nc2 as usize],
                                old_ans
                                    + grid[robot_row as usize][nc1 as usize] as i16
                                    + if nc1 != nc2 {
                                        grid[robot_row as usize][nc2 as usize] as i16
                                    } else {
                                        0
                                    },
                            );
                            dp[robot_row as usize][nc1 as usize][nc2 as usize] = new_ans;
                            ans = std::cmp::max(ans, new_ans as u16);
                        }
                    }
                }
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::cherry_pickup(vec![
                vec![3, 1, 1],
                vec![2, 5, 1],
                vec![1, 5, 5],
                vec![2, 1, 1]
            ]),
            24
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::cherry_pickup(vec![
                vec![1, 0, 0, 0, 0, 0, 1],
                vec![2, 0, 0, 0, 0, 3, 0],
                vec![2, 0, 9, 0, 0, 0, 0],
                vec![0, 3, 0, 5, 4, 0, 0],
                vec![1, 0, 2, 3, 0, 0, 6],
            ]),
            28
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::cherry_pickup(vec![vec![1, 1], vec![1, 1]]), 4);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::cherry_pickup(vec![vec![1, 1], vec![1, 0]]), 3);
    }

    #[test]
    fn myex3() {
        assert_eq!(Solution::cherry_pickup(vec![vec![1, 1], vec![0, 0]]), 2);
    }

    #[test]
    fn myex4() {
        assert_eq!(Solution::cherry_pickup(vec![vec![1, 1], vec![0, 1]]), 3);
    }

    #[test]
    fn myex5() {
        assert_eq!(Solution::cherry_pickup(vec![vec![1, 0], vec![0, 0]]), 1);
    }

    #[test]
    fn myex6() {
        assert_eq!(Solution::cherry_pickup(vec![vec![0, 1], vec![0, 0]]), 1);
    }

    #[test]
    fn myex7() {
        assert_eq!(Solution::cherry_pickup(vec![vec![0, 0], vec![0, 1]]), 1);
    }

    #[test]
    fn myex8() {
        assert_eq!(Solution::cherry_pickup(vec![vec![0, 0], vec![1, 0]]), 1);
    }

    #[test]
    fn my_extrema_ex1() {
        let input = vec![vec![100; 70]; 70];
        assert_eq!(Solution::cherry_pickup(input), 7000 * 2);
    }

    #[test]
    fn my_extrema_ex2() {
        let input = vec![vec![100; 70]; 2];
        assert_eq!(Solution::cherry_pickup(input), 400);
    }

    #[test]
    fn my_extrema_ex3() {
        let input = vec![vec![100; 2]; 70];
        assert_eq!(Solution::cherry_pickup(input), 7000 * 2);
    }

    #[test]
    fn my_extrema_ex4() {
        let input = vec![vec![100, 0]; 70];
        assert_eq!(Solution::cherry_pickup(input), 7000 * 1);
    }

    #[test]
    fn my_extrema_ex5() {
        let input = vec![vec![0, 100]; 70];
        assert_eq!(Solution::cherry_pickup(input), 7000 * 1);
    }

    #[test]
    fn discussion_case1() {
        let input = vec![
            vec![63, 56, 38, 100, 40, 50, 44, 98, 27, 20],
            vec![13, 98, 92, 31, 46, 29, 81, 2, 37, 3],
            vec![75, 5, 46, 15, 74, 17, 34, 60, 100, 44],
            vec![8, 4, 17, 14, 60, 77, 23, 0, 93, 83],
            vec![41, 40, 5, 2, 73, 80, 71, 100, 82, 39],
            vec![96, 76, 56, 42, 65, 65, 22, 11, 85, 80],
            vec![64, 71, 27, 78, 85, 15, 2, 28, 75, 31],
            vec![51, 16, 30, 65, 54, 68, 12, 5, 48, 1],
            vec![100, 57, 93, 43, 40, 51, 3, 82, 46, 91],
            vec![81, 63, 20, 12, 83, 59, 59, 46, 67, 66],
        ];
        assert_eq!(Solution::cherry_pickup(input), 1400);
    }

    #[test]
    fn discussion_case2() {
        let input = vec![
            vec![4, 0, 0, 0, 0, 0, 0, 0, 0, 86],
            vec![0, 52, 0, 0, 0, 0, 0, 0, 48, 0],
            vec![0, 0, 94, 0, 0, 0, 0, 74, 0, 0],
            vec![0, 0, 0, 98, 0, 0, 25, 0, 0, 0],
            vec![0, 0, 0, 0, 70, 66, 0, 0, 0, 0],
        ];
        assert_eq!(Solution::cherry_pickup(input), 617);
    }

    #[test]
    fn discussion_case3() {
        let input = vec![
            vec![0, 40, 0, 0, 0, 0, 0, 0, 93, 0],
            vec![0, 0, 41, 0, 0, 0, 0, 59, 0, 0],
            vec![89, 0, 0, 0, 0, 0, 0, 0, 0, 28],
            vec![0, 32, 0, 0, 0, 0, 0, 0, 80, 0],
            vec![0, 0, 40, 0, 0, 0, 0, 0, 0, 0],
            vec![50, 0, 0, 0, 0, 0, 0, 0, 0, 66],
            vec![0, 90, 0, 0, 0, 0, 0, 0, 11, 0],
            vec![0, 0, 62, 0, 0, 0, 0, 12, 0, 0],
            vec![95, 0, 0, 0, 0, 0, 0, 0, 0, 88],
            vec![0, 31, 0, 0, 0, 0, 0, 0, 26, 0],
        ];
        assert_eq!(Solution::cherry_pickup(input), 686);
    }

    #[test]
    fn discussion_case4() {
        let input = vec![
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 2],
            vec![0, 6, 0, 0, 0, 0, 0, 0, 6, 0],
            vec![0, 0, 9, 0, 0, 0, 0, 3, 0, 0],
            vec![0, 0, 0, 8, 0, 0, 5, 0, 0, 0],
            vec![100, 0, 0, 0, 2, 3, 0, 0, 0, 100],
        ];
        assert_eq!(Solution::cherry_pickup(input), 227);
    }

    #[test]
    fn discussion_case5() {
        let input = vec![
            vec![9, 79],
            vec![49, 85],
            vec![54, 48],
            vec![37, 72],
            vec![68, 14],
            vec![53, 30],
            vec![65, 80],
            vec![94, 18],
            vec![89, 46],
            vec![7, 93],
        ];
        assert_eq!(Solution::cherry_pickup(input), 1090);
    }

    #[test]
    fn discussion_case6() {
        let input = vec![
            vec![13, 14, 37, 49, 64, 98, 4, 11, 47, 81],
            vec![71, 46, 50, 50, 10, 14, 35, 35, 52, 69],
        ];
        assert_eq!(Solution::cherry_pickup(input), 13 + 81 + 71 + 69);
    }
}
