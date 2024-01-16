// https://leetcode.com/problems/determine-if-two-strings-are-close/

pub struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut word1_count = [0u16; 26];
        for b in word1.bytes() {
            word1_count[(b - b'a') as usize] += 1;
        }

        let mut word2_count = [0u16; 26];
        for b in word2.bytes() {
            word2_count[(b - b'a') as usize] += 1;
        }

        for (&a, &b) in std::iter::zip(word1_count.iter(), word2_count.iter()) {
            if (a > 0) != (b > 0) {
                return false;
            }
        }

        word1_count.sort_unstable();
        word2_count.sort_unstable();

        word1_count == word2_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::close_strings("abc".to_string(), "bca".to_string()),
            true
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::close_strings("a".to_string(), "aa".to_string()),
            false
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::close_strings("cabbba".to_string(), "abbccc".to_string()),
            true
        );
    }
}
