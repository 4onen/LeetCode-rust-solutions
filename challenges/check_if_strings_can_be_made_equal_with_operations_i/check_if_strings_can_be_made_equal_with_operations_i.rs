// https://leetcode.com/problems/check-if-strings-can-be-made-equal-with-operations-i/

pub struct Solution;

impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        fn straighten(s: &mut [u8]) {
            if s[0] > s[2] {
                s.swap(0, 2);
            }
            if s[1] > s[3] {
                s.swap(1, 3);
            }
        }
        let mut s1 = s1.into_bytes();
        let mut s2 = s2.into_bytes();
        assert_eq!(s1.len(), 4);
        assert_eq!(s1.len(), s2.len());
        straighten(&mut s1);
        straighten(&mut s2);
        s1 == s2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s1: &str, s2: &str, expected: bool) {
        assert_eq!(Solution::can_be_equal(s1.to_string(), s2.to_string()), expected);
    }

    #[test]
    fn ex1() {
        test(
            "abcd",
            "cdab",
            true
        )
    }

    #[test]
    fn ex2() {
        test(
            "abcd",
            "dacb",
            false
        )
    }

    #[test]
    fn discussion_case1() {
        test(
            "zzon",
            "zozn",
            false
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            "bnxw",
            "bwxn",
            true
        )
    }
}
