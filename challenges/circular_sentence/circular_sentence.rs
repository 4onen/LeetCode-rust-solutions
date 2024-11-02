// https://leetcode.com/problems/circular-sentence/

pub struct Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let sentence = sentence.as_bytes();
        let mut last = sentence.last().unwrap();
        sentence.split(|&c| c == b' ').all(|word| {
            let res = word.first().unwrap() == last;
            last = word.last().unwrap();
            res
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &str, expected: bool) {
        assert_eq!(Solution::is_circular_sentence(input.to_string()), expected);
    }

    #[test]
    fn ex1() {
        test("leetcode exercises sound delightful", true)
    }

    #[test]
    fn ex2() {
        test("eetcode", true)
    }

    #[test]
    fn ex3() {
        test("Leetcode is cool", false)
    }
}
