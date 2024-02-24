// https://leetcode.com/problems/length-of-last-word/

pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let bytes = s.as_bytes();
        let last_word_end = bytes.iter().rposition(|&x| x != b' ');
        match last_word_end {
            Some(end) => {
                let last_word_start = bytes[..=end].iter().rposition(|&x| x == b' ');
                match last_word_start {
                    Some(start) => (end - start) as i32,
                    None => (end + 1) as i32,
                }
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".to_string()),
            6
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::length_of_last_word("a".to_string()), 1);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::length_of_last_word("a ".to_string()), 1);
    }

    #[test]
    fn myex3() {
        assert_eq!(Solution::length_of_last_word(" a".to_string()), 1);
    }

    #[test]
    fn myex4() {
        assert_eq!(Solution::length_of_last_word("a a".to_string()), 1);
    }

    #[test]
    fn myex5() {
        assert_eq!(Solution::length_of_last_word("a a ".to_string()), 1);
    }
}
