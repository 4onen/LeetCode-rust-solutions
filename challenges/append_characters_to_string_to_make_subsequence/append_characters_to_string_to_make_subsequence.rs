// https://leetcode.com/problems/append-characters-to-string-to-make-subsequence/

pub struct Solution;

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        assert!(s.len() > 0);
        assert!(t.len() > 0);
        assert!(s.len() <= 100_000);
        assert!(t.len() <= 100_000);
        let mut next_to_match = 0i32;
        for &b in s {
            if b == t[next_to_match as usize] {
                next_to_match += 1;
                if next_to_match >= t.len() as i32 {
                    return 0;
                }
            }
        }
        t.len() as i32 - next_to_match
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, t: &str, expected: i32) {
        assert!(expected >= 0);
        assert!(s.len() > 0);
        assert!(t.len() > 0);
        assert_eq!(Solution::append_characters(s.to_owned(), t.to_owned()), expected);
    }

    #[test]
    fn ex1() {
        test("coaching", "coding", 4)
    }

    #[test]
    fn ex2() {
        test("abcde", "a", 0)
    }

    #[test]
    fn ex3() {
        test("z", "abcde", 5)
    }
}
