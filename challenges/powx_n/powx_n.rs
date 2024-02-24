// https://leetcode.com/problems/powx-n/

pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        x.powi(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
    }

    #[test]
    fn ex2() {
        let result = Solution::my_pow(2.1, 3);
        let expected = 9.261;
        assert!((result - expected).abs() < 0.00001);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::my_pow(2.0, -2), 0.25);
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::my_pow(2.0, 0), 1.0);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::my_pow(2.0, 1), 2.0);
    }

    #[test]
    fn myex3() {
        assert_eq!(Solution::my_pow(2.0, -1), 0.5);
    }

    #[test]
    fn myex4() {
        assert_eq!(Solution::my_pow(0.0, 1), 0.0);
    }

    #[test]
    fn myex5() {
        assert_eq!(Solution::my_pow(0.0, 27), 0.0);
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(Solution::my_pow(-1.0, 2147483647), -1.0);
    }
}
