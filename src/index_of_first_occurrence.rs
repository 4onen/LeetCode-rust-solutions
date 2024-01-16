// https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/

pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        } else if needle.len() > haystack.len() {
            return -1;
        }

        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();

        for i in 0..haystack.len() - needle.len() + 1 {
            if &haystack[i..i + needle.len()] == needle {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
            0
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::str_str("leetcode".to_string(), "leeto".to_string()),
            -1
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
    }

    #[test]
    fn myex2() {
        assert_eq!(
            Solution::str_str("aaaaa".to_string(), "bba".to_string()),
            -1
        );
    }

    #[test]
    fn myex3() {
        assert_eq!(Solution::str_str("".to_string(), "".to_string()), 0);
    }

    #[test]
    fn failing_case() {
        assert_eq!(
            Solution::str_str("abb".to_string(), "abaaa".to_string()),
            -1
        );
    }
}
