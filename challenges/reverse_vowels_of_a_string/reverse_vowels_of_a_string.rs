// https://leetcode.com/problems/reverse-vowels-of-a-string/

pub struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        const VOWELS: &[u8] = "aeiouAEIOU".as_bytes();
        let mut bs = s.into_bytes();
        let mut l = 0;
        let mut r = bs.len() - 1;
        loop {
            while l < r && !VOWELS.contains(&bs[l]) {
                l += 1
            }
            while l < r && !VOWELS.contains(&bs[r]) {
                r -= 1
            }
            if l >= r {
                break;
            }
            bs.swap(l, r);
            l += 1;
            r -= 1;
        }
        unsafe { std::string::String::from_utf8_unchecked(bs) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: &str) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 300_000);
        assert_eq!(Solution::reverse_vowels(s.to_owned()), expected);
    }

    #[test]
    fn ex1() {
        test("IceCreAm", "AceCreIm")
    }

    #[test]
    fn ex2() {
        test("leetcode", "leotcede")
    }

    #[test]
    fn discussion_case1() {
        test("race car", "race car")
    }

    #[test]
    fn discussion_case2() {
        test("Aa", "aA")
    }

    #[test]
    fn discussion_case3() {
        test(
            "Yo! Bottoms up, U.S. Motto, boy!",
            "Yo! Bottoms Up, u.S. Motto, boy!",
        )
    }
}
