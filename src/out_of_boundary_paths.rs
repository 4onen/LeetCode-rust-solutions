// https://leetcode.com/problems/out-of-boundary-paths/

pub struct Solution;

// Recursive sol'n with memoization
impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        const MODULO: u32 = 1_000_000_007;
        assert!(m >= 0 && n >= 0 && max_move >= 0, "Invalid input");
        assert!(m <= 50 && n <= 50 && max_move <= 50, "Invalid input");
        assert!(
            start_row >= 0 && start_column >= 0,
            "Invalid input: start position must be non-negative"
        );
        assert!(
            start_row < m && start_column < n,
            "Invalid input: start position must be within the grid"
        );
        let mut dp = vec![vec![vec![-1; n as usize]; m as usize]; max_move as usize + 1];
        fn dfs(
            m: i8,
            n: i8,
            max_move: i8,
            start_row: i8,
            start_column: i8,
            dp: &mut [Vec<Vec<i32>>],
        ) -> u32 {
            if (!(0..m).contains(&start_row)) || (!(0..n).contains(&start_column)) {
                return 1;
            }
            if max_move == 0 {
                return 0;
            }
            if dp[max_move as usize][start_row as usize][start_column as usize] != -1 {
                return dp[max_move as usize][start_row as usize][start_column as usize] as u32;
            }
            let mut paths = 0;
            paths += dfs(m, n, max_move - 1, start_row - 1, start_column, dp);
            paths += dfs(m, n, max_move - 1, start_row + 1, start_column, dp);
            paths += dfs(m, n, max_move - 1, start_row, start_column - 1, dp);
            paths += dfs(m, n, max_move - 1, start_row, start_column + 1, dp);
            paths %= MODULO;
            dp[max_move as usize][start_row as usize][start_column as usize] = paths as i32;
            paths
        }
        dfs(
            m as i8,
            n as i8,
            max_move as i8,
            start_row as i8,
            start_column as i8,
            &mut dp,
        ) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::find_paths(2, 2, 2, 0, 0), 6);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::find_paths(1, 3, 3, 0, 1), 12);
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(Solution::find_paths(8, 50, 23, 5, 26), 914783380);
    }

    #[test]
    fn my_ex2_rotated() {
        assert_eq!(Solution::find_paths(3, 1, 3, 1, 0), 12);
    }

    #[test]
    fn my_ex2_alt() {
        // One step moves -> 3 ways out
        // Two step moves -> 2 ways out
        //   (There's only one move that stays in the grid, and it moves
        //    to a position where there are only two ways out.)
        // Three step moves:
        //   1. Cross the grid then leave -> 3 ways out
        //   2. Step into the grid then back -> 3 ways out (back at the start)
        // Total: 3 + 2 + 3 + 3 = 11
        assert_eq!(Solution::find_paths(1, 3, 3, 0, 0), 11);
    }
}
