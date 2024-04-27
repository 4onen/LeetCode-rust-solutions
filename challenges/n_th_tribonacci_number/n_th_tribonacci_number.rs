// https://leetcode.com/problems/n-th-tribonacci-number/

pub struct Solution;

impl Solution {
    pub const fn tribonacci(mut n: i32) -> i32 {
        let mut arr = [0, 1, 1];
        while n > 0 {
            arr = [arr[1], arr[2], arr[0] + arr[1] + arr[2]];
            n -= 1;
        }
        arr[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::tribonacci(4), 4);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::tribonacci(25), 1389537);
    }

    #[test]
    fn myex0() {
        assert_eq!(Solution::tribonacci(0), 0);
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::tribonacci(1), 1);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::tribonacci(2), 1);
    }

    #[test]
    fn myex3() {
        assert_eq!(Solution::tribonacci(3), 2);
    }

    #[test]
    fn myex5() {
        assert_eq!(Solution::tribonacci(5), 7);
    }

    #[test]
    fn myex6() {
        assert_eq!(Solution::tribonacci(6), 13);
    }

    #[test]
    fn myex7() {
        assert_eq!(Solution::tribonacci(7), 24);
    }

    #[test]
    fn myex8() {
        assert_eq!(Solution::tribonacci(8), 44);
    }
}
