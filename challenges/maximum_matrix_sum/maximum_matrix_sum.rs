// https://leetcode.com/problems/maximum-matrix-sum/

pub struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut min_abs_val = i32::MAX;
        let mut sum: i64 = 0;
        let mut neg_count = 0;
        for row in matrix {
            for el in row {
                let el_abs = el.abs();
                neg_count += (el < 0) as u16;
                if el_abs < min_abs_val {
                    min_abs_val = el_abs;
                }
                sum += el_abs as i64;
            }
        }
        if neg_count % 2 == 0 {
            return sum;
        } else {
            return sum - 2*min_abs_val as i64;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(matrix: &[&[i32]], expected: i64) {
        assert_eq!(
            Solution::max_matrix_sum(matrix.iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[&[1, -1], &[-1, 1]], 4)
    }

    #[test]
    fn ex2() {
        test(&[&[1, 2, 3], &[-1, -2, -3], &[1, 2, 3]], 16)
    }

    #[test]
    fn discussion_case0() {
        test(&[&[0,0,0], &[0,0,0], &[0,0,0]], 0)
    }

    #[test]
    fn discussion_case0_1() {
        test(&[&[0,0,0], &[0,1,0], &[0,0,0]], 1)
    }

    #[test]
    fn discussion_case0_2() {
        test(&[&[0,0,0], &[0,-1,0], &[0,0,0]], 1)
    }
}
