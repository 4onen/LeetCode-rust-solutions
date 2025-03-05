// https://leetcode.com/problems/count-total-number-of-colored-cells/

pub struct Solution;

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        n as i64 * n as i64 + (n - 1) as i64 * (n - 1) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: i32, expected: i64) {
        assert_eq!(Solution::colored_cells(n as i32), expected);
    }

    #[test]
    fn ex1() {
        test(1, 1)
    }

    #[test]
    fn ex2() {
        test(2, 5)
    }

    #[test]
    fn ex3() {
        test(3, 4 + 5 + 4)
    }

    #[test]
    fn discussion_case4() {
        test(4, 25)
    }

    #[test]
    fn discussion_case5() {
        test(5, 41)
    }

    #[test]
    fn discussion_case6() {
        test(6, 61)
    }

    #[test]
    fn my_ex100000() {
        test(100000, 19999800001)
    }
}
