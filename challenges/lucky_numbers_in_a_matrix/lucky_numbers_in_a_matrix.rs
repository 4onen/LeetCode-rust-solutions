// https://leetcode.com/problems/lucky-numbers-in-a-matrix/

pub struct Solution;

impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        'outer: for row in matrix.iter() {
            let mut min_idx: u8 = 0;
            let mut i = 1;
            while i < row.len() as u8 {
                if row[i as usize] < row[min_idx as usize] {
                    min_idx = i;
                }
                i += 1;
            }
            let min = row[min_idx as usize];
            for i in 0..matrix.len() as u8 {
                if matrix[i as usize][min_idx as usize] > min {
                    continue 'outer;
                }
            }
            result.push(min);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(matrix: &[&[i32]], expected: &[i32]) {
        assert!(matrix.len() >= 1);
        assert!(matrix.len() <= 50);
        for &row in matrix {
            assert!(row.len() >= 1);
            assert!(row.len() <= 50);
            for &v in row {
                assert!(v >= 1);
                assert!(v <= 100_000);
            }
        }

        let mut result = Solution::lucky_numbers(matrix.iter().map(|v| v.to_vec()).collect());
        let mut expected = expected.to_vec();
        result.sort_unstable();
        expected.sort_unstable();
        assert_eq!(result, expected);
    }

    #[test]
    fn ex1() {
        test(&[
            &[3, 7, 8],
            &[9, 11, 13],
            &[15, 16, 17],
        ], &[15]);
    }

    #[test]
    fn ex2() {
        test(&[
            &[1, 10, 4, 2],
            &[9, 3, 8, 7],
            &[15, 16, 17, 12],
        ], &[12]);
    }

    #[test]
    fn ex3() {
        test(&[
            &[7, 8],
            &[1, 2],
        ], &[7]);
    }
}
