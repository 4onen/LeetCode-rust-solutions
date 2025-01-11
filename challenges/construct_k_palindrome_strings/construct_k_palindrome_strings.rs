// https://leetcode.com/problems/construct-k-palindrome-strings/

pub struct Solution;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false;
        }
        let letters = {let mut letters = [0u8; 26];
            for &b in s.as_bytes() {
                letters[(b-b'a') as usize] += 1;
            }
            letters
        };
        let mut odds = 0;
        for i in 0..26 {
            odds += letters[i] & 1;
        }
        if odds as i32 > k {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, k: i32, expected: bool) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 100_000);
        for &b in s.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        assert!(k >= 1);
        assert!(k <= 100_000);
        assert_eq!(Solution::can_construct(s.to_owned(), k), expected);
    }

    #[test]
    fn ex1() {
        test("annabelle", 2, true)
    }

    #[test]
    fn ex2() {
        test("leetcode", 3, false)
    }

    #[test]
    fn ex3() {
        test("true", 4, true)
    }

    #[test]
    fn myex1() {
        test("fall", 3, true)
    }

    #[test]
    fn myex2() {
        test("falls", 3, true)
    }

    #[test]
    fn myex3() {
        test("fallis", 3, false)
    }

    #[test]
    fn myex4() {
        test("fall",5,false)
    }

    #[test]
    fn discussion_case1() {
        test("a",1,true)
    }

    #[test]
    fn discussion_case2() {
        test("rraceca", 3, true)
    }
}
