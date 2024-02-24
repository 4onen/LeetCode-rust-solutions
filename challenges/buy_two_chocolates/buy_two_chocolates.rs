// https://leetcode.com/problems/buy-two-chocolates
pub struct Solution;

impl Solution {
    pub fn buy_choco(mut prices: Vec<i32>, money: i32) -> i32 {
        let (minslice, min2, _) = prices.select_nth_unstable(1);
        let lowest = minslice[0];
        let second_lowest = *min2;

        let result = money - (lowest + second_lowest);

        if result < 0 {
            money
        } else {
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::buy_choco(vec![1, 2, 2], 3), 0);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::buy_choco(vec![3, 2, 3], 3), 3);
    }
}
