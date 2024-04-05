// https://leetcode.com/problems/make-the-string-great/

pub struct Solution;

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut result = std::vec::Vec::with_capacity(s.len());
        for b in s.bytes() {
            match result.last() {
                Some(&val) if std::ops::BitXor::bitxor(b, val) == (b'a' - b'A') => {
                    result.pop();
                }
                _ => result.push(b),
            }
        }
        // Safety: Only uppercase and lowercase English (ascii) letters are
        // accepted into this function.
        unsafe { std::string::String::from_utf8_unchecked(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(s: &str, expected: &str) {
        assert_eq!(Solution::make_good(s.to_string()), expected);
    }

    #[test]
    fn ex1() {
        do_test("leEeetcode", "leetcode")
    }

    #[test]
    fn ex2() {
        do_test("abBAcC", "")
    }

    #[test]
    fn ex3() {
        do_test("s", "s")
    }

    #[test]
    fn myex1() {
        do_test("sS", "")
    }

    #[test]
    fn myex2() {
        do_test("Ss", "")
    }

    #[test]
    fn myex3() {
        do_test("isS", "i")
    }

    #[test]
    fn myex4() {
        do_test("iSs", "i")
    }

    #[test]
    fn myex5() {
        do_test("sSi", "i")
    }

    #[test]
    fn myex6() {
        do_test("Ssi", "i")
    }
}
