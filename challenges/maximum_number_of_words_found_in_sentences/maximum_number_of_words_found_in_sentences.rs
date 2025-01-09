// https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/

pub struct Solution;

// Naive iterator sol'n
// impl Solution {
//     pub fn most_words_found(sentences: Vec<String>) -> i32 {
//         sentences
//             .iter()
//             .map(|x| x.split_whitespace().count())
//             .max()
//             .unwrap() as i32
//     }
// }

// Raw loop sol'n
impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut max = 0;
        for i in 0..sentences.len() {
            let mut count = 1;
            let sentence = sentences[i].as_bytes();
            for j in 0..sentence.len() {
                count += (sentence[j] == b' ') as i32;
            }
            if count > max {
                max = count;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(sentences: &[&str], expected: i32) {
        assert_eq!(
            Solution::most_words_found(sentences.iter().map(|&x| x.to_owned()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(
            &[
                "alice and bob love leetcode",
                "i think so too",
                "this is great thanks very much",
            ],
            6,
        )
    }

    #[test]
    fn ex2() {
        test(&["please wait", "continue to fight", "continue to win"], 3)
    }
}
