// https://leetcode.com/problems/island-perimeter/

pub struct Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut perimeter = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 1 {
                    perimeter += 4;
                    if i > 0 && grid[i - 1][j] == 1 {
                        perimeter -= 2;
                    }
                    if j > 0 && grid[i][j - 1] == 1 {
                        perimeter -= 2;
                    }
                }
            }
        }
        perimeter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid: &[&[i32]], expected: i32) {
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(Solution::island_perimeter(grid), expected);
    }

    #[test]
    fn ex1() {
        test(
            &[&[0, 1, 0, 0], &[1, 1, 1, 0], &[0, 1, 0, 0], &[1, 1, 0, 0]],
            16,
        );
    }

    #[test]
    fn ex2() {
        test(&[&[1]], 4)
    }

    #[test]
    fn ex3() {
        test(&[&[1, 0]], 4)
    }

    #[test]
    fn myex1() {
        test(&[&[0, 1]], 4)
    }

    #[test]
    fn myex2() {
        test(&[&[1, 1]], 6)
    }

    #[test]
    fn discussion_case1() {
        test(&[&[1, 1, 1], &[1, 0, 1]], 12)
    }

    #[test]
    fn discussion_case2() {
        test(&[&[1], &[1], &[1], &[0], &[1], &[1]], 14)
    }
}
