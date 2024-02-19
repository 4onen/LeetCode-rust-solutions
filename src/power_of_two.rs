// https://leetcode.com/problems/power-of-two/

pub struct Solution;

impl Solution {
    pub const fn is_power_of_two(n: i32) -> bool {
        (n.count_ones() == 1) && (n > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::is_power_of_two(1), true);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::is_power_of_two(16), true);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::is_power_of_two(3), false);
    }

    #[test]
    fn myex0() {
        assert_eq!(Solution::is_power_of_two(0), false);
    }

    #[test]
    fn myexneg1() {
        assert_eq!(Solution::is_power_of_two(-1), false);
    }

    #[test]
    fn myexneg2() {
        assert_eq!(Solution::is_power_of_two(-2), false);
    }

    #[test]
    fn myexintmax() {
        assert_eq!(Solution::is_power_of_two(i32::MAX), false);
    }

    #[test]
    fn myexintmin() {
        assert_eq!(Solution::is_power_of_two(i32::MIN), false);
    }
}
