// https://leetcode.com/problems/minimum-time-to-make-rope-colorful/

pub struct Solution;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let colors = colors.as_bytes();
        let mut prev = None;
        let mut prev_max_cost: u32 = 0;
        let mut total_cost: u32 = 0;

        for (color, cost) in std::iter::zip(colors.into_iter(), needed_time.into_iter()) {
            if Some(color) != prev {
                prev = Some(color);
                total_cost -= prev_max_cost;
                prev_max_cost = 0;
            }

            prev_max_cost = prev_max_cost.max(cost as u32);

            total_cost += cost as u32;
        }

        total_cost -= prev_max_cost;

        total_cost as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::min_cost("abaac".to_string(), vec![1, 2, 3, 4, 5]),
            3
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::min_cost("abc".to_string(), vec![1, 2, 3]), 0);
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::min_cost("aabaa".to_string(), vec![1, 2, 3, 4, 1]),
            2
        );
    }

    #[test]
    fn discussion_ex1() {
        assert_eq!(
            Solution::min_cost("bbbaaa".to_string(), vec![4, 9, 3, 8, 8, 9]),
            23
        );
    }

    #[test]
    fn discussion_ex2() {
        assert_eq!(
            Solution::min_cost(
                "aaabbbabbbb".to_string(),
                vec![3, 5, 10, 7, 5, 3, 5, 5, 4, 8, 1]
            ),
            26
        );
    }
}
