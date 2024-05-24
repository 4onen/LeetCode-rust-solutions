// https://leetcode.com/problems/maximum-score-words-formed-by-letters/

pub struct Solution;

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        fn dfs(words: &[String], letters: &[u8], score: &[i32], current_score: i32, i: u8) -> i32 {
            if i == words.len() as u8 {
                return current_score;
            }
            let max_score = dfs(words, letters, score, current_score, i + 1);
            let mut new_letters = letters.to_vec();
            let mut new_score = current_score;
            for &c in words[i as usize].as_bytes() {
                let idx = (c - b'a') as u8;
                if new_letters[idx as usize] == 0 {
                    return max_score;
                }
                new_letters[idx as usize] -= 1;
                new_score += score[idx as usize];
            }
            std::cmp::max(max_score, dfs(words, &new_letters, score, new_score, i + 1))
        }
        let mut letters_cnts = vec![0u8; 26];
        for c in letters {
            letters_cnts[(c as u8 - b'a') as usize] += 1;
        }
        dfs(&words, &letters_cnts, &score, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(words: &[&str], letters: &[char], score: &[i32], expected: i32) {
        assert_eq!(
            Solution::max_score_words(
                words.iter().map(|&s| s.to_string()).collect(),
                letters.iter().copied().collect(),
                score.iter().copied().collect(),
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(
            &["dog", "cat", "dad", "good"],
            &['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
            &[
                1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            23,
        )
    }

    #[test]
    fn ex2() {
        test(
            &["xxxz", "ax", "bx", "cx"],
            &['z', 'a', 'b', 'c', 'x', 'x', 'x'],
            &[
                4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10,
            ],
            27,
        )
    }

    #[test]
    fn ex3() {
        test(
            &["leetcode"],
            &['l', 'e', 't', 'c', 'o', 'd'],
            &[
                0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
            ],
            0,
        )
    }
}
