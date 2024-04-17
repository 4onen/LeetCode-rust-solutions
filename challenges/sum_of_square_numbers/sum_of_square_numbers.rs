// https://leetcode.com/problems/sum-of-square-numbers/

pub struct Solution;

pub const fn isqrt_upper_bound(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    1 << (n.ilog2() / 2 + 1)
}
impl Solution {
    pub const fn judge_square_sum(c: i32) -> bool {
        if c < 0 {
            return false;
        }
        if c == 0 {
            return true;
        }
        let c = c as u32;
        let mut left = 0u32;
        let mut right = isqrt_upper_bound(c);
        while left <= right {
            let Some(right_square) = right.checked_mul(right) else {
                right -= 1;
                continue;
            };
            let sum = left * left + right_square;
            if sum == c {
                return true;
            } else if sum < c {
                left += 1;
            } else {
                right -= 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(c: i32, expected: bool) {
        assert_eq!(Solution::judge_square_sum(c), expected);
    }

    #[test]
    fn ex1() {
        test(5, true)
    }

    #[test]
    fn ex2() {
        test(3, false)
    }

    #[test]
    fn my_extreme_ex1() {
        test(i32::MAX, false)
    }
}
