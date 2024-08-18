// https://leetcode.com/problems/ugly-number-ii/

pub struct Solution;

impl Solution {
    pub const fn nth_ugly_number(n: i32) -> i32 {
        assert!(n >= 1);
        assert!(n <= 1690);
        let n = n as u16;
        let mut dp = [0; 1690];
        dp[0] = 1;
        let mut i = 1;
        let mut last_two_idx = 0;
        let mut last_three_idx = 0;
        let mut last_five_idx = 0;
        while i < n {
            let new_two = dp[last_two_idx as usize] * 2;
            let new_three = dp[last_three_idx as usize] * 3;
            let new_five = dp[last_five_idx as usize] * 5;
            let new = if new_two < new_three {
                new_two
            } else {
                new_three
            };
            let new = if new < new_five {
                new
            } else {
                last_five_idx += 1;
                new_five
            };
            if new == dp[last_two_idx as usize] * 2 {
                last_two_idx += 1;
            }
            if new == dp[last_three_idx as usize] * 3 {
                last_three_idx += 1;
            }
            dp[i as usize] = new;
            i += 1;
        }
        dp[n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u16, expected: u16) {
        assert!(n >= 1);
        assert!(n <= 1690);
        assert_eq!(Solution::nth_ugly_number(n as i32), expected as i32);
    }

    #[test]
    fn ex1() {
        test(10, 12)
    }

    #[test]
    fn ex2() {
        test(1, 1)
    }
}
