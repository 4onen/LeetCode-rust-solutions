// https://leetcode.com/problems/counting-words-with-a-given-prefix/

pub struct Solution;

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.into_iter().filter(|w| w.starts_with(&pref)).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(words: &[&str], pref: &str, expected: i32) {
        assert!(words.len() >= 1);
        assert!(words.len() <= 100);
        assert!(pref.len() >= 1);
        assert!(pref.len() <= 100);
        for &b in pref.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        for word in words {
            assert!(word.len() >= 1);
            assert!(word.len() <= 100);
            for &b in word.as_bytes() {
                assert!(b >= b'a');
                assert!(b <= b'z');
            }
        }
        assert_eq!(
            Solution::prefix_count(
                words.iter().map(|&x| x.to_owned()).collect(),
                pref.to_owned()
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&["pay", "attention", "practice", "attend"], "at", 2)
    }

    #[test]
    fn ex2() {
        test(&["leetcode", "win", "loops", "success"], "code", 0);
    }
}
