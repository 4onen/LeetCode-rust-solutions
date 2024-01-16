// https://leetcode.com/problems/difference-between-ones-and-zeros-in-row-and-column/

pub struct Solution;

impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = grid.len();
        let cols = grid[0].len();

        fn fold_fn((ones, zeros): (i32, i32), cell: i32) -> (i32, i32) {
            match cell {
                0 => (ones, zeros + 1),
                _ => (ones + 1, zeros),
            }
        }

        let (ones_row, zeros_row) = grid
            .iter()
            .map(|row| row.iter().copied().fold((0, 0), fold_fn))
            .unzip::<i32, i32, Vec<i32>, Vec<i32>>();

        let (ones_col, zeros_col) = (0..cols)
            .into_iter()
            .map(|col| grid.iter().map(|row| row[col]).fold((0, 0), fold_fn))
            .unzip::<i32, i32, Vec<i32>, Vec<i32>>();

        let mut result = grid;

        for row in 0..rows {
            for col in 0..cols {
                result[row][col] = ones_row[row] + ones_col[col] - zeros_row[row] - zeros_col[col];
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let grid: Vec<Vec<i32>> = vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]];
        let expected: Vec<Vec<i32>> = vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]];
        assert_eq!(Solution::ones_minus_zeros(grid), expected);
    }

    #[test]
    fn ex2() {
        let grid: Vec<Vec<i32>> = vec![vec![1, 1, 1], vec![1, 1, 1]];
        let expected: Vec<Vec<i32>> = vec![vec![5, 5, 5], vec![5, 5, 5]];
        assert_eq!(Solution::ones_minus_zeros(grid), expected);
    }

    #[test]
    fn one() {
        let grid: Vec<Vec<i32>> = vec![vec![1]];
        let expected: Vec<Vec<i32>> = vec![vec![2]];
        assert_eq!(Solution::ones_minus_zeros(grid), expected);
    }

    #[test]
    fn zero() {
        let grid: Vec<Vec<i32>> = vec![vec![0]];
        let expected: Vec<Vec<i32>> = vec![vec![-2]];
        assert_eq!(Solution::ones_minus_zeros(grid), expected);
    }

    #[test]
    fn two_by_two() {
        let grid: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0]];
        let expected: Vec<Vec<i32>> = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(Solution::ones_minus_zeros(grid), expected);
    }
}
