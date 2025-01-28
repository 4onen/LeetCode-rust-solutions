// https://leetcode.com/problems/maximum-number-of-fish-in-a-grid/

pub struct Solution;

// Initial sol'n
impl Solution {
    pub fn find_max_fish(mut grid: Vec<Vec<i32>>) -> i32 {
        fn water_dfs(grid: &mut [Vec<i32>], i: i8, j: i8) -> i32 {
            if i < 0 || j < 0 || i >= grid.len() as i8 || j >= grid[i as usize].len() as i8 {
                return 0;
            }
            let fish = std::mem::replace(&mut grid[i as usize][j as usize], 0);
            if fish <= 0 {
                return 0;
            }
            return fish
                + water_dfs(grid, i + 1, j)
                + water_dfs(grid, i - 1, j)
                + water_dfs(grid, i, j + 1)
                + water_dfs(grid, i, j - 1);
        }
        let mut max = 0;
        for i in 0..grid.len() as i8 {
            for j in 0..grid[i as usize].len() as i8 {
                let pond = water_dfs(&mut grid, i, j);
                if pond > max {
                    max = pond;
                }
            }
        }
        return max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid: &[&[u8]], expected: i32) {
        assert!(grid.len() >= 1);
        assert!(grid.len() <= 10);
        for &row in grid {
            assert!(row.len() >= 1);
            assert!(row.len() <= 10);
            for &el in row {
                assert!(el <= 10);
            }
        }
        assert_eq!(
            Solution::find_max_fish(
                grid.iter()
                    .map(|&x| x.iter().map(|&x| x as i32).collect())
                    .collect()
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(
            &[&[0, 2, 1, 0], &[4, 0, 0, 3], &[1, 0, 0, 4], &[0, 3, 2, 0]],
            7,
        )
    }

    #[test]
    fn ex2() {
        test(
            &[&[1, 0, 0, 0], &[0, 0, 0, 0], &[0, 0, 0, 0], &[0, 0, 0, 1]],
            1,
        )
    }

    #[test]
    fn failing_case1() {
        test(&[&[0, 4]], 4)
    }

    #[test]
    fn failing_case1_1() {
        test(&[&[0], &[4]], 4)
    }
}
