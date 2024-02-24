// https://leetcode.com/problems/replace-all-s-to-avoid-consecutive-repeating-characters/

pub struct Solution;

impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut s = s.into_bytes();

        for i in 0..s.len() {
            if s[i] == b'?' {
                let prev = 64 | if i == 0 { b'?' } else { s[i - 1] };
                let next = 64 | s.get(i + 1).copied().unwrap_or(b'?');

                s[i] = match (prev.min(next), prev.max(next)) {
                    (b'a', b'b') => b'c',
                    (b'a', _) => b'b',
                    _ => b'a',
                }
            }
        }

        unsafe { String::from_utf8_unchecked(s) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::modify_string("?zs".to_string()),
            "azs".to_string()
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::modify_string("ubv?w".to_string()),
            "ubvaw".to_string()
        );
    }

    #[test]
    fn failing_case1() {
        assert_eq!(Solution::modify_string("??".to_string()), "ab".to_string());
    }

    #[test]
    fn myex1() {
        assert_eq!(
            Solution::modify_string("???".to_string()),
            "aba".to_string()
        );
    }

    #[test]
    fn myex2() {
        assert_eq!(
            Solution::modify_string("????".to_string()),
            "abab".to_string()
        );
    }

    #[test]
    fn myex3() {
        assert_eq!(
            Solution::modify_string("?a?".to_string()),
            "bab".to_string()
        );
    }
}
