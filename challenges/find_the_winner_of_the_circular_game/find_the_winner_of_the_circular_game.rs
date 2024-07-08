// https://leetcode.com/problems/find-the-winner-of-the-circular-game/

pub struct Solution;

impl Solution {
    pub const fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut result = 1;
        let mut i = 1;
        while i <= n {
            result = (result + k - 1) % i + 1;
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u16, k: u16, expected: u16) {
        assert!(k >= 1);
        assert!(n >= k);
        assert!(n <= 500);
        let result = Solution::find_the_winner(n as i32, k as i32);
        assert!(result >= 1);
        assert_eq!(result, expected as i32);
    }

    #[test]
    fn ex1() {
        test(5, 2, 3);
    }

    #[test]
    fn ex2() {
        test(6, 5, 1);
    }

    #[test]
    fn discussion_case1() {
        test(8,8,4);
    }

    #[test]
    fn myex1() {
        test(1, 1, 1);
    }

    #[test]
    fn myex500() {
        test(500, 500, 69);
    }
}
