// https://leetcode.com/problems/sentence-similarity-iii/

pub struct Solution;

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let tokens1 = sentence1.split(' ').collect::<Vec<_>>();
        let tokens2 = sentence2.split(' ').collect::<Vec<_>>();
        let mut prefix = 0;
        while prefix < tokens1.len() && prefix < tokens2.len() && tokens1[prefix] == tokens2[prefix]
        {
            prefix += 1;
        }
        if prefix == tokens1.len() || prefix == tokens2.len() {
            return true;
        }
        let mut suffix1 = tokens1.len();
        let mut suffix2 = tokens2.len();
        while suffix1 > 0 && suffix2 > 0 && tokens1[suffix1 - 1] == tokens2[suffix2 - 1] {
            suffix1 -= 1;
            suffix2 -= 1;
        }
        if suffix1 == 0 || suffix2 == 0 {
            return true;
        }
        prefix >= suffix1 || prefix >= suffix2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(sentence1: &str, sentence2: &str, expected: bool) {
        assert_eq!(
            Solution::are_sentences_similar(sentence1.to_string(), sentence2.to_string()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test("My name is Haley", "My Haley", true)
    }

    #[test]
    fn ex2() {
        test("Frog cool", "Frogs are cool", false)
    }

    #[test]
    fn ex3() {
        test("of", "A lot of words", false)
    }

    #[test]
    fn ex4() {
        test("Eating right now", "Eating", true)
    }

    #[test]
    fn discussion_case1() {
        test("A", "a A b A", true)
    }

    #[test]
    fn discussion_case2() {
        test("hello world python", "python", true)
    }

    #[test]
    fn discussion_case3() {
        test("hello world python", "hello", true)
    }

    #[test]
    fn discussion_case4() {
        test("hello world python", "hello python", true)
    }

    #[test]
    fn discussion_case5() {
        test("hello world python", "world", false)
    }

    #[test]
    fn discussion_case6() {
        test("z z z z", "zz z", false)
    }

    #[test]
    fn discussion_case6_1() {
        test("z z z z", "z z", true)
    }

    #[test]
    fn discussion_case7() {
        test("DN PD", "D", false)
    }

    #[test]
    fn discussion_case8() {
        test("x", "y", false)
    }

    #[test]
    fn discussion_case9() {
        test("A A AAa", "A AAa", true)
    }

    #[test]
    fn failing_case1() {
        test("A B C D B B", "A B B", true)
    }

    #[test]
    fn failing_case1_1() {
        test("A B C D B B", "A B C B", true)
    }
}
