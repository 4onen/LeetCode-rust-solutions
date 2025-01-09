// https://leetcode.com/problems/count-prefixes-of-a-given-string/

pub struct Solution;

// Naive iterator sol'n
// impl Solution {
//     pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
//         words.into_iter().filter(|pref| s.starts_with(pref)).count() as i32
//     }
// }

// Optimization -- iter instead of into_iter
impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        words.iter().filter(|&pref| s.starts_with(pref)).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(words: &[&str], s: &str, expected: i32) {
        assert!(words.len() >= 1);
        assert!(words.len() <= 1000);
        assert!(s.len() >= 1);
        assert!(s.len() <= 10);
        for &b in s.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        for word in words {
            assert!(word.len() >= 1);
            assert!(word.len() <= 10);
            for &b in word.as_bytes() {
                assert!(b >= b'a');
                assert!(b <= b'z');
            }
        }
        assert_eq!(
            Solution::count_prefixes(words.iter().map(|&x| x.to_owned()).collect(), s.to_owned()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&["a", "b", "c", "ab", "bc", "abc"], "abc", 3)
    }

    #[test]
    fn ex2() {
        test(&["a", "a"], "aa", 2)
    }
}
