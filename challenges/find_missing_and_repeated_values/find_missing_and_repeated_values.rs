// https://leetcode.com/problems/find-missing-and-repeated-values/

pub struct Solution;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        assert!(n > 1);
        assert!(n <= 50);
        let mut counts = vec![0u8; n * n + 1];
        counts[0] = 1; // 0 is not in the grid
        for row in grid {
            for val in row {
                counts[val as usize] += 1;
            }
        }
        let a = counts.iter().position(|&x| x == 2).unwrap() as i32;
        let b = counts.iter().position(|&x| x == 0).unwrap() as i32;
        vec![a, b]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let grid = vec![vec![1, 3], vec![2, 2]];
        let result = Solution::find_missing_and_repeated_values(grid);
        assert_eq!(result, vec![2, 4]);
    }

    #[test]
    fn ex2() {
        let grid = vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]];
        let result = Solution::find_missing_and_repeated_values(grid);
        assert_eq!(result, vec![9, 5]);
    }
}
