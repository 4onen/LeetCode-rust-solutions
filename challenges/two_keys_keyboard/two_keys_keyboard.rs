// https://leetcode.com/problems/2-keys-keyboard/

pub struct Solution;

impl Solution {
    pub const fn min_steps(n: i32) -> i32 {
        assert!(n >= 1);
        assert!(n <= 1000);
        let mut n = n as u16;
        let mut d = 2 as u16;
        let mut steps = 0 as u16;
        while n > 1 {
            while n % d == 0 {
                steps += d;
                n /= d;
            }
            d += 1;
        }
        steps as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u16, expected: u16) {
        assert!(n >= 1);
        assert!(n <= 1000);
        assert!(expected <= n);
        assert_eq!(Solution::min_steps(n as i32), expected as i32);
    }

    #[test]
    fn ex1() {
        test(3, 3)
    }

    #[test]
    fn ex2() {
        test(1, 0)
    }

    #[test]
    fn myex3() {
        test(4, 4)
    }

    #[test]
    fn myex4() {
        test(5, 5)
    }

    #[test]
    fn myex5() {
        test(6, 5)
    }
}
