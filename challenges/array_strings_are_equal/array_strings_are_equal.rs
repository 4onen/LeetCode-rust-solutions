pub struct Solution;

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        fn byte_iter(word: Vec<String>) -> impl Iterator<Item = u8> {
            word.into_iter()
                .map(|s| s.into_bytes().into_iter())
                .flatten()
        }

        byte_iter(word1).eq(byte_iter(word2))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::array_strings_are_equal(
                vec!["ab".to_string(), "c".to_string()],
                vec!["a".to_string(), "bc".to_string()]
            ),
            true
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::array_strings_are_equal(
                vec!["a".to_string(), "cb".to_string()],
                vec!["ab".to_string(), "c".to_string()]
            ),
            false
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::array_strings_are_equal(
                vec!["abc".to_string(), "d".to_string(), "defg".to_string()],
                vec!["abcddefg".to_string()]
            ),
            true
        );
    }
}
