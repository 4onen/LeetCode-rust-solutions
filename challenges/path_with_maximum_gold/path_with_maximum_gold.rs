// https://leetcode.com/problems/path-with-maximum-gold/

pub struct Solution;

// Initial sol'n + early stopping (Didn't expect it to do 7ms where everyone else is doing 42ms.)
impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut [Vec<i32>], i: i8, j: i8) -> u16 {
            let mut max_gold = 0;
            let gold = grid[i as usize][j as usize] as u16;
            grid[i as usize][j as usize] = 0;
            for (di, dj) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let ni = i + di;
                let nj = j + dj;
                if ni >= 0 && ni < grid.len() as i8 && nj >= 0 && nj < grid[0].len() as i8 {
                    if grid[ni as usize][nj as usize] > 0 {
                        let path_gold = dfs(grid, ni, nj);
                        max_gold = max_gold.max(path_gold);
                    }
                }
            }
            grid[i as usize][j as usize] = gold as i32;
            max_gold + gold
        }
        let mut grid = grid;
        let total_gold_present = grid.iter().map(|row| row.iter().sum::<i32>()).sum::<i32>() as u16;
        let mut max_gold = 0u16;
        for i in 0..grid.len() as i8 {
            for j in 0..grid[0].len() as i8 {
                if grid[i as usize][j as usize] > 0 {
                    let result = dfs(&mut grid, i, j);
                    if result > max_gold {
                        max_gold = result;
                        // Stop early if we have found all the gold
                        if result == total_gold_present {
                            return result as i32;
                        }
                    }
                }
            }
        }
        max_gold as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid: &[&[u8]], expected: u16) {
        let grid: Vec<Vec<i32>> = grid
            .into_iter()
            .map(|&row| row.iter().copied().map(|x| x as i32).collect())
            .collect();
        assert!(grid.len() > 0);
        assert!(grid[0].len() > 0);
        assert!(grid.len() <= 15);
        assert!(grid[0].len() <= 15);
        for row in &grid {
            assert!(row.len() == grid[0].len());
            for &cell in row {
                assert!(cell >= 0);
                assert!(cell <= 100);
            }
        }
        assert_eq!(Solution::get_maximum_gold(grid), expected as i32);
    }

    #[test]
    fn ex1() {
        test(&[&[0, 6, 0], &[5, 8, 7], &[0, 9, 0]], 24)
    }

    #[test]
    fn ex2() {
        test(
            &[&[1, 0, 7], &[2, 0, 6], &[3, 4, 5], &[0, 3, 0], &[9, 0, 20]],
            28,
        )
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                &[1, 0, 7, 0, 0, 0],
                &[2, 0, 6, 0, 1, 0],
                &[3, 5, 6, 7, 4, 2],
                &[4, 3, 1, 0, 2, 0],
                &[3, 0, 5, 0, 20, 0],
            ],
            60,
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                &[1, 1, 1, 1, 1],
                &[1, 1, 1, 1, 1],
                &[1, 1, 1, 1, 1],
                &[1, 1, 1, 1, 1],
                &[1, 1, 1, 1, 1],
            ],
            25,
        )
    }
}
