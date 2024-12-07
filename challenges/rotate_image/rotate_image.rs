// https://leetcode.com/problems/rotate-image/

pub struct Solution;

// Initial sol'n
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        assert!(n >= 1);
        assert!(n <= 20);
        let n = n as u8;
        for i in 0..n / 2 {
            for j in i..n - 1 - i {
                let temp = matrix[i as usize][j as usize];
                matrix[i as usize][j as usize] = matrix[(n - j - 1) as usize][i as usize];
                matrix[(n - 1 - j) as usize][i as usize] =
                    matrix[(n - 1 - i) as usize][(n - 1 - j) as usize];
                matrix[(n - 1 - i) as usize][(n - 1 - j) as usize] =
                    matrix[j as usize][(n - 1 - i) as usize];
                matrix[j as usize][(n - 1 - i) as usize] = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(matrix: &[&[i32]], expected: &[&[i32]]) {
        assert!(matrix.len() == expected.len());
        assert!(matrix.len() >= 1);
        assert!(matrix.len() <= 20);
        for &row in matrix {
            assert!(row.len() == matrix.len());
            for &val in row {
                assert!(val >= -1000);
                assert!(val <= 1000);
            }
        }
        for &row in expected {
            assert!(row.len() == matrix.len());
            for &val in row {
                assert!(val >= -1000);
                assert!(val <= 1000);
            }
        }
        let mut matrix_to_change = matrix.iter().map(|row| row.to_vec()).collect();
        Solution::rotate(&mut matrix_to_change);
        assert_eq!(matrix_to_change, expected);
    }

    #[test]
    fn ex1() {
        test(
            &[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]],
            &[&[7, 4, 1], &[8, 5, 2], &[9, 6, 3]],
        )
    }

    #[test]
    fn ex1_1() {
        test(
            &[
                &[1, 2, 3, 33, 333],
                &[4, 5, 6, 66, 666],
                &[7, 8, 9, 99, 999],
                &[10, 11, 12, 22, 222],
                &[13, 14, 15, 55, 555],
            ],
            &[
                &[13, 10, 7, 4, 1],
                &[14, 11, 8, 5, 2],
                &[15, 12, 9, 6, 3],
                &[55, 22, 99, 66, 33],
                &[555, 222, 999, 666, 333],
            ],
        )
    }

    #[test]
    fn ex2() {
        test(
            &[
                &[5, 1, 9, 11],
                &[2, 4, 8, 10],
                &[13, 3, 6, 7],
                &[15, 14, 12, 16],
            ],
            &[
                &[15, 13, 2, 5],
                &[14, 3, 4, 1],
                &[12, 6, 8, 9],
                &[16, 7, 10, 11],
            ],
        )
    }
}
