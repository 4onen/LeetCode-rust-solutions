// https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/

pub struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        for (i, word) in sentence.split_whitespace().enumerate() {
            if word.starts_with(&search_word) {
                return (i + 1) as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(sentence: &str, search_word: &str, expected: i32) {
        assert_eq!(
            Solution::is_prefix_of_word(sentence.to_string(), search_word.to_string()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test("i love eating burger", "burg", 4);
    }

    #[test]
    fn ex2() {
        test("this problem is an easy problem", "pro", 2);
    }

    #[test]
    fn ex3() {
        test("i am tired", "you", -1);
    }
}
