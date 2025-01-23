// https://leetcode.com/problems/count-servers-that-communicate/

pub struct Solution;

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        type Idx = u8;
        let m = grid.len() as Idx;
        let n = grid[0].len() as Idx;
        let mut row_counts = vec![0; m as usize];
        let mut col_counts = vec![0; n as usize];
        for i in 0..m {
            for j in 0..n {
                if grid[i as usize][j as usize] == 1 {
                    row_counts[i as usize] += 1;
                    col_counts[j as usize] += 1;
                }
            }
        }
        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i as usize][j as usize] == 1
                    && (row_counts[i as usize] > 1 || col_counts[j as usize] > 1)
                {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid: &[&[i32]], expected: i32) {
        assert!(grid.len() >= 1);
        assert!(grid.len() <= 250);
        assert!(grid[0].len() >= 1);
        assert!(grid[0].len() <= 250);
        for &row in grid {
            assert_eq!(row.len(), grid[0].len());
            for &el in row {
                assert!(el >= 0);
                assert!(el <= 1);
            }
        }
        assert_eq!(
            Solution::count_servers(grid.iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[&[1, 0], &[0, 1]], 0)
    }

    #[test]
    fn ex2() {
        test(&[&[1, 0], &[1, 1]], 3)
    }

    #[test]
    fn ex3() {
        test(
            &[&[1, 1, 0, 0], &[0, 0, 1, 0], &[0, 0, 1, 0], &[0, 0, 0, 1]],
            4,
        )
    }
}
