// https://leetcode.com/problems/redistribute-characters-to-make-all-strings-equal/

pub struct Solution;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut counts = [0u8; 26];
        let len = words.len() as u8;
        for word in words {
            for c in word.into_bytes() {
                let count = &mut counts[(c - b'a') as usize];
                *count = (*count + 1) % len;
            }
        }

        counts.iter().all(|c| *c == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::make_equal(vec![
                "abc".to_string(),
                "aabc".to_string(),
                "bc".to_string()
            ]),
            true
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::make_equal(vec!["ab".to_string(), "a".to_string()]),
            false
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(
            Solution::make_equal(vec!["ab".to_string(), "a".to_string(), "b".to_string()]),
            false
        );
    }
}
