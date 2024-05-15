// https://leetcode.com/problems/find-the-safest-path-in-a-grid/

pub struct Solution;

impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        // Early exit: If the start point or end point is a 1, we can't
        // get a safeness factor above 0.
        if grid[0][0] == 1 || grid[grid.len() - 1][grid.len() - 1] == 1 {
            return 0;
        }
        // First, we need to find the safeness factor of each cell.
        // The safeness factor of a cell is the minimum distance to
        // any cell with a value of 1. We can use a simple two-pass
        // scan to fill this in.
        assert!(grid.len() >= 1, "grid must have at least one row");
        assert!(grid.len() <= 400, "grid must have at most 400 rows");
        let n = grid.len() as u16;
        let mut safeness = vec![vec![n << 1; n as usize]; n as usize];
        for i in 0..n {
            for j in 0..n {
                if grid[i as usize][j as usize] == 1 {
                    safeness[i as usize][j as usize] = 0;
                } else {
                    if i > 0 {
                        safeness[i as usize][j as usize] = std::cmp::min(
                            safeness[i as usize][j as usize],
                            safeness[i as usize - 1][j as usize] + 1,
                        );
                    }
                    if j > 0 {
                        safeness[i as usize][j as usize] = std::cmp::min(
                            safeness[i as usize][j as usize],
                            safeness[i as usize][j as usize - 1] + 1,
                        );
                    }
                }
            }
        }
        // Reverse pass
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if i + 1 < n {
                    safeness[i as usize][j as usize] = std::cmp::min(
                        safeness[i as usize][j as usize],
                        safeness[i as usize + 1][j as usize] + 1,
                    );
                }
                if j + 1 < n {
                    safeness[i as usize][j as usize] = std::cmp::min(
                        safeness[i as usize][j as usize],
                        safeness[i as usize][j as usize + 1] + 1,
                    );
                }
            }
        }
        // Now we try paths,
        let mut max_safeness = vec![vec![n << 1; n as usize]; n as usize];
        let mut queue = std::collections::BinaryHeap::new();
        queue.push((safeness[0][0], 0, 0));
        let result = loop {
            let Some((old_path_safeness, i, j)) = queue.pop() else {
                break 0;
            };
            let this_safeness = safeness[i as usize][j as usize];
            let path_safeness = std::cmp::min(this_safeness, old_path_safeness);
            // dbg!(path_safeness, i, j);
            if i == n - 1 && j == n - 1 {
                break path_safeness as i32; // We did it!
            }
            if path_safeness >= max_safeness[i as usize][j as usize] {
                continue; // Already been here with this safeness
            }
            max_safeness[i as usize][j as usize] = path_safeness;
            if i > 0 {
                queue.push((path_safeness, i - 1, j));
            }
            if j > 0 {
                queue.push((path_safeness, i, j - 1));
            }
            if i < n - 1 {
                queue.push((path_safeness, i + 1, j));
            }
            if j < n - 1 {
                queue.push((path_safeness, i, j + 1));
            }
        };
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid: &[&[u8]], expected: u16) {
        assert!(grid.len() >= 1, "grid must have at least one row");
        assert!(grid.len() <= 400, "grid must have at most 400 rows");
        for &row in grid {
            assert!(row.len() == grid.len(), "grid must be square");
            for &cell in row {
                assert!(cell == 0 || cell == 1, "cell must be 0 or 1");
            }
        }
        assert!(
            grid.iter().any(|row| row.iter().any(|&cell| cell == 1)),
            "grid must contain at least one 1"
        );
        assert!(
            expected <= grid.len() as u16,
            "expected safeness factor must be at most the grid side length"
        );
        let input = grid
            .iter()
            .map(|row| row.iter().map(|&cell| cell as i32).collect())
            .collect();
        assert_eq!(Solution::maximum_safeness_factor(input), expected as i32);
    }

    #[test]
    fn ex1() {
        test(&[&[1, 0, 0], &[0, 0, 0], &[0, 0, 1]], 0);
    }

    #[test]
    fn ex2() {
        test(&[&[0, 0, 1], &[0, 0, 0], &[0, 0, 0]], 2);
    }

    #[test]
    fn ex3() {
        test(
            &[&[0, 0, 0, 1], &[0, 0, 0, 0], &[0, 0, 0, 0], &[1, 0, 0, 0]],
            2,
        );
    }

    #[test]
    fn discussion_case1() {
        test(&[&[0, 1, 1], &[0, 1, 1], &[0, 0, 0]], 1)
    }

    #[test]
    fn myex1() {
        // This example disproves that you only need to consider movements
        // to the right and down.
        test(
            &[
                &[0, 1, 0, 0, 0],
                &[0, 1, 0, 1, 0],
                &[0, 1, 0, 1, 0],
                &[0, 1, 0, 1, 0],
                &[0, 0, 0, 1, 0],
            ],
            1,
        )
    }

    #[test]
    fn myex2() {
        // This example duplicates the above, but is less obvious.
        test(
            &[
                &[0, 0, 1, 0, 0, 0, 0, 0, 0],
                &[0, 0, 1, 0, 0, 0, 0, 0, 0],
                &[0, 0, 1, 0, 0, 0, 1, 0, 0],
                &[0, 0, 1, 0, 0, 0, 1, 0, 0],
                &[0, 0, 1, 0, 0, 0, 1, 0, 0],
                &[0, 0, 1, 0, 0, 0, 1, 0, 0],
                &[0, 0, 1, 0, 0, 0, 1, 0, 0],
                &[0, 0, 0, 0, 0, 0, 1, 0, 0],
                &[0, 0, 0, 0, 0, 0, 1, 0, 0],
            ],
            2,
        )
    }

    #[test]
    fn myex3() {
        // Spiral maze just to frustrate scan algorithms.
        test(
            &[
                &[0, 1, 0, 0, 0, 0, 0, 0, 0],
                &[0, 1, 0, 1, 1, 1, 1, 1, 0],
                &[0, 1, 0, 1, 0, 0, 0, 1, 0],
                &[0, 1, 0, 1, 0, 1, 0, 1, 0],
                &[0, 1, 0, 1, 0, 1, 0, 1, 0],
                &[0, 1, 0, 1, 0, 1, 0, 1, 0],
                &[0, 1, 0, 0, 0, 1, 0, 1, 0],
                &[0, 1, 1, 1, 1, 1, 0, 1, 0],
                &[0, 0, 0, 0, 0, 0, 0, 1, 0],
            ],
            1,
        )
    }

    #[test]
    fn my_extreme_ex1() {
        // This test is the largest possible grid, full of 1s except for the
        // start and end points. This is the worst case scenario for the
        // pathfinding algorithm.
        const N: usize = 400;
        let mut grid = vec![vec![1; N]; N];
        grid[0][0] = 0;
        grid[N - 1][N - 1] = 0;
        test(
            &grid.iter().map(|row| row.as_slice()).collect::<Vec<_>>(),
            0,
        );
    }
}
