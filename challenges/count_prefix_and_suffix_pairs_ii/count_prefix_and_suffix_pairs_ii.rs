// https://leetcode.com/problems/count-prefix-and-suffix-pairs-ii/

pub struct Solution;

// Easy sibling problem sol'n (Obviously not fast enough)
// impl Solution {
//     pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i64 {
//         fn is_prefix_and_suffix(s1: &str, s2: &str) -> bool {
//             s2.starts_with(s1) && s2.ends_with(s1)
//         }
//         let mut seen = 0;
//         for i in 0..words.len() as u32 {
//             for j in i + 1..words.len() as u32 {
//                 seen += is_prefix_and_suffix(&words[i as usize], &words[j as usize]) as i64;
//             }
//         }
//         seen
//     }
// }

// Trie sol'n (Based on easy problem's:
// https://leetcode.com/problems/count-prefix-and-suffix-pairs-i/solutions/6247292/simple-trie-fast-onm-solution-by-harisht-p2p7/
// impl Solution {
//     pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i64 {
//         struct Node {
//             children: std::collections::HashMap<(u8, u8), Node>,
//             count: i64,
//         }
//         impl Node {
//             fn new() -> Node {
//                 Node {
//                     count: 0,
//                     children: std::collections::HashMap::new(),
//                 }
//             }
//             fn update_count(&mut self, word: &[u8]) -> i64 {
//                 let len = word.len() as u32;
//                 let mut res = 0;
//                 let mut temp = self;
//                 for i in 0..len {
//                     let j = len - 1 - i;
//                     let key = (word[i as usize], word[j as usize]);
//                     temp = temp.children.entry(key).or_insert_with(Node::new);
//                     res += temp.count;
//                 }
//                 temp.count += 1;
//                 res
//             }
//         }
//         let mut res = 0;
//         let mut trie = Node::new();
//         for word in words {
//             res += trie.update_count(word.as_bytes());
//         }
//         res
//     }
// }

// Adjust to prevent iterative stack overflow on local computer
// Looks like this is due to deallocating all of the &mut
impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i64 {
        struct Node {
            children: std::collections::HashMap<(u8, u8), Node>,
            count: i64,
        }
        let mut res = 0;
        let mut trie = Node {
            children: std::collections::HashMap::new(),
            count: 0,
        };
        for word in words {
            let word = word.as_bytes();
            let len = word.len() as u32;
            let mut temp = &mut trie;
            for i in 0..len {
                let j = len - 1 - i;
                let key = (word[i as usize], word[j as usize]);
                temp = temp.children.entry(key).or_insert_with(|| Node {
                    children: std::collections::HashMap::new(),
                    count: 0,
                });
                res += temp.count;
            }
            temp.count += 1;
        }
        std::mem::forget(trie); // Letting this be destroyed blows the stack.
                                // Leaking mem is fine in a LeetCode env.
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(words: &[&str], expected: i64) {
        assert!(words.len() >= 1);
        assert!(words.len() <= 100_000);
        for word in words {
            assert!(word.len() >= 1);
            assert!(word.len() <= 100_000);
            for &b in word.as_bytes() {
                assert!(b >= b'a');
                assert!(b <= b'z');
            }
        }
        assert!(words.iter().map(|&x| x.len()).sum::<usize>() <= 500_000);
        let result =
            Solution::count_prefix_suffix_pairs(words.iter().map(|&x| x.to_owned()).collect());
        assert_eq!(result, expected);
    }

    #[test]
    fn ex1() {
        test(&["a", "aba", "ababa", "aa"], 4)
    }

    #[test]
    fn ex2() {
        test(&["pa", "papa", "ma", "mama"], 2)
    }

    #[test]
    fn ex3() {
        test(&["abab", "ab"], 0)
    }

    #[test]
    fn discussion_case1() {
        test(&["b", "a", "b", "a", "b"], 4)
    }

    #[test]
    fn my_extreme_ex1() {
        // 1 word, 100_000 characters.
        const WORD: &str = unsafe { std::str::from_utf8_unchecked(&[b'a'; 100_000]) };
        const WORDS: &[&str] = &[WORD; 1];
        test(WORDS, 0);
    }

    #[test]
    fn my_extreme_ex5() {
        // 5 words, 100_000 characters each.
        const WORD: &str = unsafe { std::str::from_utf8_unchecked(&[b'a'; 100_000]) };
        test(&[WORD; 5], 10) // 5 choose 2
    }

    #[test]
    fn my_extreme_ex10() {
        // 10 words, 50_000 characters each.
        const WORD: &str = unsafe { std::str::from_utf8_unchecked(&[b'a'; 50_000]) };
        test(&[WORD; 10], 45) // 10 choose 2
    }

    #[test]
    fn my_extreme_ex100() {
        // 100 words, 5_000 characters each.
        const WORD: &str = unsafe { std::str::from_utf8_unchecked(&[b'a'; 5_000]) };
        test(&[WORD; 100], 4950) // 100 choose 2
    }

    #[test]
    fn my_extreme_ex1000() {
        // 1_000 words, 500 characters each.
        let word: &str = unsafe { std::str::from_utf8_unchecked(&[b'a'; 500]) };
        test(&[word; 1000], 499_500) // 1000 choose 2
    }

    #[test]
    fn my_extreme_ex10000() {
        // 10_000 words, 50 characters each.
        let word: &str = unsafe { std::str::from_utf8_unchecked(&[b'a'; 50]) };
        test(&[word; 10_000], 49995000) // 10_000 choose 2
    }

    #[test]
    fn my_extreme_ex100000() {
        // 100_000 words, 5 characters each.
        let word: &str = unsafe { std::str::from_utf8_unchecked(&[b'a'; 5]) };
        test(&[word; 100_000], 4999950000) // 100_000 choose 2
    }
}
