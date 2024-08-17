// https://leetcode.com/problems/best-sightseeing-pair/

pub struct Solution;

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        // Okay, so the score of a pair is
        // values[i] + values[j] + i - j
        // We can rewrite this as
        // values[i] + i + values[j] - j
        // We can scan the array from left to right, keeping track of
        // max_i {values[i] + i}
        // Then, at each position, we can calculate the score of the pair
        // as max_i + values[j] - j
        let mut max = 0;
        let mut best_i = values[0];
        for i in 1..values.len() as i32 {
            max = std::cmp::max(max, best_i + values[i as usize] - i);
            best_i = std::cmp::max(best_i, values[i as usize] + i);
            // We're `best_i`es! :D
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(values: &[i32], expected: i32) {
        assert_eq!(
            Solution::max_score_sightseeing_pair(values.to_vec()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[8, 1, 5, 2, 6], 11);
    }

    #[test]
    fn ex2() {
        test(&[1, 2], 2);
    }
}
