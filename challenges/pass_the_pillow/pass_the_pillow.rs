// https://leetcode.com/problems/pass-the-pillow/

pub struct Solution;

impl Solution {
    pub const fn pass_the_pillow(n: i32, time: i32) -> i32 {
        assert!(n >= 2);
        let nm1 = n - 1;
        let time_loop = time % (2*nm1);
        if time_loop < nm1 {
            time_loop + 1
        } else {
            2*nm1 - time_loop + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u16, time: u16, expected: u16) {
        assert!(n >= 2);
        assert!(n <= 1000);
        assert!(time >= 1);
        assert!(time <= 1000);
        let result = Solution::pass_the_pillow(n as i32, time as i32);
        assert!(result >= 1);
        let result = result as u16;
        assert_eq!(result, expected);
    }

    #[test]
    fn ex1() {
        test(4, 5, 2);
    }

    #[test]
    fn ex2() {
        test(3, 2, 3);
    }
}
