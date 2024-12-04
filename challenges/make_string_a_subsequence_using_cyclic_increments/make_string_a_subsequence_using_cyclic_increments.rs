// https://leetcode.com/problems/make-string-a-subsequence-using-cyclic-increments/

pub struct Solution;

impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let str1 = str1.as_bytes();
        let str2 = str2.as_bytes();
        if str1.len() < str2.len() {
            return false;
        }
        let mut i = 0;
        let mut j = 0;
        while i < str1.len() {
            let c1 = str1[i];
            if c1 == str2[j] || (c1 - b'a' + 1) % 26 == str2[j] - b'a' {
                j += 1;
                if j == str2.len() {
                    return true;
                }
            }
            i += 1;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(str1: &str, str2: &str, expected: bool) {
        assert!(str1.len() >= 1);
        assert!(str1.len() <= 100_000);
        assert!(str2.len() >= 1);
        assert!(str2.len() <= 100_000);
        for b in str1.bytes() {
            assert!(b >= b'a' && b <= b'z');
        }
        for b in str2.bytes() {
            assert!(b >= b'a' && b <= b'z');
        }
        assert_eq!(
            Solution::can_make_subsequence(str1.to_string(), str2.to_string()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test("abc", "ad", true)
    }

    #[test]
    fn ex2() {
        test("zc", "ad", true)
    }

    #[test]
    fn ex3() {
        test("ab", "d", false)
    }

    #[test]
    fn discussion_case1() {
        test("c", "b", false)
    }

    #[test]
    fn discussion_case2() {
        test("jsdjsjdj", "adsaa", false)
    }
}
