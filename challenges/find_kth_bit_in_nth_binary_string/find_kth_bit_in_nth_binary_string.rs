// https://leetcode.com/problems/find-kth-bit-in-nth-binary-string/

// 0
// 0 1 1
// 011 1 001
// 0111001 1 0110001
// 011100110110001 1 011100100110001
// 0111001101100011011100100110001 1 0111001101100010011100100110001

pub struct Solution;

impl Solution {
    pub const fn find_kth_bit(n: i32, k: i32) -> char {
        if n == 1 {
            return '0';
        }
        let len = 2_i32.pow(n as u32) - 1;
        if k == len / 2 + 1 {
            return '1';
        }
        if k < len / 2 + 1 {
            return Self::find_kth_bit(n - 1, k);
        }
        let k = len - k + 1;
        let bit = Self::find_kth_bit(n - 1, k);
        if bit == '0' {
            return '1';
        } else {
            return '0';
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: i32, k: i32, expected: char) {
        assert!(1 <= n);
        assert!(n <= 20);
        assert!(1 <= k);
        assert!(k <= 2_i32.pow(n as u32) - 1);
        assert_eq!(Solution::find_kth_bit(n, k), expected);
    }

    #[test]
    fn ex1() {
        test(3, 1, '0');
    }

    #[test]
    fn ex2() {
        test(4, 11, '1');
    }
}
