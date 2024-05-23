// https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones/

pub struct Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let mut last = b'0';
        let mut found_falling_edge = false;
        for c in s.bytes() {
            match c {
                b'0' if last == b'1' => found_falling_edge = true,
                b'1' if found_falling_edge => return false,
                b'0' => (),
                b'1' => (),
                _ => unreachable!(),
            }
            last = c;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: bool) {
        assert!(s.len() > 0);
        assert!(s.len() <= 100);
        for c in s.bytes() {
            assert!(c == b'0' || c == b'1');
        }
        assert_eq!(Solution::check_ones_segment(s.to_string()), expected)
    }

    #[test]
    fn ex1() {
        test("1001", false);
    }

    #[test]
    fn ex2() {
        test("110", true);
    }
}
