// https://leetcode.com/problems/word-ladder/

pub struct Solution;

// Initial sol'n
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        assert!(begin_word.len() <= 10);
        assert_eq!(begin_word.len(), end_word.len());
        assert_ne!(begin_word, end_word);
        assert!(word_list.len() <= 5_000);
        if !word_list.contains(&end_word) {
            return 0;
        }
        const fn differ_by_one(word1: &[u8], word2: &[u8]) -> bool {
            let mut diff: u8 = 0;
            let mut i: u8 = 0;
            while i < word1.len() as u8 {
                if word1[i as usize] != word2[i as usize] {
                    diff += 1;
                    if diff > 1 {
                        return false;
                    }
                }
                i += 1;
            }
            diff == 1
        }
        type WordListIdx = u16;
        let mut visited = vec![false; word_list.len()];
        let mut queue: std::collections::VecDeque<WordListIdx> = std::collections::VecDeque::new();
        for i in 0..word_list.len() as u16 {
            if differ_by_one(begin_word.as_bytes(), word_list[i as usize].as_bytes()) {
                queue.push_back(i);
                visited[i as usize] = true;
            }
        }
        let mut steps = 1;
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let idx = queue.pop_front().unwrap();
                if word_list[idx as usize] == end_word {
                    return steps + 1;
                }
                for i in 0..word_list.len() as u16 {
                    if !visited[i as usize]
                        && differ_by_one(
                            word_list[idx as usize].as_bytes(),
                            word_list[i as usize].as_bytes(),
                        )
                    {
                        queue.push_back(i);
                        visited[i as usize] = true;
                    }
                }
            }
            steps += 1;
        }
        0
    }
}

// Lol, this optimized solution is slower than the initial solution
// impl Solution {
//     pub fn ladder_length(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32 {
//         assert!(begin_word.len() <= 10);
//         assert_eq!(begin_word.len(), end_word.len());
//         assert_ne!(begin_word, end_word);
//         assert!(word_list.len() <= 5_000);
//         {
//             let Some(end_word_idx) = word_list.iter().position(|s| s == &end_word) else {
//                 return 0;
//             };
//             word_list.swap_remove(end_word_idx);
//         }
//         const fn differ_by_one(word1: &[u8], word2: &[u8]) -> bool {
//             let mut diff: u8 = 0;
//             let mut i: u8 = 0;
//             while i < word1.len() as u8 {
//                 if word1[i as usize] != word2[i as usize] {
//                     diff += 1;
//                     if diff > 1 {
//                         return false;
//                     }
//                 }
//                 i += 1;
//             }
//             diff == 1
//         }
//         let begin_word = begin_word.as_bytes();
//         let end_word = end_word.as_bytes();
//         if differ_by_one(begin_word, end_word) {
//             return 2;
//         }
//         type WordListIdx = u16;
//         let mut visited = vec![false; word_list.len()];
//         let mut queue: std::collections::VecDeque<WordListIdx> = std::collections::VecDeque::new();
//         for i in 0..word_list.len() as u16 {
//             if differ_by_one(begin_word, word_list[i as usize].as_bytes()) {
//                 queue.push_back(i);
//                 visited[i as usize] = true;
//             }
//         }
//         let mut steps = 3;
//         while !queue.is_empty() {
//             for _ in 0..queue.len() {
//                 let idx = queue.pop_front().unwrap();
//                 let word = word_list[idx as usize].as_bytes();
//                 if differ_by_one(word, end_word) {
//                     return steps;
//                 }
//                 for i in 0..word_list.len() as u16 {
//                     if !visited[i as usize] && differ_by_one(word, word_list[i as usize].as_bytes())
//                     {
//                         queue.push_back(i);
//                         visited[i as usize] = true;
//                     }
//                 }
//             }
//             steps += 1;
//         }
//         0
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(begin_word: &str, end_word: &str, word_list: &[&str], expected: u16) {
        let begin_word = begin_word.to_string();
        let end_word = end_word.to_string();
        let word_list = word_list.iter().map(|s| s.to_string()).collect();
        assert_eq!(
            Solution::ladder_length(begin_word, end_word, word_list),
            expected as i32
        );
    }

    #[test]
    fn ex1() {
        test("hit", "cog", &["hot", "dot", "dog", "lot", "log", "cog"], 5);
    }

    #[test]
    fn ex2() {
        test("hit", "cog", &["hot", "dot", "dog", "lot", "log"], 0);
    }

    #[test]
    fn discussion_case1() {
        test(
            "cat",
            "fin",
            &[
                "ion", "rev", "che", "ind", "lie", "wis", "oct", "ham", "jag", "ray", "nun", "ref",
                "wig", "jul", "ken", "mit", "eel", "paw", "per", "ola", "pat", "old", "maj", "ell",
                "irk", "ivy", "beg", "fan", "rap", "sun", "yak", "sat", "fit", "tom", "fin", "bug",
                "can", "hes", "col", "pep", "tug", "ump", "arc", "fee", "lee", "ohs", "eli", "nay",
                "raw", "lot", "mat", "egg", "cat", "pol", "fat", "joe", "pis", "dot", "jaw", "hat",
                "roe", "ada", "mac",
            ],
            4,
        );
    }

    #[test]
    fn discussion_case2() {
        test("hot", "dog", &["hot", "dog"], 0)
    }

    #[test]
    fn discussion_case2_variant1() {
        test("hot", "dog", &["hot", "dog", "dot"], 3)
    }

    #[test]
    fn failing_case1() {
        test("a", "c", &["a", "b", "c"], 2)
    }
}
