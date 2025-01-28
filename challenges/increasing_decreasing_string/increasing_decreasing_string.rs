// https://leetcode.com/problems/increasing-decreasing-string/

pub struct Solution;

impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut s = s.into_bytes();
        let original_len = s.len() as u16;
        let mut chars = [0u16; 26];
        for b in s.drain(..) {
            chars[(b - b'a') as usize] += 1;
        }
        loop {
            let mut i = 0;
            while i < chars.len() as u8 {
                if chars[i as usize] > 0 {
                    s.push(i + b'a');
                    chars[i as usize] -= 1;
                }
                i += 1;
            }
            while i > 0 {
                i -= 1;
                if chars[i as usize] > 0 {
                    s.push(i + b'a');
                    chars[i as usize] -= 1;
                }
            }
            if s.len() as u16 == original_len {
                break unsafe { std::string::String::from_utf8_unchecked(s) };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: &str) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 500);
        for &b in s.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        assert_eq!(Solution::sort_string(s.to_string()), expected);
    }

    #[test]
    fn ex1() {
        test("aaaabbbbcccc", "abccbaabccba")
    }

    #[test]
    fn ex2() {
        test("rat", "art")
    }

    #[test]
    fn discussion_case1() {
        // tc3
        test("leetcode", "cdelotee")
    }
}
