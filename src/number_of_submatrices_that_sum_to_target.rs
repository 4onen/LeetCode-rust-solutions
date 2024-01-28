// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/

pub struct Solution;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let inner_len = matrix[0].len();
        let mut count = 0;
        let mut sum = vec![0; (matrix.len() + 1) * (inner_len + 1)];
        for i in 1..=matrix.len() {
            let matrix_row = &matrix[i - 1];
            for j in 1..=matrix_row.len() {
                let new_sum =
                    matrix_row[j - 1] + sum[(i - 1) * inner_len + j] + sum[i * inner_len + j - 1]
                        - sum[(i - 1) * inner_len + j - 1];
                sum[i * inner_len + j] = new_sum;
                let partial = new_sum - target;
                for k in 0..i {
                    let partial2 = partial - sum[k * inner_len + j];
                    for l in 0..j {
                        if partial2 - sum[i * inner_len + l] + sum[k * inner_len + l] == 0 {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::num_submatrix_sum_target(
                vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]],
                0
            ),
            4
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::num_submatrix_sum_target(vec![vec![1, -1], vec![-1, 1]], 0),
            5
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::num_submatrix_sum_target(vec![vec![904]], 0), 0);
    }

    #[test]
    fn discussion_case_extrema() {
        const INPUT: &str =
            include_str!("number_of_submatrices_that_sum_to_target_discussion_case.txt");
        let mut lines = INPUT.lines();
        let target = lines.next().unwrap().parse::<i32>().unwrap();
        let expected = lines.next().unwrap().parse::<i32>().unwrap();
        let matrix = lines
            .map(|line| {
                line.split(',')
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert_eq!(Solution::num_submatrix_sum_target(matrix, target), expected)
    }
}
