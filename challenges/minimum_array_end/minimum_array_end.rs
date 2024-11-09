// https://leetcode.com/problems/minimum-array-end/

pub struct Solution;

impl Solution {
    pub const fn min_end(n: i32, x: i32) -> i64 {
        let n = n - 1;
        let last_n_bit = 32 - (n.leading_zeros() as u8);
        let mut res = x as i64;
        let mut n_bit = 0u8;
        let mut res_bit = 0u8;
        while res_bit < i64::BITS as u8 {
            if res & (1i64<<res_bit) == 0 {
                if n & (1<<n_bit as i32) > 0i32 {
                    res |= 1<<res_bit;
                }
                n_bit += 1;
                if n_bit >= last_n_bit {
                    break;
                }
            }
            res_bit += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: i32, x: i32, expected: i64) {
        assert!(n >= 1);
        assert!(n <= 100_000_000);
        assert!(x >= 1);
        assert!(n <= 100_000_000);
        assert_eq!(Solution::min_end(n, x), expected);
    }

    #[test]
    fn ex1() {
        test(3, 4, 6)
    }

    #[test]
    fn ex2() {
        test(2, 7, 15)
    }

    #[test]
    fn failing_case1() {
        test(6715154, 7193485, 55012476815)
    }
}
