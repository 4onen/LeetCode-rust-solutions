// https://leetcode.com/problems/matrix-diagonal-sum/

pub struct Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        (0..mat.len())
            .map(|i| {
                let j = mat.len() - i - 1;
                mat[i][i] + if i != j { mat[i][j] } else { 0 }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::diagonal_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            25
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::diagonal_sum(vec![vec![1, 1, 1, 1]; 4]), 8);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::diagonal_sum(vec![vec![5]]), 5);
    }
}
