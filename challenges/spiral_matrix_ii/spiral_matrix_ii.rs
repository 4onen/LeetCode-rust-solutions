// https://leetcode.com/problems/spiral-matrix-ii/

pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        assert!(n >= 1);
        assert!(n <= 20);
        let n = n as u8;
        let mut matrix = vec![vec![0; n as usize]; n as usize];
        let mut num = 1;
        let mut top = 0;
        let mut bottom = n - 1;
        let mut left = 0;
        let mut right = n - 1;
        while top <= bottom && left <= right {
            for j in left..=right {
                matrix[top as usize][j as usize] = num;
                num += 1;
            }
            top += 1;
            for i in top..=bottom {
                matrix[i as usize][right as usize] = num;
                num += 1;
            }
            right = right.saturating_sub(1);
            if top <= bottom {
                for j in (left..=right).rev() {
                    matrix[bottom as usize][j as usize] = num;
                    num += 1;
                }
                bottom = bottom.saturating_sub(1);
            }
            if left <= right {
                for i in (top..=bottom).rev() {
                    matrix[i as usize][left as usize] = num;
                    num += 1;
                }
                left += 1;
            }
        }
        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u8, expected: &[&[i32]]) {
        assert!(n >= 1);
        assert!(n <= 20);
        assert_eq!(Solution::generate_matrix(n as i32), expected);
    }

    #[test]
    fn ex1() {
        test(3, &[&[1, 2, 3], &[8, 9, 4], &[7, 6, 5]]);
    }

    #[test]
    fn ex2() {
        test(1, &[&[1]]);
    }

    #[test]
    fn myex2() {
        test(2, &[&[1, 2], &[4, 3]]);
    }

    #[test]
    fn myex4() {
        test(
            4,
            &[
                &[1, 2, 3, 4],
                &[12, 13, 14, 5],
                &[11, 16, 15, 6],
                &[10, 9, 8, 7],
            ],
        );
    }

    #[test]
    fn myex5() {
        test(
            5,
            &[
                &[1, 2, 3, 4, 5],
                &[16, 17, 18, 19, 6],
                &[15, 24, 25, 20, 7],
                &[14, 23, 22, 21, 8],
                &[13, 12, 11, 10, 9],
            ],
        );
    }

    #[test]
    fn myex6() {
        test(
            6,
            &[
                &[1, 2, 3, 4, 5, 6],
                &[20, 21, 22, 23, 24, 7],
                &[19, 32, 33, 34, 25, 8],
                &[18, 31, 36, 35, 26, 9],
                &[17, 30, 29, 28, 27, 10],
                &[16, 15, 14, 13, 12, 11],
            ],
        );
    }
}
