// https://leetcode.com/problems/count-common-words-with-one-occurrence/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
//         let mut map1 = std::collections::HashMap::new();
//         for word in words1 {
//             map1.entry(word).and_modify(|v| *v = false).or_insert(true);
//         }
//         let mut map2 = std::collections::HashMap::new();
//         for word in words2 {
//             if let Some(true) = map1.get(&word) {
//                 map2.entry(word).and_modify(|v| *v = false).or_insert(true);
//             }
//         }
//         let mut count = 0;
//         for v in map2.into_values() {
//             count += v as i32;
//         }
//         count
//     }
// }

// Shorter-first optimized sol'n
// impl Solution {
//     pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
//         let mut map1 = std::collections::HashMap::new();
//         let (first, second) = if words1.len() < words2.len() {
//             (words1, words2)
//         } else {
//             (words2, words1)
//         };
//         for word in first {
//             map1.entry(word).and_modify(|v| *v = false).or_insert(true);
//         }
//         let mut map2 = std::collections::HashMap::new();
//         for word in second {
//             if let Some(true) = map1.get(&word) {
//                 map2.entry(word).and_modify(|v| *v = false).or_insert(true);
//             }
//         }
//         let mut count = 0;
//         for v in map2.into_values() {
//             count += v as i32;
//         }
//         count
//     }
// }

// One-hash-map sol'n
impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut map = std::collections::HashMap::new();
        let (first, second) = if words1.len() < words2.len() {
            (words1, words2)
        } else {
            (words2, words1)
        };
        for word in first {
            map.entry(word)
                .and_modify(|(first, _)| *first = true)
                .or_insert((false, 0u8));
        }
        for word in second {
            if let Some((false, second)) = map.get_mut(&word) {
                *second += 1;
            }
        }
        map.into_values().filter(|(_, second)| *second == 1).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(words1: &[&str], words2: &[&str], expected: i32) {
        assert!(words1.len() >= 1);
        assert!(words2.len() >= 1);
        assert!(words1.len() <= 1000);
        assert!(words2.len() <= 1000);
        for word in words1 {
            assert!(word.len() >= 1);
            assert!(word.len() <= 30);
            for b in word.bytes() {
                assert!(b.is_ascii_lowercase());
            }
        }
        for word in words2 {
            assert!(word.len() >= 1);
            assert!(word.len() <= 30);
            for b in word.bytes() {
                assert!(b.is_ascii_lowercase());
            }
        }
        assert!(expected >= 0);
        assert!(expected <= 1000);
        let words1 = words1.iter().map(|s| s.to_string()).collect();
        let words2 = words2.iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::count_words(words1, words2), expected);
    }

    #[test]
    fn ex1() {
        test(
            &["leetcode", "is", "amazing", "as", "is"],
            &["amazing", "leetcode", "is"],
            2,
        )
    }

    #[test]
    fn ex2() {
        test(&["b", "bb", "bbb"], &["a", "aa", "aaa"], 0)
    }

    #[test]
    fn ex3() {
        test(&["a", "ab"], &["a", "a", "a", "ab"], 1)
    }
}
