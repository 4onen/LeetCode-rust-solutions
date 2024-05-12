// https://leetcode.com/problems/largest-local-values-in-a-matrix/

pub struct Solution;

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut filtered_grid: Vec<Vec<i32>> = vec![vec![0; grid.len() - 2]; grid[0].len() - 2];
        // Iterate through valid filter positions (excluding borders)
        for y in 0..filtered_grid.len() {
            for x in 0..filtered_grid[0].len() {
                // Create an empty vector to store window values
                let mut max: i32 = 0;
                // Iterate through the 3x3 window and add valid values
                for dy in 0..3 {
                    for dx in 0..3 {
                        // Assert pixel is within image bounds
                        assert!(
                            (y + dy < grid.len()) && (x + dx < grid[y + dy].len()),
                            "Pixel out of bounds"
                        );
                        // Update the maximum value
                        let val = grid[y + dy][x + dx];
                        assert!(val > 0 && val <= 100, "Invalid pixel value");
                        if val > max {
                            max = val;
                        }
                    }
                }
                // Set the filtered pixel to the maximum value
                filtered_grid[y][x] = max;
            }
        }
        // Return the filtered image
        filtered_grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid: &[&[i32]], expected: &[&[i32]]) {
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(Solution::largest_local(grid), expected);
    }

    #[test]
    fn ex1() {
        test(
            &[&[9, 9, 8, 1], &[5, 6, 2, 6], &[8, 2, 6, 4], &[6, 2, 2, 2]],
            &[&[9, 9], &[8, 6]],
        )
    }

    #[test]
    fn ex2() {
        test(
            &[
                &[1, 1, 1, 1, 1],
                &[1, 1, 1, 1, 1],
                &[1, 1, 2, 1, 1],
                &[1, 1, 1, 1, 1],
                &[1, 1, 1, 1, 1],
            ],
            &[&[2, 2, 2], &[2, 2, 2], &[2, 2, 2]],
        )
    }
}
