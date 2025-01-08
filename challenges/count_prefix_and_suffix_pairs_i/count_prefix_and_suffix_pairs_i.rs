// https://leetcode.com/problems/count-prefix-and-suffix-pairs-i/

pub struct Solution;

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        fn is_prefix_and_suffix(s1: &str, s2: &str) -> bool {
            s2.starts_with(s1) && s2.ends_with(s1)
        }
        let mut seen = 0;
        for i in 0..words.len() as u8 {
            for j in i + 1..words.len() as u8 {
                seen += is_prefix_and_suffix(&words[i as usize], &words[j as usize]) as i32;
            }
        }
        seen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(words: &[&str], expected: i32) {
        assert!(words.len() >= 1);
        assert!(words.len() <= 50);
        for word in words {
            assert!(word.len() >= 1);
            assert!(word.len() <= 10);
            for &b in word.as_bytes() {
                assert!(b >= b'a');
                assert!(b <= b'z');
            }
        }
        assert_eq!(
            Solution::count_prefix_suffix_pairs(words.iter().map(|&x| x.to_owned()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&["a", "aba", "ababa", "aa"], 4)
    }

    #[test]
    fn ex2() {
        test(&["pa", "papa", "ma", "mama"], 2)
    }

    #[test]
    fn ex3() {
        test(&["abab", "ab"], 0)
    }

    #[test]
    fn failing_case1() {
        // Forgot 50^2 > 255
        test(
            &[
                "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa",
                "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa",
                "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa",
                "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa",
                "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa",
                "aaaaa", "aaaaa", "aaaaa", "aaaaa", "aaaaa",
            ],
            1225,
        )
    }
}
