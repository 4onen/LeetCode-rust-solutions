// https://leetcode.com/problems/count-square-submatrices-with-all-ones/

pub struct Solution;

impl Solution {
    pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..matrix.len() {
            ans += matrix[i][0];
        }
        for j in 1..matrix[0].len() {
            ans += matrix[0][j];
        }
        for i in 1..matrix.len() {
            for j in 1..matrix[i].len() {
                if matrix[i][j] > 0 {
                    if matrix[i - 1][j] > 0 && matrix[i - 1][j - 1] > 0 && matrix[i][j - 1] > 0 {
                        let res = std::cmp::min(
                            matrix[i - 1][j],
                            std::cmp::min(matrix[i][j - 1], matrix[i - 1][j - 1]),
                        ) + 1;
                        ans += res;
                        matrix[i][j] = res;
                    } else {
                        ans += matrix[i][j];
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &[&[i32]], expected: i32) {
        assert!(input.len() >= 1);
        assert!(input.len() <= 300);
        for &row in input {
            assert!(row.len() >= 1);
            assert!(row.len() <= 300);
            for &el in row {
                assert!(el == 0 || el == 1);
            }
        }
        let matrix = input.iter().map(|&x| x.to_vec()).collect();
        assert_eq!(Solution::count_squares(matrix), expected);
    }

    #[test]
    fn ex1() {
        test(&[&[0, 1, 1, 1], &[1, 1, 1, 1], &[0, 1, 1, 1]], 15)
    }

    #[test]
    fn ex2() {
        test(&[&[1, 0, 1], &[1, 1, 0], &[1, 1, 0]], 7)
    }

    #[test]
    fn discussion_case1() {
        test(&[&[1, 0, 0], &[1, 1, 0]], 3)
    }

    #[test]
    fn discussion_case2() {
        test(
            &[&[1, 1, 1, 1], &[1, 0, 0, 1], &[1, 1, 0, 0], &[1, 0, 0, 0]],
            4 + 2 + 2 + 1,
        )
    }

    #[test]
    fn discussion_case3() {
        test(
            &[&[0, 1, 1, 1], &[1, 1, 0, 1], &[1, 1, 1, 1], &[1, 0, 1, 0]],
            3 + 3 + 4 + 2 + 1,
        )
    }
}
