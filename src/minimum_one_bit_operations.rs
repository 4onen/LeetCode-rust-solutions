// https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero/

pub struct Solution;

impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let mut n = n;
        let mut result = 0;
        while n > 0 {
            result ^= n;
            n >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ex0() {
        assert_eq!(Solution::minimum_one_bit_operations(0), 0);
    }

    #[test]
    fn ex1() {
        assert_eq!(Solution::minimum_one_bit_operations(1), 1);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::minimum_one_bit_operations(2), 3);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::minimum_one_bit_operations(3), 2);
    }

    #[test]
    fn ex4() {
        assert_eq!(Solution::minimum_one_bit_operations(4), 7);
    }

    #[test]
    fn ex5() {
        assert_eq!(Solution::minimum_one_bit_operations(5), 6);
    }

    #[test]
    fn ex6() {
        assert_eq!(Solution::minimum_one_bit_operations(6), 4);
    }

    #[test]
    fn ex7() {
        assert_eq!(Solution::minimum_one_bit_operations(7), 5);
    }

    #[test]
    fn ex8() {
        // 1000
        // 1001
        // 1011
        // 1010
        // 1110
        // 1111
        // 1101
        // 1100
        // 0100
        // 0101
        // 0111
        // 0110
        // 0010
        // 0011
        // 0001
        // 0000
        assert_eq!(Solution::minimum_one_bit_operations(8), 15);
    }
}
