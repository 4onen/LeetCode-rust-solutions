// https://leetcode.com/problems/delete-greatest-value-in-each-row/

pub struct Solution;

impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        loop {
            if grid[0].len() == 0 {
                break res;
            }
            let mut max = 0;
            for row in &mut grid {
                let mut row_max = 0;
                for i in 0..row.len() {
                    if row[i] > row[row_max] {
                        row_max = i;
                    }
                }
                if row[row_max] > max {
                    max = row[row_max];
                }
                row.swap_remove(row_max);
            }
            res += max;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid: &[&[i32]], expected: i32) {
        assert!(grid.len() >= 1);
        assert!(grid.len() <= 50);
        for &row in grid {
            assert!(row.len() >= 1);
            assert!(row.len() <= 50);
            assert_eq!(row.len(), grid[0].len());
            for &el in row {
                assert!(el >= 1);
                assert!(el <= 100);
            }
        }
        assert_eq!(
            Solution::delete_greatest_value(grid.iter().map(|x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[&[1, 2, 4], &[3, 3, 1]], 8)
    }

    #[test]
    fn ex2() {
        test(&[&[10]], 10)
    }
}
