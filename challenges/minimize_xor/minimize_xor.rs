// https://leetcode.com/problems/minimize-xor/

pub struct Solution;

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut bitcount = num2.count_ones();
        let mut res = 0;
        let mut bit = 32 - num1.leading_zeros();
        if bit == 0 {
            return res;
        }
        loop {
            if bitcount == 0 {
                break;
            }
            let mask = 1 << bit;
            if (num1 & mask) > 0 {
                res |= mask;
                bitcount -= 1;
            }
            if bit == 0 {
                break;
            }
            bit -= 1;
        }
        bit = 0;
        while bitcount > 0 {
            let mask = 1 << bit;
            if (res & mask) == 0 {
                res |= mask;
                bitcount -= 1;
            }
            bit += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(num1: i32, num2: i32, expected: i32) {
        assert!(num1 >= 1);
        assert!(num1 <= 1_000_000_000);
        assert!(num2 >= 1);
        assert!(num2 <= 1_000_000_000);
        assert_eq!(Solution::minimize_xor(num1, num2), expected);
    }

    #[test]
    fn ex1() {
        test(3, 5, 3)
    }

    #[test]
    fn ex1_1() {
        test(2, 5, 3)
    }

    #[test]
    fn ex2() {
        test(1, 12, 3)
    }
}
