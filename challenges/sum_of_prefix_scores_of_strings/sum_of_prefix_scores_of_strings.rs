// https://leetcode.com/problems/sum-of-prefix-scores-of-strings/

pub struct Solution;

// Brute force
impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let answer = words
            .iter()
            .map(|word| {
                // Remember, score is the sum of, for each prefix of this word,
                // how many words in the list have that prefix.
                // This is the same as asking how many letters from the start
                // in common this word has with each word in the list.
                let mut score = 0;
                for word2 in words.iter() {
                    score += std::iter::Iterator::zip(word.bytes(), word2.bytes())
                        .take_while(|(a, b)| a == b)
                        .count() as i32;
                }
                score
            })
            .collect();
        answer
    }
}

// Cheap trie (MLE)
// impl Solution {
//     pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
//         struct Trie {
//             children: std::vec::Vec<Option<Trie>>,
//             count: i32,
//         }
//         impl Trie {
//             fn new() -> Trie {
//                 let children = std::iter::repeat_with(|| None).take(26).collect();
//                 Trie { children, count: 0 }
//             }
//             fn insert(&mut self, word: &str) {
//                 let mut node = self;
//                 for b in word.bytes() {
//                     dbg!(b as char);
//                     let index = (b - b'a') as usize;
//                     node = node.children[index].get_or_insert_with(Trie::new);
//                     node.count += 1;
//                 }
//             }
//             fn score(&self, word: &str) -> i32 {
//                 let mut node = self;
//                 let mut score = 0;
//                 for b in word.bytes() {
//                     let index = (b - b'a') as usize;
//                     node = match &node.children[index] {
//                         Some(child) => child,
//                         None => break,
//                     };
//                     score += node.count;
//                 }
//                 score
//             }
//         }
//         let mut trie = Trie::new();
//         for word in words.iter() {
//             trie.insert(word);
//         }
//         let answer = words.iter().map(|word| trie.score(word)).collect();
//         answer
//     }
// }

// Hash map of prefix counts
// impl Solution {
//     pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
//         let mut prefix_counts: std::collections::HashMap<&str, i32> =
//             std::collections::HashMap::new();
//         for word in words.iter() {
//             for i in 0..word.bytes().len() {
//                 *prefix_counts.entry(&word[..i + 1]).or_insert(0) += 1;
//             }
//         }
//         let answer = words
//             .iter()
//             .map(|word| {
//                 let mut score = 0;
//                 for i in 0..word.bytes().len() {
//                     score += *prefix_counts.get(&word[..i + 1]).unwrap();
//                 }
//                 score
//             })
//             .collect();
//         answer
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(words: &[&str], expected: &[i32]) {
        assert!(words.len() == expected.len());
        assert!(words.len() >= 1);
        assert!(words.len() <= 1000);
        for word in words {
            assert!(word.len() >= 1);
            assert!(word.len() <= 1000);
            for b in word.bytes() {
                assert!(b >= b'a' && b <= b'z');
            }
        }
        let output = Solution::sum_prefix_scores(words.iter().map(|s| s.to_string()).collect());
        assert_eq!(output, expected);
    }

    #[test]
    fn ex1() {
        test(&["abc", "ab", "bc", "b"], &[5, 4, 3, 2])
    }

    #[test]
    fn ex1_1() {
        test(&["abc", "ab", "bc"], &[5, 4, 2])
    }

    #[test]
    fn ex1_2() {
        test(&["abc", "ab"], &[5, 4])
    }

    #[test]
    fn ex1_3() {
        test(&["abc"], &[3])
    }

    #[test]
    fn ex1_4() {
        test(&["ab", "bc"], &[2, 2])
    }

    #[test]
    fn ex2() {
        test(&["abcd"], &[4])
    }

    #[test]
    fn ex2_1() {
        test(&["abcd", "bcd"], &[4, 3])
    }

    #[test]
    fn discussion_case0() {
        test(&["a"], &[1])
    }

    #[test]
    fn discussion_case2() {
        let a = "aaaaaaa";
        let b = "aaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let mut words = [a; 1000];
        words[999] = b;
        let mut expected = [7000; 1000];
        expected[999] = 7000 + 28 - 7;
        test(&words, &expected)
    }

    #[test]
    fn tle_case1() {
        let input = include_str!("tle_case1.txt");
        let words = input[2..input.len() - 3]
            .split("\",\"")
            .collect::<Vec<&str>>();
        // This is just a doesn't-crash test
        assert!(words.len() >= 1);
        assert!(words.len() <= 1000);
        for word in &words {
            assert!(word.len() >= 1);
            assert!(word.len() <= 1000);
            for b in word.bytes() {
                assert!(b >= b'a' && b <= b'z');
            }
        }
        Solution::sum_prefix_scores(words.iter().map(|s| s.to_string()).collect());
    }
}
