// https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/?envType=daily-question&envId=2023-12-02

pub struct Solution;

fn can_form_word(word: String, byte_map: &[u8; 26]) -> bool {
    let mut byte_map = byte_map.clone();
    for byte in word.into_bytes() {
        let index = (byte - b'a') as usize;
        if byte_map[index] == 0 {
            return false;
        } else {
            byte_map[index] -= 1;
        }
    }
    true
}

fn count_word_chars(word: String) -> [u8; 26] {
    let mut byte_map = [0; 26];
    for byte in word.into_bytes() {
        byte_map[(byte - b'a') as usize] += 1;
    }
    byte_map
}

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let byte_map = count_word_chars(chars);

        words
            .into_iter()
            .filter_map(|word| {
                let len = word.len() as i32;
                if can_form_word(word, &byte_map) {
                    Some(len)
                } else {
                    None
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::count_characters(
                vec![
                    "cat".to_string(),
                    "bt".to_string(),
                    "hat".to_string(),
                    "tree".to_string()
                ],
                "atach".to_string()
            ),
            6
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::count_characters(
                vec![
                    "hello".to_string(),
                    "world".to_string(),
                    "leetcode".to_string()
                ],
                "welldonehoneyr".to_string()
            ),
            10
        );
    }
}
