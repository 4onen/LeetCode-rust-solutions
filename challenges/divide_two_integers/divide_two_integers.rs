// https://leetcode.com/problems/divide-two-integers/

pub struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let result_negative = (dividend < 0) ^ (divisor < 0);

        let mut dividend = dividend.unsigned_abs();
        let divisor = divisor.unsigned_abs();

        let mut result: i64 = 0;
        while dividend >= divisor {
            let mut count: u8 = 0;
            while dividend as u64 >= ((divisor as u64) << 1 << count) {
                count += 1;
            }

            result += 1 << count;
            dividend -= divisor << count;
        }

        if result_negative {
            std::cmp::max(-result, i32::MIN as i64) as i32
        } else {
            std::cmp::min(result, i32::MAX as i64) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let dividend = 10;
        let divisor = 3;
        assert_eq!(Solution::divide(dividend, divisor), 3);
    }

    #[test]
    fn ex2() {
        let dividend = 7;
        let divisor = -3;
        assert_eq!(Solution::divide(dividend, divisor), -2);
    }

    #[test]
    fn discussion_case1() {
        let dividend = -2147483648;
        let divisor = -1;
        assert_eq!(Solution::divide(dividend, divisor), 2147483647);
    }

    #[test]
    fn discussion_case2() {
        let dividend = -2147483648;
        let divisor = 2147483647;
        assert_eq!(Solution::divide(dividend, divisor), -1);
    }

    #[test]
    fn discussion_case3() {
        let dividend = 2147483647;
        let divisor = 2147483647;
        assert_eq!(Solution::divide(dividend, divisor), 1);
    }
}
