// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_seen = i32::MAX;
        let mut max_profit = 0;
        for &price in prices.iter() {
            if price < min_seen {
                min_seen = price;
            } else if price - min_seen > max_profit {
                max_profit = price - min_seen;
            }
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(prices: &[i32], expected: i32) {
        assert_eq!(Solution::max_profit(prices.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[7, 1, 5, 3, 6, 4], 5)
    }

    #[test]
    fn ex2() {
        test(&[7, 6, 4, 3, 1], 0)
    }
}
