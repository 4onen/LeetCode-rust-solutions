// https://leetcode.com/problems/check-if-every-row-and-column-contains-all-numbers/

pub struct Solution;

impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        for row in &matrix {
            let mut row_set = 0u128;
            for &x in row {
                row_set |= 1 << x;
            }
            if row_set != (1 << (matrix.len() + 1)) - 2 {
                dbg!(row_set);
                return false;
            }
        }
        for j in 0..matrix.len() {
            let mut col_set = 0u128;
            for i in 0..matrix.len() {
                col_set |= 1 << matrix[i][j];
            }
            if col_set != (1 << (matrix.len() + 1)) - 2 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(matrix: &[&[i32]], expected: bool) {
        assert!(matrix.len() >= 1);
        assert!(matrix.len() <= 100);
        for &row in matrix {
            assert_eq!(row.len(), matrix.len());
            for &x in row {
                assert!(x >= 1);
                assert!(x <= matrix.len() as i32);
            }
        }
        assert_eq!(
            Solution::check_valid(matrix.iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[&[1, 2, 3], &[3, 1, 2], &[2, 3, 1]], true)
    }

    #[test]
    fn ex2() {
        test(&[&[1, 1, 1], &[1, 2, 3], &[1, 2, 3]], false)
    }
}
