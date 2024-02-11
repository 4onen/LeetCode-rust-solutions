// https://leetcode.com/problems/power-of-four/

pub struct Solution;

impl Solution {
    pub const fn is_power_of_four(n: i32) -> bool {
        // Make sure the number is at least a power of two
        (n.count_ones() == 1) && 
        // Make sure the number of trailing zeros is even
        // to check only for powers of 4
        (n.trailing_zeros() & 1 == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::is_power_of_four(16), true);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::is_power_of_four(5), false);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::is_power_of_four(1), true);
    }

    #[test]
    fn ex4() {
        assert_eq!(Solution::is_power_of_four(0), false);
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::is_power_of_four(4), true);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::is_power_of_four(8), false);
    }

    #[test]
    fn myex3() {
        assert_eq!(Solution::is_power_of_four(32), false);
    }

    #[test]
    fn myex4() {
        assert_eq!(Solution::is_power_of_four(64), true);
    }

    #[test]
    fn myex6() {
        assert_eq!(Solution::is_power_of_four(256), true);
    }
}
